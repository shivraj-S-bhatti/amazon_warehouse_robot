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
    BLIND_APPROACH_THRESHOLD, APPROACH_MIN_SPEED,
    RECENTER_YAW_THRESHOLD, RECENTER_YAW_SPEED,
    RECENTER_STRAFE_THRESHOLD, RECENTER_STRAFE_SPEED, RECENTER_SETTLED_COUNT,
    # Obstacle
    STRAFE_DIRECTION, STRAFE_DURATION,
    # Speeds
    SEARCH_ROTATE_SPEED,
    # Timing
    CONTROL_LOOP_RATE,
    # People bypass / avoidance
    BYPASS_FORWARD_SPEED, BYPASS_TURN_SPEED, BYPASS_CURVE_TIME, BYPASS_STRAIGHT_TIME,
    AVOID_TURN_SPEED,
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
        self.avoid_start_time = 0.0
        self.avoid_nudge_count = 0
        self.pre_avoid_state = STATE_NAVIGATE
        self.waypoint_search_start = None
        self.bypass_start_time = 0.0
        self.bypass_direction = 1.0       # +1 = bypass left, -1 = bypass right
        self.accumulated_yaw = 0.0        # yaw accrued during moving-person avoidance
        self._people_loop_time = time.time()
        self.running = True

        # Recentering sub-state ('yaw' → 'strafe' → settled)
        self._recenter_phase   = 'yaw'
        self._recenter_settled = 0

        # ── Logging ──
        self.logger = self.chassis.get_logger()
    
    def _spin_executor(self):
        """Spin the executor in a background thread."""
        try:
            self.executor.spin()
        except Exception:
            pass

    # =================================================================
    # Recentering helpers
    # =================================================================

    def _reset_recenter(self):
        """Reset recentering sub-state. Call whenever the marker is lost."""
        self._recenter_phase   = 'yaw'
        self._recenter_settled = 0

    def _apply_recenter(self, detection) -> bool:
        """
        Two-phase alignment before driving toward a marker.
        Phase 1: Yaw to face marker.
        Phase 2: Proportional strafe to slide into center.
        """
        err = detection['horizontal_error']

        # ── Phase 1: YAW ────────────────────────────────────────────────────
        if self._recenter_phase == 'yaw':
            if abs(err) > RECENTER_YAW_THRESHOLD:
                angular = -RECENTER_YAW_SPEED if err > 0 else RECENTER_YAW_SPEED
                self.chassis.move(linear_x=0.0, angular_z=angular)
                self.logger.info(
                    f'Recenter YAW:   h_err={err:+.5f}  '
                    f'rotating {"right" if err > 0 else "left"} @ {RECENTER_YAW_SPEED} rad/s'
                )
                return False
            
            # Yaw threshold met — advance to strafe phase
            self.chassis.stop()
            self._recenter_phase = 'strafe'
            self.logger.info(f'Recenter YAW done (h_err={err:+.5f}) → entering STRAFE phase')
            
            # Yield so the state machine can run avoidance checks before strafing starts
            return False 

        # ── Phase 2: STRAFE (Proportional Smoothing) ─────────────────────────
        if self._recenter_phase == 'strafe':
            if abs(err) > RECENTER_STRAFE_THRESHOLD:
                self._recenter_settled = 0
                
                # Scale speed based on error size for a smooth glide
                proportional_speed = RECENTER_STRAFE_SPEED * min(1.0, abs(err) * 3.0)
                strafe_speed = max(0.05, proportional_speed) 
                
                linear_y = -strafe_speed if err > 0 else strafe_speed

                self.chassis.move(linear_x=0.0, linear_y=linear_y, angular_z=0.0)
                self.logger.info(
                    f'Recenter STRAFE: h_err={err:+.5f}  '
                    f'strafing {"right" if err > 0 else "left"} @ {strafe_speed:.3f} m/s'
                )
                return False
                
            # Strafe threshold met — start counting settled frames
            self._recenter_settled += 1
            self.chassis.stop()
            self.logger.info(
                f'Recenter SETTLED: h_err={err:+.5f}  '
                f'frame {self._recenter_settled}/{RECENTER_SETTLED_COUNT}'
            )
            if self._recenter_settled < RECENTER_SETTLED_COUNT:
                return False

        # ── Done — reset for next time ───────────────────────────────────────
        self._reset_recenter()
        return True

    # =================================================================
    # State handlers
    # =================================================================

    def handle_search_object(self):
        self.chassis.set_led_searching()
        detection = self.vision.get_marker(OBJECT_MARKER_ID)

        if detection is not None:
            self.logger.info(
                f'Object marker {OBJECT_MARKER_ID} found at '
                f'{detection["distance"]:.2f}m')
            
            self.chassis.stop()
            self.logger.info('Marker found! Pausing for 1 second...')
            time.sleep(1.0)
            
            self.marker_last_seen_time = time.time()
            self.last_dist = detection['distance']
            return STATE_APPROACH_OBJECT

        self.chassis.rotate(SEARCH_ROTATE_SPEED)
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

            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                self.logger.warn('Object marker lost — returning to search')
                self.chassis.stop()
                return STATE_SEARCH_OBJECT
            return STATE_APPROACH_OBJECT

        self.last_dist = detection['distance']
        self.marker_last_seen_time = time.time()

        if detection['distance'] < APPROACH_DISTANCE:
            self.logger.info('Close enough to object — stopping to pick up')
            self.chassis.stop()
            time.sleep(0.5)
            return STATE_PICK_UP

        # ── 1. INITIAL YAW CENTERING ──
        if self._recenter_phase == 'yaw':
            self._apply_recenter(detection)
            return STATE_APPROACH_OBJECT

        # ── 2. PEOPLE AVOIDANCE (Active during Strafe & Forward Approach) ──
        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()

        if person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
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

        if person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            self.logger.warn('Stationary person blocking path — initiating S-curve bypass')
            self.chassis.stop()
            self.bypass_start_time = time.time()
            person_cx = self.people.get_person_cx()
            self.bypass_direction = 1.0 if person_cx > 0.5 else -1.0
            self.pre_avoid_state = STATE_APPROACH_OBJECT
            return STATE_BYPASS_OBSTACLE

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=correction)
            return STATE_APPROACH_OBJECT

        self.accumulated_yaw = 0.0

        # ── 3. TOF OBSTACLE CHECK ──
        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof < detection['distance'] - 0.10:
                self.logger.warn('Obstacle on approach to object!')
                self.chassis.stop()
                self.avoid_start_time = time.time()
                self.avoid_nudge_count = 0
                self.pre_avoid_state = STATE_APPROACH_OBJECT
                return STATE_OBSTACLE_AVOID

        # ── 4. STRAFE RECENTER & FORWARD APPROACH ──
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
            self.logger.info('Pick-up successful! Starting navigation.')
            self.current_waypoint_index = 0
            self.marker_last_seen_time = time.time()
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

        # Early exit if destination marker becomes visible
        dest_detection = self.vision.get_marker(DEST_MARKER_ID)
        if dest_detection is not None:
            self.logger.info('Destination marker visible — skipping remaining waypoints')
            self.chassis.stop()
            self.marker_last_seen_time = time.time()
            return STATE_APPROACH_DEST

        # ── PEOPLE / OBSTACLE AVOIDANCE (always active during navigation) ──
        # Run these checks BEFORE the marker check so the robot reacts to
        # people/obstacles even if the waypoint is momentarily out of frame
        # (e.g. occluded by the person who just stepped in front of it).
        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()

        if person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
            vel       = self.people.get_smoothed_velocity()
            person_cx = self.people.get_person_cx()
            if abs(vel) > 0.06:
                angular_z = AVOID_TURN_SPEED if vel > 0 else -AVOID_TURN_SPEED
            else:
                angular_z = AVOID_TURN_SPEED if person_cx > 0.5 else -AVOID_TURN_SPEED
            self.accumulated_yaw += angular_z * dt
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=angular_z)
            self.logger.info(
                f'Navigate: moving person (cx={person_cx:.2f}, vel={vel:+.3f}) '
                f'— turning {"left" if angular_z > 0 else "right"}'
            )
            return STATE_NAVIGATE

        if person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            self.logger.warn('Stationary person blocking path — initiating S-curve bypass')
            self.chassis.stop()
            self.bypass_start_time = time.time()
            person_cx = self.people.get_person_cx()
            self.bypass_direction = 1.0 if person_cx > 0.5 else -1.0
            self.pre_avoid_state = STATE_NAVIGATE
            return STATE_BYPASS_OBSTACLE

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=correction)
            return STATE_NAVIGATE

        self.accumulated_yaw = 0.0

        # TOF obstacle check is also runtime-independent of marker visibility.
        if self.obstacle.is_blocked():
            self.logger.warn(f'Obstacle during navigation to waypoint {current_marker_id}!')
            self.chassis.stop()
            self.avoid_start_time = time.time()
            self.avoid_nudge_count = 0
            self.pre_avoid_state = STATE_NAVIGATE
            return STATE_OBSTACLE_AVOID

        # ── MARKER-BASED NAVIGATION ──
        detection = self.vision.get_marker(current_marker_id)

        if detection is None:
            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                if self.waypoint_search_start is None:
                    self.waypoint_search_start = time.time()
                    self.logger.info(f'Waypoint {current_marker_id} lost — rotating to search')
                elif time.time() - self.waypoint_search_start > WAYPOINT_SEARCH_TIMEOUT:
                    self.logger.warn(f'Waypoint {current_marker_id} not found after {WAYPOINT_SEARCH_TIMEOUT}s — skipping')
                    self.current_waypoint_index += 1
                    self.waypoint_search_start = None
                    self.marker_last_seen_time = time.time()
                    return STATE_NAVIGATE
                self.chassis.rotate(SEARCH_ROTATE_SPEED)
            else:
                # Within the brief grace period: pause forward motion instead
                # of coasting blindly with the previous velocity command.
                self.chassis.stop()
            return STATE_NAVIGATE

        self.marker_last_seen_time = time.time()
        self.waypoint_search_start = None

        if detection['distance'] < WAYPOINT_SWITCH_DISTANCE:
            self.logger.info(f'Waypoint {current_marker_id} reached')
            self.current_waypoint_index += 1
            self.marker_last_seen_time = time.time()
            return STATE_NAVIGATE

        self.chassis.navigate_toward_marker(detection)
        return STATE_NAVIGATE

    def handle_obstacle_avoid(self):
        self.chassis.set_led_avoiding()

        if self.obstacle.is_clear():
            self.logger.info(f'Path clear — returning to {self.pre_avoid_state}')
            self.chassis.stop()
            time.sleep(0.3)
            self.marker_last_seen_time = time.time()
            return self.pre_avoid_state

        elapsed = time.time() - self.avoid_start_time
        if elapsed > STRAFE_DURATION * 3:
            if self.avoid_nudge_count >= 3:
                self.logger.warn('Obstacle not clearing after 3 nudges — resuming')
                self.avoid_nudge_count = 0
                self.chassis.stop()
                self.marker_last_seen_time = time.time()
                return self.pre_avoid_state
            self.chassis.move_forward(0.1)
            time.sleep(0.5)
            self.chassis.stop()
            self.avoid_nudge_count += 1
            self.avoid_start_time = time.time()
            return STATE_OBSTACLE_AVOID

        if STRAFE_DIRECTION > 0:
            self.chassis.strafe_left()
        else:
            self.chassis.strafe_right()

        return STATE_OBSTACLE_AVOID

    def handle_bypass_obstacle(self):
        self.chassis.set_led_avoiding()

        # During the bypass arc, if the ToF sees something dead-ahead
        # (e.g. another obstacle along the curve), abort the bypass and
        # fall through to in-place obstacle avoidance for safety.
        if self.obstacle.is_blocked():
            self.logger.warn('Obstacle detected mid-bypass — aborting S-curve')
            self.chassis.stop()
            self.avoid_start_time = time.time()
            self.avoid_nudge_count = 0
            # Keep pre_avoid_state pointed at the original travelling state so
            # the robot resumes the correct task after avoidance clears.
            return STATE_OBSTACLE_AVOID

        elapsed = time.time() - self.bypass_start_time
        p1_end  = BYPASS_CURVE_TIME
        p2_end  = p1_end + BYPASS_STRAIGHT_TIME
        p3_end  = p2_end + BYPASS_CURVE_TIME

        if elapsed < p1_end:
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=self.bypass_direction * BYPASS_TURN_SPEED)
        elif elapsed < p2_end:
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED)
        elif elapsed < p3_end:
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=-self.bypass_direction * BYPASS_TURN_SPEED)
        else:
            self.chassis.stop()
            self.people.notify_bypass_complete()
            self.marker_last_seen_time = time.time()
            # Reset recentering so the resumed approach state re-aligns to the
            # marker from scratch after the lateral offset introduced by the
            # S-curve.
            self._reset_recenter()
            resume_state = self.pre_avoid_state or STATE_NAVIGATE
            self.logger.info(f'Bypass complete — resuming {resume_state}')
            return resume_state

        return STATE_BYPASS_OBSTACLE

    def handle_search_dest(self):
        self.chassis.set_led_searching()
        detection = self.vision.get_marker(DEST_MARKER_ID)

        if detection is not None:
            self.chassis.stop()
            time.sleep(1.0)
            self.marker_last_seen_time = time.time()
            self.last_dest_dist = detection['distance']
            return STATE_APPROACH_DEST

        self.chassis.rotate(SEARCH_ROTATE_SPEED)
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

            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                self.chassis.stop()
                return STATE_SEARCH_DEST
            return STATE_APPROACH_DEST

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

        # ── 1. INITIAL YAW CENTERING ──
        if self._recenter_phase == 'yaw':
            self._apply_recenter(detection)
            return STATE_APPROACH_DEST

        # ── 2. PEOPLE AVOIDANCE (Active during Strafe & Forward Approach) ──
        now = time.time()
        dt = now - self._people_loop_time
        self._people_loop_time = now

        person_state = self.people.get_state()
        person_area  = self.people.get_person_area()

        if person_state == PERSON_MOVING and person_area >= PERSON_BLOCK_AREA:
            vel       = self.people.get_smoothed_velocity()
            person_cx = self.people.get_person_cx()
            if abs(vel) > 0.06:
                angular_z = AVOID_TURN_SPEED if vel > 0 else -AVOID_TURN_SPEED
            else:
                angular_z = AVOID_TURN_SPEED if person_cx > 0.5 else -AVOID_TURN_SPEED
            self.accumulated_yaw += angular_z * dt
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=angular_z)
            return STATE_APPROACH_DEST

        if person_state == PERSON_STATIONARY and person_area >= PERSON_BLOCK_AREA:
            self.logger.warn('Stationary person blocking path to destination — initiating S-curve bypass')
            self.chassis.stop()
            self.bypass_start_time = time.time()
            person_cx = self.people.get_person_cx()
            self.bypass_direction = 1.0 if person_cx > 0.5 else -1.0
            self.pre_avoid_state = STATE_APPROACH_DEST
            return STATE_BYPASS_OBSTACLE

        if abs(self.accumulated_yaw) > 0.02:
            correction = -BYPASS_TURN_SPEED if self.accumulated_yaw > 0 else BYPASS_TURN_SPEED
            yaw_step   = correction * dt
            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                correction = 0.0
            else:
                self.accumulated_yaw += yaw_step
            self.chassis.move(linear_x=BYPASS_FORWARD_SPEED, angular_z=correction)
            return STATE_APPROACH_DEST

        self.accumulated_yaw = 0.0

        # ── 3. TOF OBSTACLE CHECK ──
        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof >= detection['distance'] - 0.10:
                pass  # Ignore destination marker
            else:
                self.chassis.stop()
                self.avoid_start_time = time.time()
                self.avoid_nudge_count = 0
                self.pre_avoid_state = STATE_APPROACH_DEST
                return STATE_OBSTACLE_AVOID

        # ── 4. STRAFE RECENTER & FORWARD APPROACH ──
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

        self.logger.info('Waiting for camera frames...')
        for _ in range(50):
            if self.vision.get_frame_count() > 0:
                break
            time.sleep(0.1)

        self.logger.info('Waiting for arm/gripper servers...')
        if not self.arm.wait_for_servers(timeout_sec=10.0):
            return

        self.arm.retract()
        self.arm.recenter_gimbal()
        self.marker_last_seen_time = time.time()
        prev_state = None

        while self.running:
            loop_start = time.time()

            if self.state != prev_state:
                self.logger.info(f'>>> STATE: {self.state}')
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
        self.logger.info('Shutting down...')
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
        print('\nCtrl+C — stopping robot...')
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
        print(f'Error: {e}')
    finally:
        sm.shutdown()
        rclpy.shutdown()

if __name__ == '__main__':
    main()