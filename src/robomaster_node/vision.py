
"""
vision.py — ArUco marker detection and pose estimation.

Subscribes to the EP Core camera, detects ArUco markers,
and provides distance + lateral offset for visual servoing.

Usage:
    from vision import VisionNode
    vision = VisionNode()

    # In your control loop:
    detection = vision.get_marker(marker_id=1)
    if detection is not None:
        print(f"Distance: {detection['distance']:.2f}m")
        print(f"Lateral:  {detection['lateral']:.2f}m")
"""

import cv2
import numpy as np
import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
from cv_bridge import CvBridge
import threading

from config import (
    CAMERA_TOPIC,
    CAMERA_FX,
    CAMERA_FY,
    CAMERA_CX,
    CAMERA_CY,
    ARUCO_DICT_TYPE,
    MARKER_SIZE,
)

# Map string name to OpenCV ArUco dictionary
ARUCO_DICTS = {
    "DICT_4X4_50": cv2.aruco.DICT_4X4_50,
    "DICT_4X4_100": cv2.aruco.DICT_4X4_100,
    "DICT_5X5_50": cv2.aruco.DICT_5X5_50,
    "DICT_5X5_100": cv2.aruco.DICT_5X5_100,
    "DICT_6X6_50": cv2.aruco.DICT_6X6_50,
    "DICT_6X6_100": cv2.aruco.DICT_6X6_100,
}


