"""
chassis_control.py — Chassis movement, visual servoing, and LED control.

Provides high-level movement functions that the state machine in main.py
calls to control the robot's motion.

Usage:
    from chassis_control import ChassisController
    chassis = ChassisController()

    # Rotate to search for markers
    chassis.rotate(speed=0.3)

    # Drive toward a detected marker
    chassis.drive_toward_marker(detection)

    # Strafe sideways for obstacle avoidance
    chassis.strafe_left(speed=0.15)

    # Stop
    chassis.stop()
"""

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from std_msgs.msg import ColorRGBA
import math

from config import (
    SEARCH_ROTATE_SPEED,
    APPROACH_MAX_SPEED,
    APPROACH_MIN_SPEED,
    STRAFE_SPEED,
    NAVIGATE_SPEED,
    STEER_KP,
    SPEED_KP,
    STEER_DEADZONE,
    LED_SEARCHING,
    LED_APPROACHING,
    LED_PICKING,
    LED_NAVIGATING,
    LED_AVOIDING,
    LED_DONE,
)


class ChassisController(Node):
    """
    ROS 2 node for controlling the EP Core's chassis and LEDs.

    Publishes Twist messages to /cmd_vel and ColorRGBA to /leds/color.
    """

    def __init__(self):
        super().__init__('chassis_controller')

        # Publishers
        self.cmd_pub = self.create_publisher(Twist, 'cmd_vel', 10)
        self.led_pub = self.create_publisher(ColorRGBA, 'leds/color', 10)

        self.get_logger().info('ChassisController initialized')

    # ─────────────────────────────────────────────────────────────
    # Basic movement commands
    # ─────────────────────────────────────────────────────────────

    def stop(self):
        """Stop all chassis movement."""
        msg = Twist()
        # All fields default to 0.0
        self.cmd_pub.publish(msg)

    def move(self, linear_x=0.0, linear_y=0.0, angular_z=0.0):
        """
        Send a raw velocity command.

        Args:
            linear_x: Forward/backward speed (m/s). Positive = forward.
            linear_y: Left/right speed (m/s). Positive = left.
            angular_z: Rotation speed (rad/s). Positive = counter-clockwise.
        """
        msg = Twist()
        msg.linear.x = float(linear_x)
        msg.linear.y = float(linear_y)
        msg.angular.z = float(angular_z)
        self.cmd_pub.publish(msg)

    def move_forward(self, speed=None):
        """Move forward at a given speed (default: NAVIGATE_SPEED)."""
        if speed is None:
            speed = NAVIGATE_SPEED
        self.move(linear_x=speed)

    def move_backward(self, speed=None):
        """Move backward at a given speed."""
        if speed is None:
            speed = NAVIGATE_SPEED
        self.move(linear_x=-speed)

    def rotate(self, speed=None):
        """
        Rotate in place (counter-clockwise by default).

        Args:
            speed: Rotation speed in rad/s. Positive = CCW, negative = CW.
                   Default: SEARCH_ROTATE_SPEED.
        """
        if speed is None:
            speed = SEARCH_ROTATE_SPEED
        self.move(angular_z=speed)

    def strafe_left(self, speed=None):
        """Strafe left using Mecanum wheels."""
        if speed is None:
            speed = STRAFE_SPEED
        self.move(linear_y=abs(speed))

    def strafe_right(self, speed=None):
        """Strafe right using Mecanum wheels."""
        if speed is None:
            speed = STRAFE_SPEED
        self.move(linear_y=-abs(speed))

    # ─────────────────────────────────────────────────────────────
    # Visual servoing — drive toward a detected marker
    # ─────────────────────────────────────────────────────────────

    def drive_toward_marker(self, detection, approach_speed=None):
        """
        Drive toward a detected ArUco marker using proportional control.

        Uses the marker's lateral offset to steer and distance to
        control forward speed.

        Args:
            detection (dict): Detection from VisionNode.get_marker().
                Must contain 'lateral' and 'distance' keys.
            approach_speed (float, optional): Override max forward speed.

        Returns:
            tuple: (forward_speed, angular_speed) that was commanded.
        """
        if approach_speed is None:
            approach_speed = APPROACH_MAX_SPEED

        lateral = detection['lateral']    # meters, + = right of center
        distance = detection['distance']  # meters ahead

        # ── Steering (angular.z) ──
        # Negative lateral means marker is left → turn left (positive angular.z)
        # Positive lateral means marker is right → turn right (negative angular.z)
        if abs(lateral) < STEER_DEADZONE:
            angular_z = 0.0
        else:
            angular_z = -STEER_KP * lateral

        # Clamp angular speed
        angular_z = max(-0.5, min(0.5, angular_z))

        # ── Forward speed (linear.x) ──
        # Proportional to distance — slow down as we get closer
        forward_speed = SPEED_KP * distance
        forward_speed = max(APPROACH_MIN_SPEED, min(approach_speed, forward_speed))

        # If the marker is far off-center, stop and rotate in place first
        if abs(angular_z) > 0.3:
            forward_speed = 0.0

        self.move(linear_x=forward_speed, angular_z=angular_z)

        return (forward_speed, angular_z)

    def align_to_marker(self, detection):
        """
        Rotate in place to center a marker in the camera frame.
        Used for fine alignment before picking/placing.

        Args:
            detection (dict): Detection from VisionNode.get_marker().

        Returns:
            bool: True if the marker is approximately centered.
        """
        lateral = detection['lateral']

        if abs(lateral) < STEER_DEADZONE:
            self.stop()
            return True

        # Rotate slowly toward the marker
        angular_z = -STEER_KP * lateral * 0.5  # Slower gain for fine alignment
        angular_z = max(-0.4, min(0.4, angular_z))
        self.move(angular_z=angular_z)
        return False

    # ─────────────────────────────────────────────────────────────
    # Navigation helpers
    # ─────────────────────────────────────────────────────────────

    def navigate_toward_marker(self, detection):
        """
        Navigate toward a waypoint marker at cruising speed.
        Similar to drive_toward_marker but with less aggressive
        slowdown — we don't need to stop precisely at waypoints.

        Args:
            detection (dict): Detection from VisionNode.get_marker().

        Returns:
            tuple: (forward_speed, angular_speed) that was commanded.
        """
        lateral = detection['lateral']
        distance = detection['distance']

        # Steering
        if abs(lateral) < STEER_DEADZONE:
            angular_z = 0.0
        else:
            angular_z = -STEER_KP * lateral

        angular_z = max(-0.5, min(0.5, angular_z))

        # Maintain cruising speed — only slow down when very close
        if distance < 0.5:
            forward_speed = APPROACH_MIN_SPEED + (NAVIGATE_SPEED - APPROACH_MIN_SPEED) * (distance / 0.5)
        else:
            forward_speed = NAVIGATE_SPEED

        # Stop and rotate in place if the marker is far off-center
        if abs(angular_z) > 0.3:
            forward_speed = 0.0

        self.move(linear_x=forward_speed, angular_z=angular_z)

        return (forward_speed, angular_z)

    # ─────────────────────────────────────────────────────────────
    # LED control
    # ─────────────────────────────────────────────────────────────

    def set_led_color(self, rgba_tuple):
        """
        Set all LEDs to a single color.

        Args:
            rgba_tuple: (r, g, b, a) with values 0.0 to 1.0.
                        Use constants from config.py like LED_SEARCHING.
        """
        msg = ColorRGBA()
        msg.r = float(rgba_tuple[0])
        msg.g = float(rgba_tuple[1])
        msg.b = float(rgba_tuple[2])
        msg.a = float(rgba_tuple[3])
        self.led_pub.publish(msg)

    def set_led_searching(self):
        """Blue LEDs — robot is searching for a marker."""
        self.set_led_color(LED_SEARCHING)

    def set_led_approaching(self):
        """Yellow LEDs — robot is approaching a target."""
        self.set_led_color(LED_APPROACHING)

    def set_led_picking(self):
        """Orange LEDs — robot is picking or placing."""
        self.set_led_color(LED_PICKING)

    def set_led_navigating(self):
        """Cyan LEDs — robot is navigating between waypoints."""
        self.set_led_color(LED_NAVIGATING)

    def set_led_avoiding(self):
        """Red LEDs — robot is avoiding an obstacle."""
        self.set_led_color(LED_AVOIDING)

    def set_led_done(self):
        """Green LEDs — task complete."""
        self.set_led_color(LED_DONE)


