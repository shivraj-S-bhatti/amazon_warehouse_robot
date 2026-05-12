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
    "DICT_4X4_50":  cv2.aruco.DICT_4X4_50,
    "DICT_4X4_100": cv2.aruco.DICT_4X4_100,
    "DICT_5X5_50":  cv2.aruco.DICT_5X5_50,
    "DICT_5X5_100": cv2.aruco.DICT_5X5_100,
    "DICT_6X6_50":  cv2.aruco.DICT_6X6_50,
    "DICT_6X6_100": cv2.aruco.DICT_6X6_100,
}

# ── Subpixel refinement window (pixels).  Smaller = faster but less precise.
# Half of the typical marker border width at 360p works well.
_SUBPIX_WIN  = (5, 5)
_SUBPIX_ZERO = (-1, -1)          # no dead zone
_SUBPIX_CRITERIA = (
    cv2.TERM_CRITERIA_EPS + cv2.TERM_CRITERIA_MAX_ITER, 30, 0.01
)

# ── Maximum reprojection error (pixels) to accept a solvePnP solution.
# IPPE_SQUARE returns two candidate poses; we pick the one with the lower
# reprojection error.  If both exceed this threshold the detection is dropped.
_MAX_REPROJ_ERROR = 2.5   # pixels — tighten for a noisy camera


def _reprojection_error(
    obj_pts: np.ndarray,
    img_pts: np.ndarray,
    rvec: np.ndarray,
    tvec: np.ndarray,
    camera_matrix: np.ndarray,
    dist_coeffs: np.ndarray,
) -> float:
    """Return mean pixel reprojection error for a given pose."""
    projected, _ = cv2.projectPoints(
        obj_pts, rvec, tvec, camera_matrix, dist_coeffs
    )
    return float(np.mean(np.linalg.norm(img_pts - projected.reshape(-1, 2), axis=1)))