class VisionNode(Node):
    """
    ROS 2 node that detects ArUco markers from the EP Core camera.

    Maintains an internal dictionary of all currently visible markers,
    updated every frame. Other modules query this via get_marker().
    """

    def __init__(self):
        super().__init__("vision_node")

        # CV bridge for ROS Image ↔ OpenCV conversion
        self.bridge = CvBridge()

        # Camera intrinsic matrix (approximate for EP Core at 360p)
        self.camera_matrix = np.array(
            [[CAMERA_FX, 0, CAMERA_CX], [0, CAMERA_FY, CAMERA_CY], [0, 0, 1]],
            dtype=np.float64,
        )

        # Distortion coefficients (assume zero — EP Core has wide FOV
        # but we don't have calibration data; set to zero for now)
        self.dist_coeffs = np.zeros(5)

        # ArUco detector setup
        dict_id = ARUCO_DICTS.get(ARUCO_DICT_TYPE, cv2.aruco.DICT_4X4_50)
        self.aruco_dict = cv2.aruco.getPredefinedDictionary(dict_id)
        self.aruco_params = cv2.aruco.DetectorParameters()
        self.detector = cv2.aruco.ArucoDetector(self.aruco_dict, self.aruco_params)

        # Marker size for pose estimation
        self.marker_size = MARKER_SIZE

        # ── Shared state (thread-safe) ──
        self.lock = threading.Lock()
        self.detections = {}  # {marker_id: detection_dict}
        self.latest_frame = None  # Latest OpenCV frame for debugging
        self.frame_count = 0

        # Subscribe to camera
        self.sub = self.create_subscription(
            Image, CAMERA_TOPIC, self._image_callback, 10
        )

        self.get_logger().info("VisionNode initialized — waiting for camera frames")

    # ─────────────────────────────────────────────────────────────
    # Camera callback
    # ─────────────────────────────────────────────────────────────
    def _image_callback(self, msg):
        """Process each camera frame: detect markers and estimate poses."""
        try:
            frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding="bgr8")
        except Exception as e:
            self.get_logger().warn(f"Failed to convert image: {e}")
            return

        with self.lock:
            self.frame_count += 1

        # Convert to grayscale for detection
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

        # Detect markers
        corners, ids, _ = self.detector.detectMarkers(gray)

        new_detections = {}

        if ids is not None:
            for i, marker_id in enumerate(ids.flatten()):
                # Estimate pose: get translation and rotation vectors
                # estimatePoseSingleMarkers removed in OpenCV 4.7+; use solvePnP instead
                half = self.marker_size / 2.0
                obj_pts = np.array([
                    [-half,  half, 0],
                    [ half,  half, 0],
                    [ half, -half, 0],
                    [-half, -half, 0],
                ], dtype=np.float32)
                img_pts = corners[i][0].astype(np.float32)
                _, rvec, tvec = cv2.solvePnP(
                    obj_pts, img_pts,
                    self.camera_matrix, self.dist_coeffs,
                    flags=cv2.SOLVEPNP_IPPE_SQUARE,
                )
                tx, ty, tz = tvec.flatten()

                # Compute center of marker in pixel coordinates
                marker_corners = corners[i][0]  # shape: (4, 2)
                cx = np.mean(marker_corners[:, 0])
                cy = np.mean(marker_corners[:, 1])

                # Compute marker area in pixels (for rough size estimate)
                area = cv2.contourArea(marker_corners.astype(np.float32))

                # Horizontal error: normalized (-1 to +1)
                # Negative = marker is left, positive = marker is right
                horizontal_error = (cx - CAMERA_CX) / CAMERA_CX

                detection = {
                    "id": int(marker_id),
                    "distance": float(tz),  # meters forward
                    "lateral": float(tx),  # meters right (+) / left (-)
                    "vertical": float(ty),  # meters down (+) / up (-)
                    "pixel_center": (float(cx), float(cy)),
                    "horizontal_error": float(horizontal_error),
                    "area": float(area),
                    "corners": marker_corners,
                    "rvec": rvec.flatten(),
                    "tvec": tvec.flatten(),
                }

                new_detections[int(marker_id)] = detection

            # Draw detected markers on frame for debugging
            cv2.aruco.drawDetectedMarkers(frame, corners, ids)

        # Update shared state
        with self.lock:
            self.detections = new_detections
            self.latest_frame = frame

    # ─────────────────────────────────────────────────────────────
    # Public API — called from main.py
    # ─────────────────────────────────────────────────────────────

    def get_marker(self, marker_id):
        """
        Get detection info for a specific marker.

        Args:
            marker_id (int): The ArUco marker ID to look for.

        Returns:
            dict or None: Detection dictionary with keys:
                - 'id': marker ID
                - 'distance': forward distance in meters
                - 'lateral': lateral offset in meters (+ = right)
                - 'vertical': vertical offset in meters (+ = down)
                - 'horizontal_error': normalized error (-1 to +1)
                - 'pixel_center': (cx, cy) in pixels
                - 'area': marker area in pixels
            Returns None if marker is not currently visible.
        """
        with self.lock:
            return self.detections.get(marker_id, None)

    def get_any_marker(self, marker_ids):
        """
        Get detection for the first visible marker from a list of IDs.

        Args:
            marker_ids (list[int]): List of marker IDs to search for.

        Returns:
            dict or None: Detection dict for the first found marker,
                          or None if none are visible.
        """
        with self.lock:
            for mid in marker_ids:
                if mid in self.detections:
                    return self.detections[mid]
            return None

    def get_all_markers(self):
        """
        Get all currently detected markers.

        Returns:
            dict: {marker_id: detection_dict} for all visible markers.
        """
        with self.lock:
            return dict(self.detections)

    def is_marker_visible(self, marker_id):
        """Check if a specific marker is currently visible."""
        with self.lock:
            return marker_id in self.detections

    def get_frame(self):
        """
        Get the latest camera frame with ArUco detections drawn.
        Useful for debugging with cv2.imshow().

        Returns:
            numpy.ndarray or None
        """
        with self.lock:
            if self.latest_frame is not None:
                return self.latest_frame.copy()
            return None

    def get_frame_count(self):
        """Get number of frames processed so far."""
        return self.frame_count


# =============================================================================
# Standalone test — run this file directly to verify camera + detection
# =============================================================================
def main():
    """
    Test the vision node standalone.
    Hold an ArUco marker in front of the camera and see detection output.

    Usage:
        # Terminal 1: launch driver
        ros2 launch robomaster_ros main.launch model:=ep conn_type:=ap

        # Terminal 2: test vision
        cd ~/robomaster_ws/my_project
        python3 vision.py
    """
    rclpy.init()
    node = VisionNode()

    # Create a timer to print detections every 0.5 seconds
    def print_detections():
        markers = node.get_all_markers()
        if markers:
            for mid, det in markers.items():
                node.get_logger().info(
                    f"Marker {mid}: "
                    f'dist={det["distance"]:.2f}m, '
                    f'lateral={det["lateral"]:.3f}m, '
                    f'h_error={det["horizontal_error"]:.2f}, '
                    f'area={det["area"]:.0f}px²'
                )
        else:
            node.get_logger().info("No markers detected")

    node.create_timer(0.5, print_detections)

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