# =============================================================================
# Standalone test
# =============================================================================
def main():
    """
    Test chassis control standalone.
    The robot will: move forward 2s → strafe left 1s → rotate 1s → stop.
    LEDs change color with each action.

    WARNING: Robot will move! Clear the area first.

    Usage:
        # Terminal 1: launch driver with chassis.enabled: true
        # Terminal 2:
        cd ~/robomaster_ws/my_project
        python3 chassis_control.py
    """
    import time

    rclpy.init()
    node = ChassisController()

    try:
        node.get_logger().info('Starting chassis test in 3 seconds...')
        time.sleep(3)

        # Test forward
        node.get_logger().info('Moving forward...')
        node.set_led_navigating()
        node.move_forward(0.2)
        time.sleep(2)

        # Test strafe
        node.get_logger().info('Strafing left...')
        node.set_led_avoiding()
        node.strafe_left(0.15)
        time.sleep(1)

        # Test rotation
        node.get_logger().info('Rotating...')
        node.set_led_searching()
        node.rotate(0.5)
        time.sleep(1)

        # Stop
        node.get_logger().info('Stopping.')
        node.stop()
        node.set_led_done()
        time.sleep(1)

        node.get_logger().info('Test complete!')

    except KeyboardInterrupt:
        node.stop()
    finally:
        node.stop()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()