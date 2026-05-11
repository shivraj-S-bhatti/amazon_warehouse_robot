#!/usr/bin/env python3
"""
marker_diagnostic.py — Standalone diagnostic for DJI vision-marker detection.

Run this alongside the launch file to see exactly what the EP's onboard
vision SDK is putting onto /vision. It is the fastest way to figure out
WHY markers aren't being detected.

Usage:
    # Terminal 1: launch the driver + detection + avoidance
    ros2 launch robomaster_detection detection.launch.py \\
        model:=ep conn_type:=ap

    # Terminal 2: this diagnostic
    source ~/robomaster_ws/install/setup.bash
    python3 marker_diagnostic.py

Every 2 seconds it prints a status block like:

    [VISION DIAG] msgs=84 (21.0 Hz)
      people:   total=37  current_frame=1
      markers:  total=0   current_frame=0    kinds_ever_seen=(none)
      gestures: total=0
      lines:    total=0
      robots:   total=0

Whenever a marker is actually detected by the SDK, it also prints a single
loud line like:

    [MARKER DETECTED!] kind=['A'] roi_area=0.0231

Interpreting the output
-----------------------
- `msgs` stays at 0:
      The driver's vision module never initialised. Re-check that
      `vision.enabled: True` is in the launch file and that there are no
      errors during driver startup.
- `people: total > 0` but `markers: total = 0`:
      The vision module is alive but the SDK is rejecting your marker.
      One of:
        1. Color filter mismatch — your marker isn't the color the launch
           file is filtering for (default: red). Edit `vision.targets` in
           the launch file: try "marker:green" or "marker:blue".
        2. The pattern isn't a recognised DJI vision-marker design. Only
           the official set works (red heart, sword, arrows, digits 0–9,
           letters A–Z, the three colored squares, and !, ?, #). Print
           one from the DJI Robomaster app or website.
        3. Marker is too small, too far, badly lit, tilted, glossy, or
           moving. Hold a card-sized marker stationary at ~50–80 cm,
           square to the camera, in good light.
- `markers: total > 0` and `kinds_ever_seen` lists strings like
  ['1', 'A', 'heart']:
      Detection IS working. The avoidance node should already be
      transitioning out of SEARCHING.
"""

import sys

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy
from robomaster_msgs.msg import Detection


SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)

VISION_TOPIC = "/vision"


class MarkerDiagnostic(Node):
    def __init__(self) -> None:
        super().__init__("marker_diagnostic")

        # Aggregate counters
        self.msg_count: int = 0
        self.last_msg_count: int = 0

        self.people_total: int = 0
        self.marker_total: int = 0
        self.gesture_total: int = 0
        self.line_total: int = 0
        self.robot_total: int = 0

        # Current-frame counts (reset every status tick to the most recent value)
        self.last_people_in_frame: int = 0
        self.last_markers_in_frame: int = 0

        # What kinds have we ever seen?
        self.marker_kinds_seen: set[str] = set()

        # Already warned about no-markers-but-people pattern?
        self._already_warned: bool = False

        self.create_subscription(
            Detection, VISION_TOPIC, self._vision_cb, SENSOR_QOS
        )
        self.create_timer(2.0, self._tick)

        self.get_logger().warn("=" * 64)
        self.get_logger().warn(" DJI VISION-MARKER DIAGNOSTIC")
        self.get_logger().warn("")
        self.get_logger().warn(f" subscribing to {VISION_TOPIC}")
        self.get_logger().warn(" hold up your DJI marker and watch for the line:")
        self.get_logger().warn("    [MARKER DETECTED!] kind=[…]")
        self.get_logger().warn("=" * 64)

    # ───────────────────────────────────────────────────────────────────
    # Callbacks
    # ───────────────────────────────────────────────────────────────────
    def _vision_cb(self, msg: Detection) -> None:
        self.msg_count += 1

        n_people  = len(msg.people)
        n_markers = len(msg.markers)
        n_gest    = len(msg.gestures)
        n_line    = len(msg.lines)
        n_robot   = len(msg.robots)

        self.people_total  += n_people
        self.marker_total  += n_markers
        self.gesture_total += n_gest
        self.line_total    += n_line
        self.robot_total   += n_robot

        self.last_people_in_frame  = n_people
        self.last_markers_in_frame = n_markers

        if n_markers > 0:
            kinds = [m.kind for m in msg.markers]
            self.marker_kinds_seen.update(kinds)
            for m in msg.markers:
                roi_area = float(m.roi.width) * float(m.roi.height)
                self.get_logger().warn(
                    f"[MARKER DETECTED!] kind={m.kind!r}  "
                    f"roi_area={roi_area:.4f}  "
                    f"centre=({m.roi.x_offset + m.roi.width / 2.0:.3f}, "
                    f"{m.roi.y_offset + m.roi.height / 2.0:.3f})"
                )

    def _tick(self) -> None:
        new_msgs = self.msg_count - self.last_msg_count
        rate_hz = new_msgs / 2.0
        self.last_msg_count = self.msg_count

        if self.msg_count == 0:
            self.get_logger().error(
                f"No messages on {VISION_TOPIC} yet. "
                "Is the driver actually running with vision.enabled=True?"
            )
            return

        kinds_str = (
            "(none yet — SDK has not classified any marker)"
            if not self.marker_kinds_seen
            else str(sorted(self.marker_kinds_seen))
        )

        self.get_logger().info(
            f"\n[VISION DIAG] msgs={self.msg_count} ({rate_hz:.1f} Hz)\n"
            f"  people:   total={self.people_total:<5d} "
            f"current_frame={self.last_people_in_frame}\n"
            f"  markers:  total={self.marker_total:<5d} "
            f"current_frame={self.last_markers_in_frame}    "
            f"kinds_ever_seen={kinds_str}\n"
            f"  gestures: total={self.gesture_total}\n"
            f"  lines:    total={self.line_total}\n"
            f"  robots:   total={self.robot_total}"
        )

        if (
            self.marker_total == 0
            and self.people_total > 0
            and not self._already_warned
            and self.msg_count > 30
        ):
            self._already_warned = True
            self.get_logger().warn(
                "\n"
                "  ┌────────────────────────────────────────────────────────────┐\n"
                "  │ People detection is working but NO markers are coming in.  │\n"
                "  │ The DJI SDK is rejecting whatever you're holding up.       │\n"
                "  │ Try, in order:                                             │\n"
                "  │   1. Use an OFFICIAL DJI vision marker (red heart, digit,  │\n"
                "  │      letter, arrow). Custom prints will not be detected.   │\n"
                "  │   2. Edit detection.launch.py and change marker:red to     │\n"
                "  │      marker:green or marker:blue, then re-launch.          │\n"
                "  │   3. Hold the marker stationary, square to the camera,     │\n"
                "  │      at ~50–80 cm, in bright even light.                   │\n"
                "  └────────────────────────────────────────────────────────────┘"
            )


def main(args=None) -> None:
    rclpy.init(args=args)
    node = MarkerDiagnostic()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main(sys.argv)
