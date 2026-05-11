#!/usr/bin/env python3
"""
color_check.py — Find out what color the camera is actually perceiving.

Print-quality "red" can drift quite far in HSV space — many inkjets produce
coral, magenta, or pink instead of the saturated red the DJI vision SDK
filter expects. This tool subscribes to the EP's camera stream, samples a
patch in the centre of every Nth frame, and prints both the raw HSV value
and the SDK color category it is most likely to fall into (red, green,
blue, or "none of the above").

Use it to figure out which value of `marker:<color>` you should put in your
detection.launch.py, OR to discover that your print is so off that no
color filter will ever match it (in which case you need to reprint).

Usage:
    # Terminal 1 — launch the driver so /camera/image_color is alive
    ros2 launch robomaster_detection detection.launch.py \\
        model:=ep conn_type:=ap

    # Terminal 2 — this tool
    source ~/robomaster_ws/install/setup.bash
    python3 color_check.py

Procedure:
    1. Hold your marker so it fills the centre of the frame
       (the tool samples a 60×60 px patch in the middle).
    2. Watch one of these log lines per ~half second:
           center patch:  BGR=(…)  HSV=(…)  → RED
    3. Whatever it says (RED / GREEN / BLUE) is the value to put after
       "marker:" in detection.launch.py. If it says LOW-SATURATION or
       names a color the SDK doesn't filter on (yellow, magenta, cyan),
       your print is the problem — reprint with more saturated ink, or
       buy / download the official DJI markers.
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from cv_bridge import CvBridge


CAMERA_TOPIC = "camera/image_color"
PATCH_SIZE = 60              # px — width and height of the centre patch
LOG_EVERY_N_FRAMES = 10      # ~2 Hz at 20 fps


class ColorCheck(Node):
    def __init__(self) -> None:
        super().__init__("color_check")

        self.bridge = CvBridge()
        self.frame_count = 0

        self.create_subscription(
            Image, CAMERA_TOPIC, self._cb, 10
        )

        self.get_logger().warn("=" * 64)
        self.get_logger().warn(" CAMERA COLOR PROBE")
        self.get_logger().warn("")
        self.get_logger().warn(f" subscribing to {CAMERA_TOPIC}")
        self.get_logger().warn(
            f" sampling a {PATCH_SIZE}×{PATCH_SIZE} px patch at the centre"
        )
        self.get_logger().warn(" Hold your marker so it fills the centre of frame.")
        self.get_logger().warn("=" * 64)

    def _cb(self, msg: Image) -> None:
        try:
            frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding="bgr8")
        except Exception as e:
            self.get_logger().warn(f"image conversion failed: {e}")
            return

        self.frame_count += 1
        if self.frame_count % LOG_EVERY_N_FRAMES != 0:
            return

        h, w = frame.shape[:2]
        py = (h - PATCH_SIZE) // 2
        px = (w - PATCH_SIZE) // 2
        patch_bgr = frame[py:py + PATCH_SIZE, px:px + PATCH_SIZE]
        patch_hsv = cv2.cvtColor(patch_bgr, cv2.COLOR_BGR2HSV)

        mean_bgr = np.mean(patch_bgr.reshape(-1, 3), axis=0)
        mean_hsv = np.mean(patch_hsv.reshape(-1, 3), axis=0)

        b, g, r = mean_bgr
        H, S, V = mean_hsv

        # Classify using OpenCV's H ∈ [0, 180], S/V ∈ [0, 255] convention.
        # The DJI SDK's exact hue thresholds aren't documented, but its three
        # "marker_detection_color" categories are pure red, green, and blue.
        # Anything else won't pass its color filter.
        category, advice = _classify(H, S, V)

        self.get_logger().info(
            f"center patch:  BGR=({b:5.1f},{g:5.1f},{r:5.1f})  "
            f"HSV=({H:5.1f},{S:5.1f},{V:5.1f})  "
            f"→ {category}    {advice}"
        )


def _classify(H: float, S: float, V: float) -> tuple[str, str]:
    """Map HSV pixel values to one of the three DJI-supported filters."""
    if V < 40:
        return "TOO DARK", "lighting is too dim — light the marker better"
    if S < 60:
        return (
            "LOW SATURATION (looks grey/white)",
            "marker isn't colorful enough for any filter — reprint or relight",
        )

    if H < 10 or H > 165:
        return "RED", "use \"marker:red\" in detection.launch.py"
    if 35 < H < 85:
        return "GREEN", "use \"marker:green\" in detection.launch.py"
    if 100 < H < 135:
        return "BLUE", "use \"marker:blue\" in detection.launch.py"

    if 10 <= H <= 35:
        return (
            "YELLOW / ORANGE",
            "between red and green — SDK will reject; reprint with deeper red",
        )
    if 85 <= H <= 100:
        return (
            "CYAN",
            "between green and blue — SDK will reject; reprint with deeper blue/green",
        )
    if 135 <= H <= 165:
        return (
            "MAGENTA / PINK",
            "between blue and red — SDK will reject; reprint with deeper red",
        )

    return ("UNKNOWN", "no DJI filter matches this hue — reprint")


def main(args=None) -> None:
    rclpy.init(args=args)
    node = ColorCheck()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
