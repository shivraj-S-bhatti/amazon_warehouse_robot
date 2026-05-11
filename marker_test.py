#!/usr/bin/env python3
"""
marker_test.py — RoboMaster Marker Detection Diagnostic
========================================================
Run this INSTEAD of your full launch stack to isolate exactly where
the marker detection pipeline is breaking.

It monitors three layers simultaneously:
  Layer 1 — Raw driver output:      /vision  (or /<namespace>/vision)
  Layer 2 — Detection node output:  /robomaster_detection_node/markers
  Layer 3 — Topic health:           lists every active topic at startup

Usage
-----
  # With no namespace (default launch):
  ros2 run robomaster_detection marker_test

  # If you launched with name:=ep (or any other namespace):
  ros2 run robomaster_detection marker_test --ros-args -p namespace:=ep

What to look for
----------------
  ✓  Layer 1 shows markers  → driver is detecting, detection_node is broken
  ✓  Layer 1 empty          → driver isn't detecting (lighting/distance/target param)
  ✓  Layer 2 shows markers  → full pipeline working, check avoidance_node subscription
  ✗  /vision not listed     → driver not running or wrong namespace

DJI Marker IDs (for reference)
-------------------------------
  0  = emit-sounds    1  = stop         2  = dice
  3  = target         4  = left-arrow   5  = right-arrow
  6  = forward-arrow  8  = red-heart
  10-19 = numbers 0–9
  20-45 = letters A–Z

IMPORTANT PHYSICAL REQUIREMENTS
--------------------------------
  • Markers must be printed in RED on white background (the EP/S1 only
    detects red variants — a black-and-white printout will NOT work)
  • Detection range: 0.5 m – 3.0 m (too close or too far = no detection)
  • Good ambient light is required (avoid dim rooms or direct backlight)
  • Hold the card flat and square-on to the camera
"""

import sys
import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy

from robomaster_msgs.msg import Detection

# ─────────────────────────────────────────────────────────────────────────────

SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)

MARKER_ID_NAMES = {
    0:  "emit-sounds",
    1:  "stop",
    2:  "dice",
    3:  "target",
    4:  "left-arrow",
    5:  "right-arrow",
    6:  "forward-arrow",
    8:  "red-heart",
    **{10 + i: f"number-{i}" for i in range(10)},
    **{20 + i: f"letter-{chr(65+i)}" for i in range(26)},
}

SEP  = "─" * 72
SEP2 = "═" * 72


