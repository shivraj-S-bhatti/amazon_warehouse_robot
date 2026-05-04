#!/usr/bin/env python3
"""
robomaster_detection - detection_node.py
========================================
Subscribes to the RoboMaster driver's /vision topic
(robomaster_msgs/msg/Detection) and republishes structured
data for people and markers on dedicated topics.

Topics published
----------------
~/people          robomaster_msgs/msg/Detection   (person-only detections)
~/markers         robomaster_msgs/msg/Detection   (marker-only detections)
~/detection_info  std_msgs/msg/String             (human-readable summary)

Parameters
----------
verbose : bool  (default True)  — log every detection to the console
"""

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy

from std_msgs.msg import String
from robomaster_msgs.msg import Detection

# ─────────────────────────────────────────────────────────────────────────────

SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)


class DetectionNode(Node):
    """Processes and re-publishes RoboMaster vision detections."""

    def __init__(self) -> None:
        super().__init__("robomaster_detection_node")

        # ── parameters ────────────────────────────────────────────────────────
        self.declare_parameter("verbose", True)
        self._verbose: bool = self.get_parameter("verbose").get_parameter_value().bool_value

        # ── subscriber ────────────────────────────────────────────────────────
        self._sub = self.create_subscription(
            Detection,
            "vision",           # published by robomaster_ros driver
            self._detection_cb,
            SENSOR_QOS,
        )

        # ── publishers ────────────────────────────────────────────────────────
        self._people_pub = self.create_publisher(
            Detection, "~/people", 10
        )
        self._markers_pub = self.create_publisher(
            Detection, "~/markers", 10
        )
        self._info_pub = self.create_publisher(
            String, "~/detection_info", 10
        )

        self.get_logger().info(
            "DetectionNode started — subscribing to /vision"
        )

    # ── callback ──────────────────────────────────────────────────────────────

    def _detection_cb(self, msg: Detection) -> None:
        """Called whenever the driver publishes a new Detection frame."""

        stamp = msg.header.stamp

        # ── people ────────────────────────────────────────────────────────────
        if msg.people:
            people_msg = Detection()
            people_msg.header = msg.header
            people_msg.people = msg.people
            self._people_pub.publish(people_msg)

            if self._verbose:
                for i, person in enumerate(msg.people):
                    cx, cy = _roi_center(person.roi)
                    area = _roi_area(person.roi)
                    self.get_logger().info(
                        f"[PERSON {i}] centre=({cx:.3f}, {cy:.3f})  "
                        f"size={person.roi.width:.3f}×{person.roi.height:.3f}  "
                        f"area={area:.4f}"
                    )

        # ── markers ───────────────────────────────────────────────────────────
        if msg.markers:
            markers_msg = Detection()
            markers_msg.header = msg.header
            markers_msg.markers = msg.markers
            self._markers_pub.publish(markers_msg)

            if self._verbose:
                for i, marker in enumerate(msg.markers):
                    cx, cy = _roi_center(marker.roi)
                    area = _roi_area(marker.roi)
                    self.get_logger().info(
                        f"[MARKER {i}] kind={marker.kind!r}  "
                        f"centre=({cx:.3f}, {cy:.3f})  "
                        f"size={marker.roi.width:.3f}×{marker.roi.height:.3f}  "
                        f"area={area:.4f}"
                    )

        # ── info summary ──────────────────────────────────────────────────────
        n_people  = len(msg.people)
        n_markers = len(msg.markers)
        n_robots  = len(msg.robots)
        n_gestures = len(msg.gestures)
        n_lines   = len(msg.lines)

        if n_people + n_markers + n_robots + n_gestures + n_lines > 0:
            marker_kinds = [m.kind for m in msg.markers]
            summary = (
                f"t={stamp.sec}.{stamp.nanosec:09d} | "
                f"people={n_people} | "
                f"markers={n_markers}{(' ' + str(marker_kinds)) if marker_kinds else ''} | "
                f"robots={n_robots} | "
                f"gestures={n_gestures} | "
                f"lines={n_lines}"
            )
            info_msg = String()
            info_msg.data = summary
            self._info_pub.publish(info_msg)


# ─────────────────────────────────────────────────────────────────────────────
# helpers
# ─────────────────────────────────────────────────────────────────────────────

def _roi_center(roi) -> tuple[float, float]:
    """Return the (x, y) centre of a RegionOfInterest (values in [0,1])."""
    cx = roi.x_offset + roi.width  / 2.0
    cy = roi.y_offset + roi.height / 2.0
    return cx, cy


def _roi_area(roi) -> float:
    """Return the fractional area of a RegionOfInterest."""
    return roi.width * roi.height


# ─────────────────────────────────────────────────────────────────────────────
# entry point
# ─────────────────────────────────────────────────────────────────────────────

def main(args=None):
    rclpy.init(args=args)
    node = DetectionNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
