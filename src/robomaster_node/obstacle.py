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
    OBSTACLE_THRESHOLD,
    OBSTACLE_CLEAR_THRESHOLD,
)


class ObstacleNode(Node):
    """
    ROS 2 node that monitors the ToF distance sensor for obstacles.

    The ToF sensor gives a single distance reading (in meters) in the
    direction it's pointing. On the EP Core, it faces forward.
    """

    def __init__(self):
        super().__init__('obstacle_node')

        # ── Shared state (thread-safe) ──
        self.lock = threading.Lock()
        self._distance = float('inf')   # No reading yet = assume clear
        self._last_update = 0.0         # Timestamp of last reading
        self._sensor_active = False     # Have we received any data?

        # Subscribe to ToF sensor
        self.sub = self.create_subscription(
            Range, TOF_TOPIC, self._range_callback, 10)

        self.get_logger().info(
            f'ObstacleNode initialized — listening on /{TOF_TOPIC}')
        self.get_logger().info(
            f'  Obstacle threshold: {OBSTACLE_THRESHOLD}m, '
            f'Clear threshold: {OBSTACLE_CLEAR_THRESHOLD}m')

    # ─────────────────────────────────────────────────────────────
    # ToF callback
    # ─────────────────────────────────────────────────────────────
    def _range_callback(self, msg):
        """Called each time the ToF sensor publishes a distance reading."""
        distance = msg.range

        # ToF sensor may return -1 or 0 for invalid readings
        if distance < 0.0 or distance > msg.max_range:
            return

        with self.lock:
            self._distance = distance
            self._last_update = time.time()
            self._sensor_active = True

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