class MarkerDiagNode(Node):

    def __init__(self, namespace: str = "") -> None:
        super().__init__("marker_diag_node")

        self.ns = namespace.strip("/")
        prefix  = f"/{self.ns}" if self.ns else ""

        # Counters
        self.vision_msg_count   = 0
        self.vision_marker_hits = 0
        self.repub_msg_count    = 0
        self.repub_marker_hits  = 0

        # ── Layer 1: raw /vision from the driver ──────────────────────────
        raw_topic = f"{prefix}/vision"
        self.create_subscription(
            Detection,
            raw_topic,
            self._raw_vision_cb,
            SENSOR_QOS,
        )

        # ── Layer 2: republished markers from detection_node ──────────────
        repub_topic = f"{prefix}/robomaster_detection_node/markers"
        self.create_subscription(
            Detection,
            repub_topic,
            self._repub_markers_cb,
            SENSOR_QOS,
        )

        # ── Periodic summary timer ─────────────────────────────────────────
        self.create_timer(5.0, self._print_summary)

        # ── Startup banner ─────────────────────────────────────────────────
        self.get_logger().info(SEP2)
        self.get_logger().info("  MARKER DIAGNOSTIC — starting up")
        self.get_logger().info(SEP2)
        self.get_logger().info(f"  Namespace   : '{self.ns}' (empty = no namespace)")
        self.get_logger().info(f"  Layer 1 sub : {raw_topic}")
        self.get_logger().info(f"  Layer 2 sub : {repub_topic}")
        self.get_logger().info(SEP2)
        self.get_logger().info(
            "  CHECKLIST BEFORE TESTING:"
        )
        self.get_logger().info(
            "  [?] Markers printed in RED on white? (B&W prints do NOT work)"
        )
        self.get_logger().info(
            "  [?] Card held 0.5 – 3.0 m from camera?"
        )
        self.get_logger().info(
            "  [?] vision.targets includes 'marker' in driver params?"
        )
        self.get_logger().info(
            "  [?] Sufficient ambient lighting?"
        )
        self.get_logger().info(SEP2)

        # Print active topics after a short delay (graph needs to settle)
        self.create_timer(2.0, self._print_topic_list)
        self._topic_list_printed = False

    # ── Callbacks ─────────────────────────────────────────────────────────

    def _raw_vision_cb(self, msg: Detection) -> None:
        """Called for every frame the driver publishes on /vision."""
        self.vision_msg_count += 1

        if not msg.markers:
            return  # nothing to report this frame

        self.vision_marker_hits += 1
        self.get_logger().info(SEP)
        self.get_logger().info(
            f"  [LAYER 1 ✓] Raw /vision — "
            f"{len(msg.markers)} marker(s) detected!"
        )
        for i, m in enumerate(msg.markers):
            cx   = m.roi.x_offset + m.roi.width  / 2.0
            cy   = m.roi.y_offset + m.roi.height / 2.0
            area = m.roi.width * m.roi.height

            # Try to resolve numeric kind to a human label
            label = _resolve_kind(m.kind)

            self.get_logger().info(
                f"    Marker[{i}]  kind={m.kind!r}  ({label})  "
                f"centre=({cx:.3f}, {cy:.3f})  "
                f"size={m.roi.width:.3f}×{m.roi.height:.3f}  "
                f"area={area:.4f}"
            )
        self.get_logger().info(SEP)

        # Also print everything else in the frame for context
        if msg.people:
            self.get_logger().info(
                f"  [LAYER 1]   Also in frame: {len(msg.people)} person(s)"
            )
        if msg.robots:
            self.get_logger().info(
                f"  [LAYER 1]   Also in frame: {len(msg.robots)} robot(s)"
            )

    def _repub_markers_cb(self, msg: Detection) -> None:
        """Called for every message detection_node republishes on ~/markers."""
        self.repub_msg_count += 1

        if not msg.markers:
            return

        self.repub_marker_hits += 1
        self.get_logger().info(
            f"  [LAYER 2 ✓] Republished markers topic — "
            f"{len(msg.markers)} marker(s) — pipeline OK end-to-end!"
        )

    # ── Summary & helpers ──────────────────────────────────────────────────

    def _print_summary(self) -> None:
        self.get_logger().info(SEP)
        self.get_logger().info("  SUMMARY (last 5 s)")
        self.get_logger().info(SEP)

        # Layer 1 health
        if self.vision_msg_count == 0:
            self.get_logger().info(
                "  [LAYER 1 ✗] No messages on /vision at all."
            )
            self.get_logger().info(
                "               → Is the driver running?  "
                "Run: ros2 topic list | grep vision"
            )
            if self.ns:
                self.get_logger().info(
                    f"               → Did you launch with name:={self.ns}? "
                    f"Try: ros2 topic echo /{self.ns}/vision"
                )
        else:
            self.get_logger().info(
                f"  [LAYER 1  ] /vision alive — "
                f"{self.vision_msg_count} msgs, "
                f"{self.vision_marker_hits} had markers"
            )
            if self.vision_marker_hits == 0:
                self.get_logger().info(
                    "  [LAYER 1 ✗] Driver receiving frames but NO markers detected."
                )
                self.get_logger().info(
                    "               Possible causes:"
                )
                self.get_logger().info(
                    "               1. vision.targets missing 'marker' → check launch file"
                )
                self.get_logger().info(
                    "               2. Marker not red → must be red-on-white printed"
                )
                self.get_logger().info(
                    "               3. Distance outside 0.5–3.0 m range"
                )
                self.get_logger().info(
                    "               4. Poor lighting or marker obscured / tilted"
                )

        # Layer 2 health
        if self.repub_msg_count == 0:
            self.get_logger().info(
                "  [LAYER 2 ✗] No messages on republished markers topic."
            )
            self.get_logger().info(
                "               → Is detection_node running?"
            )
        else:
            self.get_logger().info(
                f"  [LAYER 2  ] Republished topic alive — "
                f"{self.repub_msg_count} msgs, "
                f"{self.repub_marker_hits} had markers"
            )
            if self.repub_marker_hits == 0 and self.vision_marker_hits > 0:
                self.get_logger().info(
                    "  [LAYER 2 ✗] Markers seen on /vision but NOT republished."
                )
                self.get_logger().info(
                    "               → Check detection_node.py _detection_cb"
                )

        self.get_logger().info(SEP)

        # Reset counters for next window
        self.vision_msg_count   = 0
        self.vision_marker_hits = 0
        self.repub_msg_count    = 0
        self.repub_marker_hits  = 0

    def _print_topic_list(self) -> None:
        if self._topic_list_printed:
            return
        self._topic_list_printed = True

        all_topics = self.get_topic_names_and_types()
        vision_topics  = [(t, types) for t, types in all_topics if "vision"    in t]
        detect_topics  = [(t, types) for t, types in all_topics if "detection" in t]
        cmd_vel_topics = [(t, types) for t, types in all_topics if "cmd_vel"   in t]

        self.get_logger().info(SEP2)
        self.get_logger().info("  ACTIVE TOPICS (filtered)")
        self.get_logger().info(SEP2)

        if vision_topics:
            self.get_logger().info("  /vision topics found:")
            for t, types in vision_topics:
                self.get_logger().info(f"    {t}  [{', '.join(types)}]")
        else:
            self.get_logger().info(
                "  [✗] NO /vision topics found — driver may not be running"
            )

        if detect_topics:
            self.get_logger().info("  /detection topics found:")
            for t, types in detect_topics:
                self.get_logger().info(f"    {t}  [{', '.join(types)}]")
        else:
            self.get_logger().info(
                "  [✗] NO /detection topics found — detection_node may not be running"
            )

        if cmd_vel_topics:
            for t, types in cmd_vel_topics:
                self.get_logger().info(f"  /cmd_vel: {t}  [{', '.join(types)}]")

        self.get_logger().info(SEP2)


# ─────────────────────────────────────────────────────────────────────────────

def _resolve_kind(kind) -> str:
    """Map a marker kind (int or string) to a human-readable label."""
    try:
        return MARKER_ID_NAMES.get(int(kind), f"unknown({kind})")
    except (ValueError, TypeError):
        # kind may already be a string like "number_1" from the SDK
        return str(kind)


# ─────────────────────────────────────────────────────────────────────────────

def main(args=None):
    rclpy.init(args=args)

    # Allow passing a namespace via ROS param so you don't need to
    # hardcode it:  ros2 run ... --ros-args -p namespace:=ep
    tmp_node = rclpy.create_node("_tmp_ns_reader")
    tmp_node.declare_parameter("namespace", "")
    ns = tmp_node.get_parameter("namespace").get_parameter_value().string_value
    tmp_node.destroy_node()

    node = MarkerDiagNode(namespace=ns)
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()