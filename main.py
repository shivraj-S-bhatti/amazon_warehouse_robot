#!/usr/bin/env python3
"""
main.py — Main state machine for the pick-and-transport project.

Coordinates vision, obstacle avoidance, chassis control, and arm control
to pick up an object at Point A, navigate through waypoints while avoiding
obstacles, and place the object at Point B.

Usage:
    # Terminal 1: Launch the driver
    source ~/robomaster_ws/install/setup.bash
    ros2 launch robomaster_ros main.launch model:=ep conn_type:=ap \
      --ros-args --params-file ~/robomaster_ws/config.yaml

    # Terminal 2: Run this script
    source ~/robomaster_ws/install/setup.bash
    cd ~/robomaster_ws/my_project
    python3 main.py
"""
import sys
import rclpy
from rclpy.executors import MultiThreadedExecutor
import threading
import time
import signal

from config import (
    # States
    STATE_SEARCH_OBJECT, STATE_APPROACH_OBJECT, STATE_PICK_UP,
    STATE_NAVIGATE, STATE_OBSTACLE_AVOID, STATE_BYPASS_OBSTACLE,
    STATE_SEARCH_DEST, STATE_APPROACH_DEST, STATE_PLACE, STATE_DONE,
    # Marker IDs
    OBJECT_MARKER_ID, WAYPOINT_MARKER_IDS, DEST_MARKER_ID,
    # Thresholds
    APPROACH_DISTANCE, WAYPOINT_SWITCH_DISTANCE, DEST_APPROACH_DISTANCE,
    MARKER_LOST_TIMEOUT, WAYPOINT_SEARCH_TIMEOUT, PERSON_BLOCK_AREA,
    PERSON_CENTER_BLOCK_HALF_WIDTH, REACQUIRE_TURN_TIME,
    BLIND_APPROACH_THRESHOLD, APPROACH_MIN_SPEED, MARKER_BLOCKED_THRESHOLD,
    RECENTER_YAW_THRESHOLD, RECENTER_YAW_SPEED,
    RECENTER_STRAFE_THRESHOLD, RECENTER_STRAFE_SPEED, RECENTER_SETTLED_COUNT,
    # Speeds
    SEARCH_ROTATE_SPEED,
    # Timing
    CONTROL_LOOP_RATE,
    # Moving-person avoidance (in-place steering)
    BYPASS_FORWARD_SPEED, BYPASS_TURN_SPEED,
    AVOID_TURN_SPEED,
    # U-shape detour around stationary obstacles
    DETOUR_STRAFE_SPEED, DETOUR_STRAFE_DURATION,
    DETOUR_FORWARD_SPEED, DETOUR_MIN_FORWARD_DURATION, DETOUR_FORWARD_DURATION,
    DETOUR_DEFAULT_DIR,
)

from vision import VisionNode
from obstacle import ObstacleNode
from chassis_control import ChassisController
from arm_control import ArmController
from people_avoidance import PeopleAvoidanceNode, PERSON_MOVING, PERSON_STATIONARY


