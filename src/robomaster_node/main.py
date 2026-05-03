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

import rclpy
from rclpy.executors import MultiThreadedExecutor
import threading
import time
import signal
import sys

from config import (
    # States
    STATE_SEARCH_OBJECT, STATE_APPROACH_OBJECT, STATE_PICK_UP,
    STATE_NAVIGATE, STATE_OBSTACLE_AVOID,
    STATE_SEARCH_DEST, STATE_APPROACH_DEST, STATE_PLACE, STATE_DONE,
    # Marker IDs
    OBJECT_MARKER_ID, WAYPOINT_MARKER_IDS, DEST_MARKER_ID,
    # Thresholds
    APPROACH_DISTANCE, WAYPOINT_SWITCH_DISTANCE, DEST_APPROACH_DISTANCE,
    MARKER_LOST_TIMEOUT, WAYPOINT_SEARCH_TIMEOUT,
    # Obstacle
    STRAFE_DIRECTION, STRAFE_DURATION,
    # Speeds
    SEARCH_ROTATE_SPEED,
    # Timing
    CONTROL_LOOP_RATE,
)

from vision import VisionNode
from obstacle import ObstacleNode
from chassis_control import ChassisController
from arm_control import ArmController


class StateMachine:
    """
    Main state machine that orchestrates the pick-and-transport task.

    Architecture:
        - VisionNode and ObstacleNode run in a background executor thread
          (they need continuous callback processing for camera and ToF)
        - ChassisController publishes cmd_vel (added to executor for LEDs)
        - ArmController uses blocking action calls with its own spinning
          (NOT added to the shared executor)
        - State machine logic runs in the main thread
    """

    def __init__(self):
        # ── Create all nodes ──
        self.vision = VisionNode()
        self.obstacle = ObstacleNode()
        self.chassis = ChassisController()
        self.arm = ArmController()

        # ── Background executor for nodes needing callback processing ──
        self.executor = MultiThreadedExecutor(num_threads=4)
        self.executor.add_node(self.vision)
        self.executor.add_node(self.obstacle)
        self.executor.add_node(self.chassis)
        # NOTE: ArmController is NOT added here because it uses
        # rclpy.spin_until_future_complete() internally

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
        self.pre_avoid_state = STATE_NAVIGATE  # state to return to after avoidance
        self.waypoint_search_start = None  # set when we begin rotating for a waypoint
        self.running = True

        # ── Logging ──
        self.logger = self.chassis.get_logger()

    def _spin_executor(self):
        """Spin the executor in a background thread."""
        try:
            self.executor.spin()
        except Exception:
            pass

    # =================================================================
    # State handlers — each returns the next state
    # =================================================================

    def handle_search_object(self):
        """
        SEARCH_OBJECT: Rotate to find the object marker (ID=0).
        """
        self.chassis.set_led_searching()

        detection = self.vision.get_marker(OBJECT_MARKER_ID)

        if detection is not None:
            self.logger.info(
                f'Object marker {OBJECT_MARKER_ID} found at '
                f'{detection["distance"]:.2f}m')
            self.marker_last_seen_time = time.time()
            return STATE_APPROACH_OBJECT

        # Rotate slowly to search
        self.chassis.rotate(SEARCH_ROTATE_SPEED)
        return STATE_SEARCH_OBJECT

    def handle_approach_object(self):
        """
        APPROACH_OBJECT: Drive toward the object marker using visual servoing.
        Stop when close enough to pick up.
        """
        self.chassis.set_led_approaching()

        detection = self.vision.get_marker(OBJECT_MARKER_ID)

        if detection is None:
            # Marker lost — check timeout
            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                self.logger.warn('Object marker lost — returning to search')
                self.chassis.stop()
                return STATE_SEARCH_OBJECT
            # Keep last command briefly
            return STATE_APPROACH_OBJECT

        self.marker_last_seen_time = time.time()

        # Check if close enough to pick up
        if detection['distance'] < APPROACH_DISTANCE:
            self.logger.info('Close enough to object — stopping to pick up')
            self.chassis.stop()
            time.sleep(0.5)

            # Fine alignment: rotate to center the marker
            aligned = False
            for _ in range(20):
                det = self.vision.get_marker(OBJECT_MARKER_ID)
                if det is None:
                    break
                aligned = self.chassis.align_to_marker(det)
                if aligned:
                    break
                time.sleep(0.1)

            self.chassis.stop()
            if not aligned:
                self.logger.warn('Fine alignment incomplete — proceeding anyway')
            return STATE_PICK_UP

        # Check for obstacles — skip if the "obstacle" is the target itself
        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof >= detection['distance'] - 0.10:
                # ToF is reading the target object, not a real obstacle
                pass
            else:
                self.logger.warn('Obstacle detected while approaching object!')
                self.chassis.stop()
                self.avoid_start_time = time.time()
                self.avoid_nudge_count = 0
                self.pre_avoid_state = STATE_APPROACH_OBJECT
                return STATE_OBSTACLE_AVOID

        # Drive toward marker
        speed, angular = self.chassis.drive_toward_marker(detection)
        self.logger.info(
            f'Approaching object: dist={detection["distance"]:.2f}m, '
            f'lateral={detection["lateral"]:.3f}m')

        return STATE_APPROACH_OBJECT

    def handle_pick_up(self):
        """
        PICK_UP: Execute the pick-up sequence (arm + gripper).
        This state blocks while the arm moves.
        """
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
        """
        NAVIGATE: Follow waypoint markers toward the destination.
        Switches to the next waypoint when close enough.
        """
        self.chassis.set_led_navigating()

        # Check if all waypoints passed
        if self.current_waypoint_index >= len(WAYPOINT_MARKER_IDS):
            self.logger.info('All waypoints passed — searching for destination')
            return STATE_SEARCH_DEST

        current_marker_id = WAYPOINT_MARKER_IDS[self.current_waypoint_index]

        # Check if destination is already visible (shortcut)
        dest_detection = self.vision.get_marker(DEST_MARKER_ID)
        if dest_detection is not None:
            self.logger.info(
                f'Destination marker {DEST_MARKER_ID} visible at '
                f'{dest_detection["distance"]:.2f}m — heading there directly')
            return STATE_SEARCH_DEST

        # Look for current waypoint marker
        detection = self.vision.get_marker(current_marker_id)

        if detection is None:
            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                # Start tracking how long we've been searching for this waypoint
                if self.waypoint_search_start is None:
                    self.waypoint_search_start = time.time()

                # Give up on this waypoint after WAYPOINT_SEARCH_TIMEOUT seconds
                if time.time() - self.waypoint_search_start > WAYPOINT_SEARCH_TIMEOUT:
                    self.logger.warn(
                        f'Waypoint {current_marker_id} not found after '
                        f'{WAYPOINT_SEARCH_TIMEOUT:.0f}s — skipping to destination search')
                    self.waypoint_search_start = None
                    return STATE_SEARCH_DEST

                self.chassis.rotate(SEARCH_ROTATE_SPEED)
            return STATE_NAVIGATE

        self.marker_last_seen_time = time.time()
        self.waypoint_search_start = None  # marker found, reset search timer

        # Close enough to switch to next waypoint
        if detection['distance'] < WAYPOINT_SWITCH_DISTANCE:
            self.current_waypoint_index += 1
            if self.current_waypoint_index < len(WAYPOINT_MARKER_IDS):
                next_id = WAYPOINT_MARKER_IDS[self.current_waypoint_index]
                self.logger.info(
                    f'Waypoint {current_marker_id} reached — '
                    f'next: waypoint {next_id}')
            else:
                self.logger.info(
                    f'Waypoint {current_marker_id} reached — '
                    f'all waypoints done')
            self.marker_last_seen_time = time.time()
            return STATE_NAVIGATE

        # Check for obstacles
        if self.obstacle.is_blocked():
            self.logger.warn(
                f'Obstacle at {self.obstacle.get_distance():.2f}m!')
            self.chassis.stop()
            self.avoid_start_time = time.time()
            self.avoid_nudge_count = 0
            self.pre_avoid_state = STATE_NAVIGATE
            return STATE_OBSTACLE_AVOID

        # Navigate toward waypoint
        self.chassis.navigate_toward_marker(detection)
        self.logger.info(
            f'Navigating to waypoint {current_marker_id}: '
            f'dist={detection["distance"]:.2f}m')

        return STATE_NAVIGATE

    def handle_obstacle_avoid(self):
        """
        OBSTACLE_AVOID: Strafe sideways to clear an obstacle.
        Resume navigation when the path is clear.
        """
        self.chassis.set_led_avoiding()

        # Check if path is clear
        if self.obstacle.is_clear():
            self.logger.info(f'Path clear — returning to {self.pre_avoid_state}')
            self.chassis.stop()
            time.sleep(0.3)
            self.marker_last_seen_time = time.time()
            return self.pre_avoid_state

        # Timeout: strafed too long without clearing
        elapsed = time.time() - self.avoid_start_time
        if elapsed > STRAFE_DURATION * 3:
            if self.avoid_nudge_count >= 3:
                self.logger.warn('Obstacle not clearing after 3 nudges — resuming')
                self.avoid_nudge_count = 0
                self.chassis.stop()
                self.marker_last_seen_time = time.time()
                return self.pre_avoid_state
            self.logger.warn(
                f'Extended avoidance — trying forward nudge '
                f'({self.avoid_nudge_count + 1}/3)')
            self.chassis.move_forward(0.1)
            time.sleep(0.5)
            self.chassis.stop()
            self.avoid_nudge_count += 1
            self.avoid_start_time = time.time()
            return STATE_OBSTACLE_AVOID

        # Strafe sideways
        if STRAFE_DIRECTION > 0:
            self.chassis.strafe_left()
        else:
            self.chassis.strafe_right()

        self.logger.info(
            f'Avoiding: ToF={self.obstacle.get_distance():.2f}m, '
            f'strafing {"left" if STRAFE_DIRECTION > 0 else "right"}')

        return STATE_OBSTACLE_AVOID

    def handle_search_dest(self):
        """
        SEARCH_DEST: Rotate to find the destination marker.
        """
        self.chassis.set_led_searching()

        detection = self.vision.get_marker(DEST_MARKER_ID)

        if detection is not None:
            self.logger.info(
                f'Destination marker {DEST_MARKER_ID} found at '
                f'{detection["distance"]:.2f}m')
            self.marker_last_seen_time = time.time()
            return STATE_APPROACH_DEST

        self.chassis.rotate(SEARCH_ROTATE_SPEED)
        return STATE_SEARCH_DEST

    def handle_approach_dest(self):
        """
        APPROACH_DEST: Drive toward the destination marker.
        """
        self.chassis.set_led_approaching()

        detection = self.vision.get_marker(DEST_MARKER_ID)

        if detection is None:
            if time.time() - self.marker_last_seen_time > MARKER_LOST_TIMEOUT:
                self.logger.warn('Destination marker lost — searching')
                self.chassis.stop()
                return STATE_SEARCH_DEST
            return STATE_APPROACH_DEST

        self.marker_last_seen_time = time.time()

        # Close enough to place
        if detection['distance'] < DEST_APPROACH_DISTANCE:
            self.logger.info('At destination — stopping to place')
            self.chassis.stop()
            time.sleep(0.5)

            # Fine alignment
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
            if not aligned:
                self.logger.warn('Fine alignment incomplete — proceeding anyway')
            return STATE_PLACE

        # Check for obstacles — skip if the "obstacle" is the destination itself
        if self.obstacle.is_blocked():
            tof = self.obstacle.get_distance()
            if tof >= detection['distance'] - 0.10:
                pass  # ToF is reading the destination marker, not a real obstacle
            else:
                self.logger.warn('Obstacle on approach to destination!')
                self.chassis.stop()
                self.avoid_start_time = time.time()
                self.avoid_nudge_count = 0
                self.pre_avoid_state = STATE_APPROACH_DEST
                return STATE_OBSTACLE_AVOID

        self.chassis.drive_toward_marker(detection)
        return STATE_APPROACH_DEST

    def handle_place(self):
        """
        PLACE: Place the object down, then back away.
        """
        self.chassis.set_led_picking()
        self.chassis.stop()

        self.logger.info('Starting place-down sequence...')
        success = self.arm.place_down()

        if success:
            self.logger.info('Object placed successfully!')
            # Back away
            self.chassis.move_backward(0.15)
            time.sleep(2.0)
            self.chassis.stop()
        else:
            self.logger.error('Place-down failed!')

        return STATE_DONE

    def handle_done(self):
        """
        DONE: Task complete.
        """
        self.chassis.stop()
        self.chassis.set_led_done()
        self.logger.info('========== TASK COMPLETE ==========')
        self.running = False
        return STATE_DONE

    # =================================================================
    # Main loop
    # =================================================================

    def run(self):
        """Run the state machine at CONTROL_LOOP_RATE Hz."""

        handlers = {
            STATE_SEARCH_OBJECT: self.handle_search_object,
            STATE_APPROACH_OBJECT: self.handle_approach_object,
            STATE_PICK_UP: self.handle_pick_up,
            STATE_NAVIGATE: self.handle_navigate,
            STATE_OBSTACLE_AVOID: self.handle_obstacle_avoid,
            STATE_SEARCH_DEST: self.handle_search_dest,
            STATE_APPROACH_DEST: self.handle_approach_dest,
            STATE_PLACE: self.handle_place,
            STATE_DONE: self.handle_done,
        }

        loop_period = 1.0 / CONTROL_LOOP_RATE

        self.logger.info('========================================')
        self.logger.info('  Pick-and-Transport State Machine')
        self.logger.info('========================================')
        self.logger.info(f'Object marker:      ID={OBJECT_MARKER_ID}')
        self.logger.info(f'Waypoint markers:   IDs={WAYPOINT_MARKER_IDS}')
        self.logger.info(f'Destination marker: ID={DEST_MARKER_ID}')
        self.logger.info('========================================')

        # Wait for camera
        self.logger.info('Waiting for camera frames...')
        for _ in range(50):
            if self.vision.get_frame_count() > 0:
                break
            time.sleep(0.1)

        if self.vision.get_frame_count() == 0:
            self.logger.warn(
                'No camera frames — is camera.enabled: true in config?')
        else:
            self.logger.info(
                f'Camera active ({self.vision.get_frame_count()} frames)')

        # Check ToF
        if not self.obstacle.is_sensor_active():
            self.logger.warn(
                'ToF not active — obstacle avoidance disabled. '
                'Set tof.enabled: true in config.')

        # Wait for arm servers
        self.logger.info('Waiting for arm/gripper servers...')
        if not self.arm.wait_for_servers(timeout_sec=10.0):
            self.logger.error(
                'Arm/gripper servers unavailable! '
                'Set arm.enabled and gripper.enabled: true.')
            return

        # Prepare arm
        self.logger.info('Retracting arm...')
        self.arm.retract()

        self.logger.info('Starting! Press Ctrl+C to stop.')
        self.marker_last_seen_time = time.time()
        prev_state = None

        while self.running:
            loop_start = time.time()

            # Log state transitions
            if self.state != prev_state:
                self.logger.info(f'>>> STATE: {self.state}')
                prev_state = self.state

            # Execute handler
            handler = handlers.get(self.state)
            if handler is None:
                self.logger.error(f'Unknown state: {self.state}')
                break

            self.state = handler()

            # Maintain loop rate
            elapsed = time.time() - loop_start
            sleep_time = loop_period - elapsed
            if sleep_time > 0:
                time.sleep(sleep_time)

    def shutdown(self):
        """Clean shutdown."""
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
        self.arm.destroy_node()


# =============================================================================
# Entry point
# =============================================================================
def main():
    rclpy.init()

    sm = StateMachine()

    # Ctrl+C handler — always stop the robot
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
        import traceback
        traceback.print_exc()
    finally:
        sm.shutdown()
        rclpy.shutdown()


if __name__ == '__main__':
    main()