class VisionNode(Node):
    """
    ROS 2 node that detects ArUco markers from the EP Core camera.

    Maintains an internal dictionary of all currently visible markers,
    updated every frame. Other modules query this via get_marker().

    Improvements over the baseline:
    - Subpixel corner refinement for more accurate pose estimation.
    - IPPE_SQUARE dual-solution arbitration: picks the pose with the lower
      reprojection error and discards detections that exceed _MAX_REPROJ_ERROR.
    - Tuned DetectorParameters for the EP Core's 360p wide-FOV feed.
    - CLAHE contrast enhancement so detection survives poor lighting.
    - Object points pre-allocated once instead of per frame.
    - EMA covers lateral, distance, vertical *and* horizontal_error.
    - Brief-dropout tolerance: a marker that vanishes for up to
      DETECTION_HOLD_FRAMES is kept in the confirmed set (with the last
      smoothed values), so a single occluded frame doesn't force
      re-confirmation from scratch.
    - get_confirmation_count() is now lock-protected (was a data race).
    """

    def __init__(self):
        super().__init__("vision_node")

        self.bridge = CvBridge()

        # ── Camera intrinsics ──────────────────────────────────────────────
        self.camera_matrix = np.array(
            [[CAMERA_FX, 0, CAMERA_CX], [0, CAMERA_FY, CAMERA_CY], [0, 0, 1]],
            dtype=np.float64,
        )
        # Zero distortion — replace with calibrated values if available.
        self.dist_coeffs = np.zeros(5)

        # ── Marker geometry (pre-allocated, reused every frame) ────────────
        half = MARKER_SIZE / 2.0
        self._obj_pts = np.array(
            [[-half,  half, 0],
             [ half,  half, 0],
             [ half, -half, 0],
             [-half, -half, 0]],
            dtype=np.float32,
        )
        self.marker_size = MARKER_SIZE

        # ── ArUco detector ─────────────────────────────────────────────────
        dict_id = ARUCO_DICTS.get(ARUCO_DICT_TYPE, cv2.aruco.DICT_4X4_50)
        self.aruco_dict = cv2.aruco.getPredefinedDictionary(dict_id)

        params = cv2.aruco.DetectorParameters()

        # Adaptive thresholding — wider window range handles uneven
        # illumination better than the defaults at 360p.
        params.adaptiveThreshWinSizeMin  = 3
        params.adaptiveThreshWinSizeMax  = 53   # default 23 misses distant markers
        params.adaptiveThreshWinSizeStep = 10
        params.adaptiveThreshConstant    = 7

        # Contour filtering — allow smaller markers (further away) and
        # reject obviously large blobs that aren't markers.
        params.minMarkerPerimeterRate = 0.02   # default 0.03 — loosen for distance
        params.maxMarkerPerimeterRate = 4.0

        # Corner refinement — CONTOUR is fast and sufficient for 360p.
        # Switch to SUBPIX if you already apply cornerSubPix yourself.
        
        # CHANGE THIS LINE:
        params.cornerRefinementMethod   = cv2.aruco.CORNER_REFINE_NONE 
        
        params.cornerRefinementWinSize  = 5
        params.cornerRefinementMaxIterations = 30
        params.cornerRefinementMinAccuracy  = 0.1

        # Bit extraction — slightly larger border tolerance for markers
        # printed on slightly reflective surfaces.
        params.perspectiveRemoveIgnoredMarginPerCell = 0.15  # default 0.13

        # Marker validation — how many error bits are tolerated.
        params.errorCorrectionRate = 0.6   # default 0.6, leave unless FPs are high

        self.aruco_params = params
        self.detector = cv2.aruco.ArucoDetector(self.aruco_dict, self.aruco_params)

        # ── CLAHE for low-light / high-contrast scenes ─────────────────────
        # Applied to the gray image before detection.  clipLimit=2.0 is
        # conservative; raise to 4.0 if the environment is very dark.
        self._clahe = cv2.createCLAHE(clipLimit=2.0, tileGridSize=(8, 8))

        # ── Detection smoothing / confirmation ─────────────────────────────
        # Minimum consecutive frames a marker must be seen before it is
        # reported via get_marker().
        self.DETECTION_MIN_FRAMES = 4

        # EMA weight for new samples (higher = more reactive, less smooth).
        self.DETECTION_EMA_ALPHA = 0.35

        # A marker that disappears briefly is kept in the confirmed set for
        # this many frames, avoiding a full re-confirmation cycle after a
        # momentary occlusion or detection dropout.
        self.DETECTION_HOLD_FRAMES = 3

        # ── Shared state (all accesses under self.lock) ────────────────────
        self.lock = threading.Lock()
        self.detections      = {}   # confirmed + smoothed  {marker_id: detection_dict}
        self._raw_detections = {}   # current-frame raw      {marker_id: detection_dict}
        self._smoothed       = {}   # EMA state              {marker_id: detection_dict}
        self._frame_counts   = {}   # consecutive-seen count {marker_id: int}
        self._hold_counts    = {}   # frames since last seen {marker_id: int}
        self.latest_frame    = None
        self.frame_count     = 0

        self.sub = self.create_subscription(
            Image, CAMERA_TOPIC, self._image_callback, 10
        )
        self.get_logger().info("VisionNode initialized — waiting for camera frames")

    # ─────────────────────────────────────────────────────────────────────────
    # Internal helpers
    # ─────────────────────────────────────────────────────────────────────────

    def _refine_corners(self, gray: np.ndarray, corners: list) -> list:
        """
        Apply subpixel refinement to all detected marker corners.

        cornerSubPix improves lateral/distance accuracy noticeably at ranges
        beyond ~0.5 m where individual corner pixels matter most.
        """
        refined = []
        for c in corners:
            pts = c[0].copy()  # shape (4, 2), float32
            cv2.cornerSubPix(gray, pts, _SUBPIX_WIN, _SUBPIX_ZERO, _SUBPIX_CRITERIA)
            refined.append(pts.reshape(1, 4, 2))
        return refined

    def _solve_pose(self, img_pts: np.ndarray):
        """
        Estimate pose using IPPE_SQUARE and pick the better of the two solutions.

        IPPE_SQUARE always returns exactly two poses.  The correct one is the
        one with the smaller reprojection error *and* positive Z (in front of
        the camera).  If neither meets the reprojection threshold, returns None.

        Returns:
            (rvec, tvec, reproj_error) or None
        """
        ret, rvecs, tvecs, reproj_errors = cv2.solvePnPGeneric(
            self._obj_pts, img_pts,
            self.camera_matrix, self.dist_coeffs,
            flags=cv2.SOLVEPNP_IPPE_SQUARE,
        )
        if not ret:
            return None

        best = None
        best_err = float("inf")

        for rvec, tvec, err_arr in zip(rvecs, tvecs, reproj_errors):
            tz = float(tvec[2])
            if tz <= 0:          # behind the camera — skip
                continue
            err = float(err_arr[0]) if err_arr is not None else _reprojection_error(
                self._obj_pts, img_pts, rvec, tvec,
                self.camera_matrix, self.dist_coeffs,
            )
            if err < best_err:
                best_err = err
                best = (rvec, tvec)

        if best is None or best_err > _MAX_REPROJ_ERROR:
            return None

        rvec, tvec = best
        return rvec.flatten(), tvec.flatten(), best_err

    # ─────────────────────────────────────────────────────────────────────────
    # Camera callback
    # ─────────────────────────────────────────────────────────────────────────

    def _image_callback(self, msg: Image) -> None:
        """Process each camera frame: detect markers and estimate poses."""
        try:
            frame = self.bridge.imgmsg_to_cv2(msg, desired_encoding="bgr8")
        except Exception as e:
            self.get_logger().warn(f"Failed to convert image: {e}")
            return

        with self.lock:
            self.frame_count += 1

        # ── Preprocessing ──
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        gray = self._clahe.apply(gray)              # contrast-normalise

        # ── Detection ──
        corners, ids, _ = self.detector.detectMarkers(gray)

        new_detections: dict = {}

        if ids is not None and len(ids) > 0:
            # Subpixel refinement on all corners at once
            corners = self._refine_corners(gray, corners)

            for i, marker_id in enumerate(ids.flatten()):
                img_pts = corners[i][0].astype(np.float32)

                pose = self._solve_pose(img_pts)
                if pose is None:
                    # Bad reprojection — skip this detection entirely
                    continue

                rvec, tvec, reproj_err = pose
                tx, ty, tz = tvec

                cx = float(np.mean(img_pts[:, 0]))
                cy = float(np.mean(img_pts[:, 1]))
                area = float(cv2.contourArea(img_pts))

                # Horizontal error derived from the *pose* (lateral / distance)
                # rather than raw pixel position — more accurate at oblique angles.
                horizontal_error = tx / tz if tz > 1e-6 else 0.0

                new_detections[int(marker_id)] = {
                    "id":               int(marker_id),
                    "distance":         float(tz),
                    "lateral":          float(tx),
                    "vertical":         float(ty),
                    "pixel_center":     (cx, cy),
                    "horizontal_error": float(horizontal_error),
                    "area":             float(area),
                    "corners":          img_pts,
                    "rvec":             rvec,
                    "tvec":             tvec,
                    "reproj_error":     reproj_err,
                }

            cv2.aruco.drawDetectedMarkers(frame, corners, ids)

        # ── Smoothing, confirmation, and hold-on-dropout ───────────────────
        alpha      = self.DETECTION_EMA_ALPHA
        min_frames = self.DETECTION_MIN_FRAMES
        hold_max   = self.DETECTION_HOLD_FRAMES

        confirmed: dict = {}

        # --- markers visible this frame ---
        for mid, det in new_detections.items():
            if mid in self._smoothed:
                prev = self._smoothed[mid]
                # EMA over all continuous pose-derived scalars
                for key in ("lateral", "distance", "vertical", "horizontal_error"):
                    det[key] = alpha * det[key] + (1.0 - alpha) * prev[key]
                self._frame_counts[mid] = self._frame_counts.get(mid, 0) + 1
            else:
                self._frame_counts[mid] = 1

            self._hold_counts.pop(mid, None)   # reset hold timer
            self._smoothed[mid] = det

            if self._frame_counts[mid] >= min_frames:
                confirmed[mid] = det

        # --- markers NOT visible this frame ---
        disappeared = set(self._smoothed.keys()) - set(new_detections.keys())
        for mid in disappeared:
            hold = self._hold_counts.get(mid, 0) + 1
            self._hold_counts[mid] = hold

            if hold <= hold_max and mid in self.detections:
                # Keep the last confirmed reading during the hold window
                confirmed[mid] = self.detections[mid]
            else:
                # Hold expired — evict all state for this marker
                self._frame_counts.pop(mid, None)
                self._smoothed.pop(mid, None)
                self._hold_counts.pop(mid, None)

        # ── Atomic state update ────────────────────────────────────────────
        with self.lock:
            self.detections      = confirmed
            self._raw_detections = new_detections
            self.latest_frame    = frame

    # ─────────────────────────────────────────────────────────────────────────
    # Public API — called from main.py
    # ─────────────────────────────────────────────────────────────────────────

    def get_marker(self, marker_id: int):
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
                - 'horizontal_error': tan(angle) from optical axis (+ = right)
                - 'pixel_center': (cx, cy) in pixels
                - 'area': marker area in pixels²
                - 'reproj_error': mean reprojection error in pixels
            Returns None if marker is not currently visible.
        """
        with self.lock:
            return self.detections.get(marker_id, None)

    def get_any_marker(self, marker_ids: list):
        """
        Get detection for the first visible marker from a list of IDs.

        Args:
            marker_ids (list[int]): List of marker IDs to search for.

        Returns:
            dict or None
        """
        with self.lock:
            for mid in marker_ids:
                if mid in self.detections:
                    return self.detections[mid]
            return None

    def get_all_markers(self) -> dict:
        """
        Get all currently detected markers.

        Returns:
            dict: {marker_id: detection_dict} for all visible markers.
        """
        with self.lock:
            return dict(self.detections)

    def is_marker_visible(self, marker_id: int) -> bool:
        """Check if a specific marker is currently visible."""
        with self.lock:
            return marker_id in self.detections

    def get_frame(self):
        """
        Get the latest camera frame with ArUco detections drawn.

        Returns:
            numpy.ndarray or None
        """
        with self.lock:
            if self.latest_frame is not None:
                return self.latest_frame.copy()
            return None

    def get_frame_count(self) -> int:
        """Get number of frames processed so far."""
        with self.lock:
            return self.frame_count

    # ── Debugging helpers ──────────────────────────────────────────────────

    def get_raw_marker(self, marker_id: int):
        """
        Unfiltered, unsmoothed detection from the current frame.
        Bypasses the EMA smoother and min-frame gate.

        Normal callers should prefer get_marker().
        """
        with self.lock:
            return self._raw_detections.get(marker_id, None)

    def get_confirmation_count(self, marker_id: int) -> int:
        """
        How many consecutive frames a marker has been detected so far.
        Reaches DETECTION_MIN_FRAMES before get_marker() will return data.

        Returns:
            int: 0 if not currently visible.
        """
        with self.lock:   # was missing — fixed data race
            return self._frame_counts.get(marker_id, 0)

    def get_hold_count(self, marker_id: int) -> int:
        """
        How many frames a disappeared marker has been held.
        Zero if the marker is currently visible.
        """
        with self.lock:
            return self._hold_counts.get(marker_id, 0)


# =============================================================================
# Standalone test
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

    def print_detections():
        markers = node.get_all_markers()
        if markers:
            for mid, det in markers.items():
                node.get_logger().info(
                    f"Marker {mid}: "
                    f'dist={det["distance"]:.2f}m  '
                    f'lat={det["lateral"]:.3f}m  '
                    f'h_err={det["horizontal_error"]:.3f}  '
                    f'reproj={det["reproj_error"]:.2f}px  '
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