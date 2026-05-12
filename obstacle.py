"""
obstacle.py — ToF sensor-based obstacle detection.

Subscribes to the EP Core's Time-of-Flight distance sensor and provides
a simple API for checking if the path ahead is blocked.

Usage:
    from obstacle import ObstacleNode
    obstacle = ObstacleNode()

    # In your control loop:
    if obstacle.is_blocked():
        print(f"Obstacle at {obstacle.get_distance():.2f}m!")
"""

import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Range
import threading
import time

from config import (
    TOF_TOPIC,
    TOF_LEFT_TOPIC,
    TOF_RIGHT_TOPIC,
    OBSTACLE_THRESHOLD,
    OBSTACLE_CLEAR_THRESHOLD,
    DETOUR_SIDE_CLEAR_THRESHOLD,
)


class ObstacleNode(Node):
    """
    ROS 2 node that monitors the ToF distance sensor for obstacles.

    The ToF sensor gives a single distance reading (in meters) in the
    direction it's pointing. On the EP Core, it faces forward.
    """

    def __init__(self):
        super().__init__('obstacle_node')

        self.lock = threading.Lock()

        # Forward TOF
        self._distance      = float('inf')
        self._last_update   = 0.0
        self._sensor_active = False

        # Side TOFs — float('inf') until first reading; _active guards usage
        self._left_distance  = float('inf')
        self._right_distance = float('inf')
        self._left_active    = False
        self._right_active   = False

        self.sub       = self.create_subscription(Range, TOF_TOPIC,       self._range_callback, 10)
        self.left_sub  = self.create_subscription(Range, TOF_LEFT_TOPIC,  self._left_callback,  10)
        self.right_sub = self.create_subscription(Range, TOF_RIGHT_TOPIC, self._right_callback, 10)

        self.get_logger().info(
            f'ObstacleNode initialized — forward={TOF_TOPIC} '
            f'left={TOF_LEFT_TOPIC} right={TOF_RIGHT_TOPIC}')
        self.get_logger().info(
            f'  Obstacle threshold: {OBSTACLE_THRESHOLD}m, '
            f'Clear threshold: {OBSTACLE_CLEAR_THRESHOLD}m, '
            f'Side-clear threshold: {DETOUR_SIDE_CLEAR_THRESHOLD}m')

    # ─────────────────────────────────────────────────────────────
    # ToF callbacks
    # ─────────────────────────────────────────────────────────────

    @staticmethod
    def _parse_range(msg) -> float | None:
        """Validate a Range message and return the distance, or None if invalid."""
        d = msg.range
        if d < 0.0:
            return None
        if msg.max_range > 0.0 and d > msg.max_range:
            return None
        return d

    def _range_callback(self, msg):
        d = self._parse_range(msg)
        if d is None:
            return
        with self.lock:
            self._distance = d
            self._last_update = time.time()
            self._sensor_active = True

    def _left_callback(self, msg):
        d = self._parse_range(msg)
        if d is None:
            return
        with self.lock:
            self._left_distance = d
            self._left_active = True

    def _right_callback(self, msg):
        d = self._parse_range(msg)
        if d is None:
            return
        with self.lock:
            self._right_distance = d
            self._right_active = True

    # ─────────────────────────────────────────────────────────────
    # Public API
    # ─────────────────────────────────────────────────────────────

    def get_distance(self):
        """
        Get the current forward distance reading from the ToF sensor.

        Returns:
            float: Distance in meters. Returns float('inf') if no
                   reading is available.
        """
        with self.lock:
            return self._distance

    def is_blocked(self):
        """
        Check if an obstacle is blocking the path ahead.

        Returns:
            bool: True if an obstacle is closer than OBSTACLE_THRESHOLD.
        """
        with self.lock:
            return self._distance < OBSTACLE_THRESHOLD

    def is_clear(self):
        """
        Check if the path ahead is clear enough to resume navigation.

        Uses a higher threshold than is_blocked() to provide hysteresis
        and avoid oscillating between blocked/clear states.

        Returns:
            bool: True if no obstacle within OBSTACLE_CLEAR_THRESHOLD.
        """
        with self.lock:
            return self._distance >= OBSTACLE_CLEAR_THRESHOLD

    def is_sensor_active(self):
        """
        Check if the ToF sensor is publishing data.

        Returns:
            bool: True if at least one reading has been received.
        """
        with self.lock:
            return self._sensor_active

    def get_left_distance(self) -> float:
        """Distance reading from the left side ToF (float('inf') if no data)."""
        with self.lock:
            return self._left_distance

    def get_right_distance(self) -> float:
        """Distance reading from the right side ToF (float('inf') if no data)."""
        with self.lock:
            return self._right_distance

    def is_detour_side_clear(self, detour_direction: float) -> bool:
        """
        During the U-detour forward leg, return True when the obstacle (which
        sits on the side opposite to the strafe direction) is no longer beside
        the robot.

        detour_direction: +1.0 = robot strafed LEFT  → obstacle is on the RIGHT
                          -1.0 = robot strafed RIGHT → obstacle is on the LEFT

        Returns False if the relevant side sensor has not yet received any data,
        so an inactive sensor never falsely signals 'clear'.
        """
        with self.lock:
            if detour_direction > 0:
                # Strafed left — check right side TOF
                return self._right_active and self._right_distance > DETOUR_SIDE_CLEAR_THRESHOLD
            else:
                # Strafed right — check left side TOF
                return self._left_active and self._left_distance > DETOUR_SIDE_CLEAR_THRESHOLD

    def get_time_since_update(self):
        """
        Get seconds since the last ToF reading.

        Returns:
            float: Seconds since last update, or float('inf') if
                   no reading received yet.
        """
        with self.lock:
            if not self._sensor_active:
                return float('inf')
            return time.time() - self._last_update


# =============================================================================
# Standalone test
# =============================================================================
def main():
    """
    Test the obstacle node standalone.
    Place objects in front of the ToF sensor and see distance readings.

    Note: Make sure the ToF module is enabled in your config.yaml:
        tof:
          enabled: true

    Usage:
        # Terminal 1: launch driver
        # Terminal 2:
        cd ~/robomaster_ws/my_project
        python3 obstacle.py
    """
    rclpy.init()
    node = ObstacleNode()

    def print_status():
        dist = node.get_distance()
        blocked = node.is_blocked()
        active = node.is_sensor_active()

        if not active:
            node.get_logger().warn(
                'No ToF data received yet — is tof.enabled: true in config?')
        else:
            status = "BLOCKED" if blocked else "CLEAR"
            node.get_logger().info(
                f'Distance: {dist:.2f}m  [{status}]')

    timer = node.create_timer(0.5, print_status)

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()