class StateMachine:
    """
    Main state machine that orchestrates the pick-and-transport task.
    """

    def __init__(self):
        # ── Create all nodes ──
        self.vision = VisionNode()
        self.obstacle = ObstacleNode()
        self.chassis = ChassisController()
        self.arm = ArmController()
        self.people = PeopleAvoidanceNode()

        # ── Background executor for nodes needing callback processing ──
        self.executor = MultiThreadedExecutor(num_threads=4)
        self.executor.add_node(self.vision)
        self.executor.add_node(self.obstacle)
        self.executor.add_node(self.chassis)
        self.executor.add_node(self.people)

        # Start background spinning
        self.spin_thread = threading.Thread(
            target=self._spin_executor, daemon=True)
        self.spin_thread.start()

        # ── State machine variables ──
        self.state = STATE_SEARCH_OBJECT
        self.current_waypoint_index = 0
        self.marker_last_seen_time = time.time()
        self.pre_avoid_state = STATE_NAVIGATE
        self.waypoint_search_start = None
        self.accumulated_yaw = 0.0        
        self._people_loop_time = time.time()
        self.running = True

        self.detour_direction = DETOUR_DEFAULT_DIR
        self.detour_start_time = 0.0
        self._last_marker_hint = {}
        self._search_sweep_direction = 1.0
        self._search_sweep_last_flip = time.time()

        # Start with strafe so the robot slides back to the initial line
        self._recenter_phase   = 'strafe'
        self._recenter_settled = 0

        self._post_detour_time = 0.0
        self.logger = self.chassis.get_logger()
    
    def _spin_executor(self):
        try:
            self.executor.spin()
        except Exception:
            pass

    def _remember_marker(self, marker_id: int, detection):
        self._last_marker_hint[marker_id] = {
            'time': time.time(),
            'horizontal_error': detection.get('horizontal_error', 0.0),
            'distance': detection.get('distance', float('inf')),
        }

    def _search_rotation_for_marker(self, marker_id: int) -> float:
        now = time.time()
        hint = self._last_marker_hint.get(marker_id)
        if hint is not None and now - hint['time'] <= MARKER_LOST_TIMEOUT + REACQUIRE_TURN_TIME:
            h_err = hint.get('horizontal_error', 0.0)
            if abs(h_err) > 0.03:
                return -SEARCH_ROTATE_SPEED if h_err > 0.0 else SEARCH_ROTATE_SPEED

        if now - self._search_sweep_last_flip > REACQUIRE_TURN_TIME:
            self._search_sweep_direction *= -1.0
            self._search_sweep_last_flip = now
        return self._search_sweep_direction * SEARCH_ROTATE_SPEED

    def _person_blocks_path(self) -> bool:
        if not self.people.is_person_recently_detected(max_age=0.6):
            return False

        area = self.people.get_person_area()
        if area < PERSON_BLOCK_AREA:
            return False

        cx = self.people.get_person_cx()
        return abs(cx - 0.5) <= PERSON_CENTER_BLOCK_HALF_WIDTH

    def _try_obstacle_detour(self, resume_state: str, elapsed: float) -> str | None:
        if time.time() - self._post_detour_time < 6.0:
            return None

        tof_dist = self.obstacle.get_distance()
        tof_blocks = self.obstacle.is_sensor_active() and tof_dist < MARKER_BLOCKED_THRESHOLD
        person_blocks = self._person_blocks_path()

        if (tof_blocks or person_blocks) and elapsed >= 1.0:
            if person_blocks:
                pcx = self.people.get_person_cx()
                self.detour_direction = 1.0 if pcx > 0.5 else -1.0
            else:
                self.detour_direction = DETOUR_DEFAULT_DIR
            self.logger.warn(
                f'Marker lost with obstacle suspected '
                f'(tof={tof_dist:.2f}m, tof_blocks={tof_blocks}, person_blocks={person_blocks}) — '
                f'U-detour {"LEFT" if self.detour_direction > 0 else "RIGHT"}'
            )
        else:
            return None

        self.chassis.stop()
        self.detour_start_time = time.time()
        self.pre_avoid_state = resume_state
        return STATE_OBSTACLE_AVOID

    def _reset_recenter(self):
        # Always default back to strafe first to ensure lateral return
        self._recenter_phase   = 'strafe'
        self._recenter_settled = 0

    def _apply_recenter(self, detection) -> bool:
        err = detection['horizontal_error']

        # ── Phase 1: STRAFE (Return to initial line) ─────────────────────────
        # By strafing first, the robot slides laterally back onto the optical axis before driving
        if self._recenter_phase == 'strafe':
            if abs(err) > RECENTER_STRAFE_THRESHOLD:
                self._recenter_settled = 0
                proportional_speed = RECENTER_STRAFE_SPEED * min(1.0, abs(err) * 3.0)
                strafe_speed = max(0.05, proportional_speed) 
                linear_y = -strafe_speed if err > 0 else strafe_speed

                self.chassis.move(linear_x=0.0, linear_y=linear_y, angular_z=0.0)
                self.logger.info(
                    f'Recenter STRAFE: h_err={err:+.5f}  '
                    f'strafing {"right" if err > 0 else "left"} @ {strafe_speed:.3f} m/s'
                )
                return False
                
            self.chassis.stop()
            self._recenter_phase = 'yaw'
            self.logger.info(f'Recenter STRAFE done (h_err={err:+.5f}) → entering YAW phase')
            return False 

        # ── Phase 2: YAW (Fix heading) ───────────────────────────────────────
        if self._recenter_phase == 'yaw':
            if abs(err) > RECENTER_YAW_THRESHOLD:
                angular = -RECENTER_YAW_SPEED if err > 0 else RECENTER_YAW_SPEED
                self.chassis.move(linear_x=0.0, linear_y=0.0, angular_z=angular)
                self.logger.info(
                    f'Recenter YAW:   h_err={err:+.5f}  '
                    f'rotating {"right" if err > 0 else "left"} @ {RECENTER_YAW_SPEED} rad/s'
                )
                return False
            
            self._recenter_settled += 1
            self.chassis.stop()
            self.logger.info(
                f'Recenter SETTLED: h_err={err:+.5f}  '
                f'frame {self._recenter_settled}/{RECENTER_SETTLED_COUNT}'
            )
            if self._recenter_settled < RECENTER_SETTLED_COUNT:
                return False

        self._reset_recenter()
        return True

    def handle_search_object(self):
        self.chassis.set_led_searching()
        detection = self.vision.get_marker(OBJECT_MARKER_ID)

        if detection is not None:
            self._remember_marker(OBJECT_MARKER_ID, detection)
            self.chassis.stop()
            time.sleep(1.0)
            self.marker_last_seen_time = time.time()
            self.last_dist = detection['distance']
            return STATE_APPROACH_OBJECT

        self.chassis.rotate(self._search_rotation_for_marker(OBJECT_MARKER_ID))
        return STATE_SEARCH_OBJECT

    def handle_approach_object(self):
        self.chassis.set_led_approaching()
        detection = self.vision.get_marker(OBJECT_MARKER_ID)

        if detection is None:
            self._reset_recenter()
            if hasattr(self, 'last_dist') and self.last_dist < BLIND_APPROACH_THRESHOLD:
                self.logger.info(f'Marker in blind spot (last seen {self.last_dist:.2f}m). Finishing blindly.')
                remaining_dist = self.last_dist - APPROACH_DISTANCE
                if remaining_dist > 0:
                    drive_time = remaining_dist / APPROACH_MIN_SPEED
                    self.chassis.move_forward(APPROACH_MIN_SPEED)
                    time.sleep(drive_time)
                self.chassis.stop()
                time.sleep(0.5)
                return STATE_PICK_UP

            elapsed = time.time() - self.marker_last_seen_time

            # Safely check for obstacles only during the grace period
            if elapsed <= MARKER_LOST_TIMEOUT:
                if elapsed > 1.0:
                    next_state = self._try_obstacle_detour(STATE_APPROACH_OBJECT, elapsed)
                    if next_state is not None:
                        return next_state
                self.chassis.stop()
                return STATE_APPROACH_OBJECT

            self.logger.warn('Object marker lost — returning to search')
            self.chassis.stop()
            return STATE_SEARCH_OBJECT

        self._remember_marker(OBJECT_MARKER_ID, detection)
        self.last_dist = detection['distance']
        self.marker_last_seen_time = time.time()

        if detection['distance'] < APPROACH_DISTANCE:
            self.chassis.stop()
            time.sleep(0.5)
            return STATE_PICK_UP

        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()
        person_blocks = self._person_blocks_path()

        if person_blocks and person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
            vel       = self.people.get_smoothed_velocity()
            person_cx = self.people.get_person_cx()
            if abs(vel) > 0.06:
                angular_z = AVOID_TURN_SPEED if vel > 0 else -AVOID_TURN_SPEED
            else:
                angular_z = AVOID_TURN_SPEED if person_cx > 0.5 else -AVOID_TURN_SPEED
            self.accumulated_yaw += angular_z * dt
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=angular_z)
            self.logger.info(f'Moving person (cx={person_cx:.2f}, vel={vel:+.3f}) — turning {"left" if angular_z > 0 else "right"}')
            return STATE_APPROACH_OBJECT

        if person_blocks and person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            person_cx = self.people.get_person_cx()
            self.detour_direction = 1.0 if person_cx > 0.5 else -1.0
            self.chassis.stop()
            self.detour_start_time = time.time()
            self.pre_avoid_state = STATE_APPROACH_OBJECT
            return STATE_OBSTACLE_AVOID

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=0.0, angular_z=correction)
            return STATE_APPROACH_OBJECT

        self.accumulated_yaw = 0.0

        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof < detection['distance'] - 0.10:
                h_err = detection.get('horizontal_error', 0.0)
                if abs(h_err) > 0.05:
                    self.detour_direction = -1.0 if h_err < 0 else 1.0
                else:
                    self.detour_direction = DETOUR_DEFAULT_DIR
                self.chassis.stop()
                self.detour_start_time = time.time()
                self.pre_avoid_state = STATE_APPROACH_OBJECT
                return STATE_OBSTACLE_AVOID

        if not self._apply_recenter(detection):
            return STATE_APPROACH_OBJECT
        
        self.chassis.drive_toward_marker(detection)
        return STATE_APPROACH_OBJECT

    def handle_pick_up(self):
        self.chassis.set_led_picking()
        self.chassis.stop()

        self.logger.info('Starting pick-up sequence...')
        success = self.arm.pick_up()

        if success:
            self.logger.info('Pick-up successful! Backing away from pickup zone.')
            # FIX: Back away from the pickup zone so we don't hit it when spinning
            self.chassis.move_backward(0.15)
            time.sleep(2.0)
            self.chassis.stop()

            self.logger.info('Starting navigation.')
            self.current_waypoint_index = 0
            # FIX: Force immediate spin/search in handle_navigate
            self.marker_last_seen_time = 0.0
            return STATE_NAVIGATE
        else:
            self.logger.error('Pick-up failed! Retrying search...')
            self.arm.retract()
            return STATE_SEARCH_OBJECT

    def handle_navigate(self):
        self.chassis.set_led_navigating()

        if self.current_waypoint_index >= len(WAYPOINT_MARKER_IDS):
            self.logger.info('All waypoints passed — searching for destination')
            return STATE_SEARCH_DEST

        current_marker_id = WAYPOINT_MARKER_IDS[self.current_waypoint_index]

        dest_detection = self.vision.get_marker(DEST_MARKER_ID)
        if dest_detection is not None:
            self.logger.info('Destination marker visible — skipping remaining waypoints')
            self._remember_marker(DEST_MARKER_ID, dest_detection)
            self.chassis.stop()
            self.marker_last_seen_time = time.time()
            return STATE_APPROACH_DEST

        detection = self.vision.get_marker(current_marker_id)

        if detection is None:
            self._reset_recenter()
            elapsed = time.time() - self.marker_last_seen_time

            # A. Grace period / Obstacle check (0s to 3s)
            if elapsed <= MARKER_LOST_TIMEOUT:
                if elapsed > 1.0:
                    next_state = self._try_obstacle_detour(STATE_NAVIGATE, elapsed)
                    if next_state is not None:
                        return next_state
                self.chassis.stop()
                return STATE_NAVIGATE

            # B. Searching (> 3s, or forced by 0.0)
            # FIX: Robot will not trigger ToF detours while it is just spinning
            if self.waypoint_search_start is None:
                self.waypoint_search_start = time.time()
                self.logger.info(f'Waypoint {current_marker_id} lost — rotating to search')
            elif time.time() - self.waypoint_search_start > WAYPOINT_SEARCH_TIMEOUT:
                self.logger.warn(f'Waypoint {current_marker_id} not found after timeout — skipping')
                self.current_waypoint_index += 1
                self.waypoint_search_start = None
                self.marker_last_seen_time = 0.0 # Force instant search for next
                return STATE_NAVIGATE

            self.chassis.rotate(self._search_rotation_for_marker(current_marker_id))
            return STATE_NAVIGATE

        # ── If we reach here, marker IS visible ──
        self._remember_marker(current_marker_id, detection)
        self.marker_last_seen_time = time.time()
        self.waypoint_search_start = None

        if detection['distance'] < WAYPOINT_SWITCH_DISTANCE:
            self.logger.info(f'Waypoint {current_marker_id} reached')
            self.current_waypoint_index += 1
            self.marker_last_seen_time = 0.0 # Force instant search for next
            return STATE_NAVIGATE

        # ── 1. PEOPLE AVOIDANCE ──
        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()
        person_blocks = self._person_blocks_path()

        if person_blocks and person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
            vel       = self.people.get_smoothed_velocity()
            person_cx = self.people.get_person_cx()
            if abs(vel) > 0.06:
                angular_z = AVOID_TURN_SPEED if vel > 0 else -AVOID_TURN_SPEED
            else:
                angular_z = AVOID_TURN_SPEED if person_cx > 0.5 else -AVOID_TURN_SPEED
            self.accumulated_yaw += angular_z * dt
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=angular_z)
            return STATE_NAVIGATE

        if person_blocks and person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            person_cx = self.people.get_person_cx()
            self.detour_direction = 1.0 if person_cx > 0.5 else -1.0
            self.chassis.stop()
            self.detour_start_time = time.time()
            self.pre_avoid_state = STATE_NAVIGATE
            return STATE_OBSTACLE_AVOID

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=0.0, angular_z=correction)
            return STATE_NAVIGATE

        self.accumulated_yaw = 0.0

        # ── 2. TOF OBSTACLE CHECK ──
        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof < detection['distance'] - 0.10:
                h_err = detection.get('horizontal_error', 0.0)
                if abs(h_err) > 0.05:
                    self.detour_direction = -1.0 if h_err < 0 else 1.0
                else:
                    self.detour_direction = DETOUR_DEFAULT_DIR
                self.chassis.stop()
                self.detour_start_time = time.time()
                self.pre_avoid_state = STATE_NAVIGATE
                return STATE_OBSTACLE_AVOID

        # ── 3. STRAFE RECENTER then YAW & FORWARD APPROACH ──
        if not self._apply_recenter(detection):
            return STATE_NAVIGATE
        
        self.chassis.navigate_toward_marker(detection)
        return STATE_NAVIGATE

    def _active_detour_marker_id(self):
        if self.pre_avoid_state == STATE_NAVIGATE:
            idx = self.current_waypoint_index
            if idx < len(WAYPOINT_MARKER_IDS):
                return WAYPOINT_MARKER_IDS[idx]
        elif self.pre_avoid_state == STATE_APPROACH_OBJECT:
            return OBJECT_MARKER_ID
        elif self.pre_avoid_state == STATE_APPROACH_DEST:
            return DEST_MARKER_ID
        return None

    def _obstacle_cleared_for_strafe_back(self) -> bool:
        mid = self._active_detour_marker_id()
        if mid is not None and self.vision.get_marker(mid) is not None:
            return True

        if self.people.is_person_recently_detected():
            pcx = self.people.get_person_cx()
            side = self.detour_direction
            if (side > 0 and pcx > 0.75) or (side < 0 and pcx < 0.25):
                return True

        return False

    def handle_obstacle_avoid(self):
        self.chassis.set_led_avoiding()

        side    = self.detour_direction
        elapsed = time.time() - self.detour_start_time
        p1_end  = DETOUR_STRAFE_DURATION
        p2_end  = p1_end + DETOUR_FORWARD_DURATION
        p3_end  = p2_end + DETOUR_STRAFE_DURATION

        if elapsed < p1_end:
            self.chassis.move(linear_y=side * DETOUR_STRAFE_SPEED)
            return STATE_OBSTACLE_AVOID

        if elapsed < p2_end:
            forward_elapsed = elapsed - p1_end
            if (
                forward_elapsed >= DETOUR_MIN_FORWARD_DURATION
                and self._obstacle_cleared_for_strafe_back()
            ):
                self.detour_start_time = time.time() - (p2_end + 0.01)
                return STATE_OBSTACLE_AVOID

            if self.obstacle.is_blocked():
                self.logger.warn('Second obstacle mid-detour — extending the U')
                self.detour_start_time = time.time()
                return STATE_OBSTACLE_AVOID

            self.chassis.move(linear_x=DETOUR_FORWARD_SPEED)
            return STATE_OBSTACLE_AVOID

        if elapsed < p3_end:
            self.chassis.move(linear_y=-side * DETOUR_STRAFE_SPEED)
            return STATE_OBSTACLE_AVOID

        self.chassis.stop()
        self.people.notify_bypass_complete()
        self.marker_last_seen_time = time.time()
        self._post_detour_time = time.time()
        self.waypoint_search_start = None
        self._reset_recenter()
        resume_state = self.pre_avoid_state or STATE_NAVIGATE
        return resume_state

    def handle_bypass_obstacle(self):
        return self.handle_obstacle_avoid()

    def handle_search_dest(self):
        self.chassis.set_led_searching()
        detection = self.vision.get_marker(DEST_MARKER_ID)

        if detection is not None:
            self._remember_marker(DEST_MARKER_ID, detection)
            self.chassis.stop()
            time.sleep(1.0)
            self.marker_last_seen_time = time.time()
            self.last_dest_dist = detection['distance']
            return STATE_APPROACH_DEST

        self.chassis.rotate(self._search_rotation_for_marker(DEST_MARKER_ID))
        return STATE_SEARCH_DEST

    def handle_approach_dest(self):
        self.chassis.set_led_approaching()
        detection = self.vision.get_marker(DEST_MARKER_ID)

        if detection is None:
            self._reset_recenter()
            if hasattr(self, 'last_dest_dist') and self.last_dest_dist < BLIND_APPROACH_THRESHOLD:
                remaining_dist = self.last_dest_dist - DEST_APPROACH_DISTANCE
                if remaining_dist > 0:
                    drive_time = remaining_dist / APPROACH_MIN_SPEED
                    self.chassis.move_forward(APPROACH_MIN_SPEED)
                    time.sleep(drive_time)
                self.chassis.stop()
                time.sleep(0.5)
                return STATE_PLACE

            elapsed = time.time() - self.marker_last_seen_time

            if elapsed <= MARKER_LOST_TIMEOUT:
                if elapsed > 1.0:
                    next_state = self._try_obstacle_detour(STATE_APPROACH_DEST, elapsed)
                    if next_state is not None:
                        return next_state
                self.chassis.stop()
                return STATE_APPROACH_DEST

            self.chassis.stop()
            return STATE_SEARCH_DEST

        self._remember_marker(DEST_MARKER_ID, detection)
        self.last_dest_dist = detection['distance']
        self.marker_last_seen_time = time.time()

        if detection['distance'] < DEST_APPROACH_DISTANCE:
            self.chassis.stop()
            time.sleep(0.5)

            aligned = False
            for _ in range(20):
                det = self.vision.get_marker(DEST_MARKER_ID)
                if det is None:
                    break
                aligned = self.chassis.align_to_marker(det)
                if aligned:
                    break
                time.sleep(0.1)

            self.chassis.stop()
            return STATE_PLACE

        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()
        person_blocks = self._person_blocks_path()

        if person_blocks and person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
            vel       = self.people.get_smoothed_velocity()
            person_cx = self.people.get_person_cx()
            if abs(vel) > 0.06:
                angular_z = AVOID_TURN_SPEED if vel > 0 else -AVOID_TURN_SPEED
            else:
                angular_z = AVOID_TURN_SPEED if person_cx > 0.5 else -AVOID_TURN_SPEED
            self.accumulated_yaw += angular_z * dt
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=angular_z)
            return STATE_APPROACH_DEST

        if person_blocks and person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            person_cx = self.people.get_person_cx()
            self.detour_direction = 1.0 if person_cx > 0.5 else -1.0
            self.chassis.stop()
            self.detour_start_time = time.time()
            self.pre_avoid_state = STATE_APPROACH_DEST
            return STATE_OBSTACLE_AVOID

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=0.0, angular_z=correction)
            return STATE_APPROACH_DEST

        self.accumulated_yaw = 0.0

        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof < detection['distance'] - 0.10:
                h_err = detection.get('horizontal_error', 0.0)
                if abs(h_err) > 0.05:
                    self.detour_direction = -1.0 if h_err < 0 else 1.0
                else:
                    self.detour_direction = DETOUR_DEFAULT_DIR
                self.chassis.stop()
                self.detour_start_time = time.time()
                self.pre_avoid_state = STATE_APPROACH_DEST
                return STATE_OBSTACLE_AVOID

        if not self._apply_recenter(detection):
            return STATE_APPROACH_DEST
        
        self.chassis.drive_toward_marker(detection)
        return STATE_APPROACH_DEST

    def handle_place(self):
        self.chassis.set_led_picking()
        self.chassis.stop()

        success = self.arm.place_down()
        if success:
            self.chassis.move_backward(0.15)
            time.sleep(2.0)
            self.chassis.stop()
        return STATE_DONE

    def handle_done(self):
        self.chassis.stop()
        self.chassis.set_led_done()
        self.logger.info('========== TASK COMPLETE ==========')
        self.running = False
        return STATE_DONE

    def run(self):
        handlers = {
            STATE_SEARCH_OBJECT: self.handle_search_object,
            STATE_APPROACH_OBJECT: self.handle_approach_object,
            STATE_PICK_UP: self.handle_pick_up,
            STATE_NAVIGATE: self.handle_navigate,
            STATE_OBSTACLE_AVOID: self.handle_obstacle_avoid,
            STATE_BYPASS_OBSTACLE: self.handle_bypass_obstacle,
            STATE_SEARCH_DEST: self.handle_search_dest,
            STATE_APPROACH_DEST: self.handle_approach_dest,
            STATE_PLACE: self.handle_place,
            STATE_DONE: self.handle_done,
        }

        loop_period = 1.0 / CONTROL_LOOP_RATE

        for _ in range(50):
            if self.vision.get_frame_count() > 0:
                break
            time.sleep(0.1)

        if not self.arm.wait_for_servers(timeout_sec=10.0):
            return

        self.arm.retract()
        self.arm.recenter_gimbal()
        self.marker_last_seen_time = time.time()
        prev_state = None

        while self.running:
            loop_start = time.time()

            if self.state != prev_state:
                prev_state = self.state

            handler = handlers.get(self.state)
            if handler is None:
                break

            self.state = handler()

            elapsed = time.time() - loop_start
            sleep_time = loop_period - elapsed
            if sleep_time > 0:
                time.sleep(sleep_time)

    def shutdown(self):
        self.running = False
        self.chassis.stop()

        try:
            self.arm.retract()
        except Exception:
            pass

        self.executor.shutdown()
        self.vision.destroy_node()
        self.obstacle.destroy_node()
        self.chassis.destroy_node()
        self.people.destroy_node()
        self.arm.destroy_node()


def main():
    rclpy.init()
    sm = StateMachine()

    def signal_handler(sig, frame):
        sm.shutdown()
        rclpy.shutdown()
        sys.exit(0)

    signal.signal(signal.SIGINT, signal_handler)

    try:
        sm.run()
    except KeyboardInterrupt:
        pass
    except Exception as e:
        sm.chassis.stop()
    finally:
        sm.shutdown()
        rclpy.shutdown()

if __name__ == '__main__':
    main()
