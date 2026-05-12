"""
people_avoidance.py — Person detection and movement classification.

Subscribes to the RoboMaster driver's /vision topic to track people
in the camera frame. Classifies each person as MOVING or STATIONARY
so the state machine can decide whether to wait or bypass.

This node is detection-only — it never publishes cmd_vel.
The state machine in main.py reads the state and controls movement.
"""

import threading
import time
import numpy as np
from collections import deque

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy
from sensor_msgs.msg import Imu
from robomaster_msgs.msg import Detection

from config import CAMERA_FX  # used to convert yaw rate to pixel velocity

SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)

# Person classification states
PERSON_UNKNOWN    = "UNKNOWN"
PERSON_MOVING     = "MOVING"
PERSON_STATIONARY = "STATIONARY"


class PeopleAvoidanceNode(Node):
    """
    Tracks people detected by the RoboMaster driver's vision system.

    Subscribes to /vision and monitors whether a person in the path
    is moving or stationary. Provides get_state() for the state machine
    to query — never touches cmd_vel.
    """

    # ── Tuning constants ──────────────────────────────────────────────────
    VEL_WINDOW_SIZE       = 5      # number of cx samples for velocity estimate
    VEL_EMA_ALPHA         = 0.4    # smoothing factor (higher = more reactive)
    MOVING_THRESHOLD      = 0.15   # smoothed velocity above this → MOVING
    STATIONARY_THRESHOLD  = 0.06   # smoothed velocity below this → STATIONARY
    MOVING_CONFIRM_TIME   = 0.2    # seconds of consistent reading to commit MOVING
    STATIONARY_CONFIRM_TIME = 0.5  # seconds of consistent reading to commit STATIONARY
    AREA_THRESHOLD        = 0.05   # min bounding-box area (fraction of frame) to count
    DETECTION_TIMEOUT     = 0.5    # seconds without detection → clear state
    BYPASS_COOLDOWN       = 2.5    # seconds after bypass before re-classifying

    def __init__(self):
        super().__init__('people_avoidance_node')

        self.lock = threading.Lock()

        # Person detection data
        self._person_cx    = None   # horizontal centre of person (0.0–1.0)
        self._person_area  = 0.0
        self._cx_history   = deque(maxlen=self.VEL_WINDOW_SIZE)
        self._last_seen    = 0.0    # time.time() of last detection

        # Velocity estimate
        self._smoothed_vel = 0.0
        self._robot_yaw    = 0.0   # from IMU, used to compensate camera motion
        self._last_cmd_yaw = 0.0

        # Classification state machine
        self._state          = PERSON_UNKNOWN
        self._candidate      = None
        self._candidate_time = 0.0
        self._last_dt_time   = time.time()

        # Bypass cooldown
        self._bypass_end_time = None

        self.create_subscription(Detection, 'vision', self._vision_cb, SENSOR_QOS)
        self.create_subscription(Imu, 'imu', self._imu_cb, SENSOR_QOS)
        self.create_timer(0.1, self._update_loop)

        self.get_logger().info('PeopleAvoidanceNode initialized — watching /vision for persons')

    # ── Callbacks ─────────────────────────────────────────────────────────

    def _vision_cb(self, msg: Detection):
        if not msg.people:
            with self.lock:
                # Only clear if not in a locked state
                if self._state not in (PERSON_STATIONARY,):
                    self._clear_detection()
            return

        # Pick the largest person in frame (most likely the one in the path)
        best_cx, max_area = 0.5, 0.0
        for person in msg.people:
            area = person.roi.width * person.roi.height
            if area > max_area:
                max_area = area
                best_cx = person.roi.x_offset + person.roi.width / 2.0

        if max_area < self.AREA_THRESHOLD:
            with self.lock:
                if self._state not in (PERSON_STATIONARY,):
                    self._clear_detection()
            return

        now = time.time()
        with self.lock:
            self._person_cx   = best_cx
            self._person_area = max_area
            self._last_seen   = now
            self._cx_history.append((now, best_cx))
            self._update_velocity()

    def _imu_cb(self, msg: Imu):
        with self.lock:
            self._robot_yaw = msg.angular_velocity.z

    # ── Internal helpers ──────────────────────────────────────────────────

    def _update_velocity(self):
        """Compute EMA-smoothed lateral velocity of the person in frame."""
        if len(self._cx_history) < 2:
            self._smoothed_vel = 0.0
            return
        t0, cx0 = self._cx_history[0]
        t1, cx1 = self._cx_history[-1]
        dt = t1 - t0
        if dt <= 0.0:
            return
        # Match original AvoidanceNode: 80° horizontal FOV parameter
        hfov_rad = np.radians(80.0)
        raw_vel = (cx1 - cx0) / dt
        yaw = self._robot_yaw if abs(self._robot_yaw) > 0.001 else self._last_cmd_yaw
        compensated = raw_vel - yaw / hfov_rad
        self._smoothed_vel = (
            self.VEL_EMA_ALPHA * compensated
            + (1.0 - self.VEL_EMA_ALPHA) * self._smoothed_vel
        )

    def _clear_detection(self):
        """Reset detection data and return to UNKNOWN (called with lock held)."""
        self._person_cx    = None
        self._person_area  = 0.0
        self._cx_history.clear()
        self._smoothed_vel = 0.0
        self._candidate    = None
        self._candidate_time = 0.0
        self._state        = PERSON_UNKNOWN

    # ── Update loop (10 Hz) ───────────────────────────────────────────────

    def _update_loop(self):
        now = time.time()
        dt = now - self._last_dt_time
        self._last_dt_time = now

        with self.lock:
            # Expire stale detections (but not while locked in STATIONARY)
            if self._last_seen > 0 and (now - self._last_seen) > self.DETECTION_TIMEOUT:
                if self._state != PERSON_STATIONARY:
                    self._clear_detection()
                return

            if self._person_cx is None:
                return

            vel = abs(self._smoothed_vel)

            # Check bypass cooldown
            in_cooldown = (
                self._bypass_end_time is not None
                and (now - self._bypass_end_time) < self.BYPASS_COOLDOWN
            )

            if vel > self.MOVING_THRESHOLD:
                suggested = PERSON_MOVING
            elif vel < self.STATIONARY_THRESHOLD:
                suggested = PERSON_UNKNOWN if in_cooldown else PERSON_STATIONARY
            else:
                suggested = None  # in transition band — keep current state

            if suggested is None or suggested == self._state:
                self._candidate = None
                self._candidate_time = 0.0
                return

            if suggested != self._candidate:
                self._candidate = suggested
                self._candidate_time = 0.0
            else:
                self._candidate_time += dt

            confirm = (
                self.MOVING_CONFIRM_TIME
                if self._candidate == PERSON_MOVING
                else self.STATIONARY_CONFIRM_TIME
            )
            if self._candidate_time >= confirm:
                prev = self._state
                self._state = self._candidate
                self._candidate = None
                self._candidate_time = 0.0
                self.get_logger().info(
                    f'Person state: {prev} → {self._state} '
                    f'(vel={self._smoothed_vel:+.3f})'
                )

    # ── Public API ────────────────────────────────────────────────────────

    def get_state(self):
        """
        Returns the current person classification state.

        Returns:
            str: PERSON_UNKNOWN, PERSON_MOVING, or PERSON_STATIONARY
        """
        with self.lock:
            return self._state

    def get_person_cx(self):
        """
        Returns the horizontal centre of the detected person (0.0–1.0).
        Returns 0.5 (centre) if no person visible.
        """
        with self.lock:
            return self._person_cx if self._person_cx is not None else 0.5

    def is_person_visible(self):
        """True if a person large enough to react to is currently detected."""
        with self.lock:
            return self._person_cx is not None

    def get_person_area(self):
        """
        Returns the bounding-box area of the detected person as a fraction
        of the frame (0.0–1.0). Returns 0.0 if no person is detected.
        """
        with self.lock:
            return self._person_area

    def get_smoothed_velocity(self):
        """
        Returns the current EMA-smoothed lateral velocity of the person.

        Positive = person moving right in frame.
        Negative = person moving left in frame.
        Returns 0.0 if no person is detected.
        """
        with self.lock:
            return self._smoothed_vel

    def is_person_recently_detected(self, max_age: float = 0.4) -> bool:
        """
        True if a person appeared in an actual camera frame within the last
        max_age seconds.  Unlike is_person_visible(), this ignores state
        retention for STATIONARY persons — it reflects whether the person
        is currently in the camera's field of view.
        """
        with self.lock:
            return self._last_seen > 0 and (time.time() - self._last_seen) < max_age

    def notify_bypass_complete(self):
        """Call this after the bypass maneuver finishes to start cooldown."""
        with self.lock:
            self._bypass_end_time = time.time()
            self._clear_detection()
