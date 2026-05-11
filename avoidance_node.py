#!/usr/bin/env python3
"""
RoboMaster Avoidance Node — v12 (Marker Search & Approach)
===========================================================

Control Architecture
--------------------
A two-layer state machine drives the robot:

  Top-level  (nav_state)
  ├── SEARCHING   No marker visible → spin in place
  ├── CENTERING   Marker found, |cx − 0.5| > center_threshold → turn to align
  └── APPROACHING Marker centred → drive forward while running person avoidance

  Sub-level  (person_state, active only during APPROACHING)
  ├── UNKNOWN      classifying person motion / post-bypass cooldown
  ├── MOVING       person crossing → track & steer
  ├── STATIONARY   person blocking → observe then trigger bypass
  └── BYPASSING    committed S-curve maneuver (pre-empts all marker updates)

Changes vs v11
--------------
  1. Adds _markers_cb subscribing to /robomaster_detection_node/markers.
  2. Replaces the old "drive straight when path clear" section with the
     SEARCHING / CENTERING / APPROACHING nav-state machine.
  3. APPROACHING adds a soft heading-hold correction so the marker stays
     centred while the robot moves forward.
  4. All v11 fixes (bypass timer, stationary lock, detection-timeout guard)
     are preserved unchanged.

Topics subscribed
-----------------
  /range_0                               sensor_msgs/msg/Range
  /imu                                   sensor_msgs/msg/Imu
  /robomaster_detection_node/people      robomaster_msgs/msg/Detection
  /robomaster_detection_node/markers     robomaster_msgs/msg/Detection   ← NEW

Topics published
----------------
  /cmd_vel                               geometry_msgs/msg/Twist

Parameters
----------
  --- existing (unchanged) ---
  forward_speed              float  0.2    m/s, approach speed
  turn_speed                 float  0.4    rad/s, person-tracking turn
  reaction_area_threshold    float  0.05   min marker/person ROI area
  detection_timeout          float  0.5    s, person-lost timeout
  emergency_stop_dist        float  0.6    m, ToF hard stop
  vel_window_size            int    5
  vel_ema_alpha              float  0.4
  moving_threshold           float  0.15
  stationary_threshold       float  0.06
  moving_confirm_time        float  0.2    s
  stationary_confirm_time    float  0.5    s
  stationary_observation_time float 1.2   s
  bypass_turn_speed          float  0.8    rad/s
  bypass_curve_time          float  1.5    s
  bypass_straight_time       float  2.5    s
  post_bypass_cooldown       float  2.5    s
  camera_hfov_deg            float  80.0
  use_cmd_vel_compensation   bool   True

  --- new ---
  search_spin_speed          float  0.5    rad/s, CCW spin while searching
  center_threshold           float  0.08   normalised cx error to call it centred
  center_kp                  float  1.5    P-gain for centering controller
  center_turn_speed          float  0.6    rad/s max for centering (and approach hold)
  marker_timeout             float  1.0    s without a marker msg → back to SEARCHING
"""

import numpy as np
from collections import deque

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy

from geometry_msgs.msg import Twist
from sensor_msgs.msg import Range, Imu
from robomaster_msgs.msg import Detection

# ─────────────────────────────────────────────────────────────────────────────

SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)

# Top-level navigation states
NAV_SEARCHING   = "SEARCHING"    # spinning to look for a marker
NAV_CENTERING   = "CENTERING"    # aligning marker to the frame centre
NAV_APPROACHING = "APPROACHING"  # driving toward the marker

# Person sub-states (only relevant inside NAV_APPROACHING)
STATE_UNKNOWN    = "UNKNOWN"
STATE_MOVING     = "MOVING"
STATE_STATIONARY = "STATIONARY"
STATE_BYPASSING  = "BYPASSING"


# ─────────────────────────────────────────────────────────────────────────────

class AvoidanceNode(Node):

    def __init__(self):
        super().__init__("robomaster_avoidance_node")

        # ── Parameters ────────────────────────────────────────────────────
        self.declare_parameter("forward_speed",               0.2)
        self.declare_parameter("turn_speed",                  0.4)
        self.declare_parameter("reaction_area_threshold",     0.05)
        self.declare_parameter("detection_timeout",           0.5)
        self.declare_parameter("emergency_stop_dist",         0.2)

        self.declare_parameter("vel_window_size",             5)
        self.declare_parameter("vel_ema_alpha",               0.4)
        self.declare_parameter("moving_threshold",            0.15)
        self.declare_parameter("stationary_threshold",        0.06)
        self.declare_parameter("moving_confirm_time",         0.2)
        self.declare_parameter("stationary_confirm_time",     0.5)
        self.declare_parameter("stationary_observation_time", 1.2)

        self.declare_parameter("bypass_turn_speed",           0.8)
        self.declare_parameter("bypass_curve_time",           1.5)
        self.declare_parameter("bypass_straight_time",        2.5)
        self.declare_parameter("post_bypass_cooldown",        2.5)

        self.declare_parameter("camera_hfov_deg",             80.0)
        self.declare_parameter("use_cmd_vel_compensation",    True)

        # Marker search & approach
        self.declare_parameter("search_spin_speed",           0.5)
        self.declare_parameter("center_threshold",            0.08)
        self.declare_parameter("center_kp",                   1.5)
        self.declare_parameter("center_turn_speed",           0.6)
        self.declare_parameter("marker_timeout",              1.0)

        # Resolve values
        self.forward_speed            = self.get_parameter("forward_speed").value
        self.turn_speed               = self.get_parameter("turn_speed").value
        self.bypass_turn_speed        = self.get_parameter("bypass_turn_speed").value
        self.area_threshold           = self.get_parameter("reaction_area_threshold").value
        self.detection_timeout        = self.get_parameter("detection_timeout").value
        self.emergency_stop_dist      = self.get_parameter("emergency_stop_dist").value

        self.vel_window_size          = self.get_parameter("vel_window_size").value
        self.vel_ema_alpha            = self.get_parameter("vel_ema_alpha").value
        self.moving_threshold         = self.get_parameter("moving_threshold").value
        self.stationary_threshold     = self.get_parameter("stationary_threshold").value
        self.moving_confirm_time      = self.get_parameter("moving_confirm_time").value
        self.stationary_confirm_time  = self.get_parameter("stationary_confirm_time").value

        self.obs_time                 = self.get_parameter("stationary_observation_time").value
        self.bypass_curve_time        = self.get_parameter("bypass_curve_time").value
        self.bypass_straight_time     = self.get_parameter("bypass_straight_time").value
        self.bypass_cooldown_time     = self.get_parameter("post_bypass_cooldown").value

        hfov_deg                      = self.get_parameter("camera_hfov_deg").value
        self.hfov_rad                 = np.radians(hfov_deg)
        self.use_cmd_compensation     = self.get_parameter("use_cmd_vel_compensation").value

        self.search_spin_speed        = self.get_parameter("search_spin_speed").value
        self.center_threshold         = self.get_parameter("center_threshold").value
        self.center_kp                = self.get_parameter("center_kp").value
        self.center_turn_speed        = self.get_parameter("center_turn_speed").value
        self.marker_timeout           = self.get_parameter("marker_timeout").value

        # ── State variables ───────────────────────────────────────────────

        # Top-level navigation
        self.nav_state                = NAV_SEARCHING

        # Marker tracking
        self.marker_cx                = 0.5      # normalised horizontal centre [0,1]
        self.marker_area              = 0.0
        self.last_marker_time         = None     # rclpy Time, or None if never seen

        # Person / obstacle avoidance
        self.current_distance         = float("inf")
        self.target_cx                = None
        self.target_area              = 0.0
        self.cx_history               = deque(maxlen=self.vel_window_size)
        self.smoothed_velocity        = 0.0
        self.last_detection_time      = self.get_clock().now()
        self.robot_yaw_rate           = 0.0
        self.last_cmd_angular_z       = 0.0

        self.person_state             = STATE_UNKNOWN
        self.candidate_state          = None
        self.candidate_timer          = 0.0
        self.stationary_timer         = 0.0

        self.bypass_timer             = 0.0
        self.bypass_direction         = 1.0
        self.last_bypass_end_time     = None
        self.accumulated_yaw          = 0.0

        self.last_loop_time           = self.get_clock().now()
        self.current_dt               = 0.1

        # ── Subscriptions ─────────────────────────────────────────────────
        self.create_subscription(
            Range,     "/range_0",
            self._tof_cb,     SENSOR_QOS)
        self.create_subscription(
            Imu,       "/imu",
            self._imu_cb,     SENSOR_QOS)
        self.create_subscription(
            Detection, "/robomaster_detection_node/people",
            self._people_cb,  SENSOR_QOS)
        self.create_subscription(
            Detection, "/robomaster_detection_node/markers",
            self._markers_cb, SENSOR_QOS)

        # ── Publisher & timer ─────────────────────────────────────────────
        self._cmd_pub = self.create_publisher(Twist, "/cmd_vel", 10)
        self.create_timer(0.1, self._control_loop)

        self.get_logger().info(
            "AvoidanceNode v12 started — "
            "spin-search → centre → approach with person avoidance."
        )

    # ─────────────────────────────────────────────────────────────────────
    # Sensor callbacks
    # ─────────────────────────────────────────────────────────────────────

    def _tof_cb(self, msg: Range) -> None:
        dist = msg.range
        if dist > 10.0:            # driver publishes mm, convert to m
            dist = dist / 1000.0
        self.current_distance = dist if dist > 0.0 else float("inf")

    def _imu_cb(self, msg: Imu) -> None:
        self.robot_yaw_rate = msg.angular_velocity.z

    def _markers_cb(self, msg: Detection) -> None:
        """Track the largest visible marker by ROI area."""
        if not msg.markers:
            return  # loss-of-signal is handled by the marker_timeout watchdog

        best_cx  = 0.5
        max_area = 0.0
        for marker in msg.markers:
            cx   = marker.roi.x_offset + marker.roi.width  / 2.0
            area = marker.roi.width * marker.roi.height
            if area > max_area:
                max_area = area
                best_cx  = cx

        self.marker_cx        = best_cx
        self.marker_area      = max_area
        self.last_marker_time = self.get_clock().now()

    def _people_cb(self, msg: Detection) -> None:
        """Person avoidance callback — data is only consumed in APPROACHING."""
        # Bypass is committed — ignore all vision updates
        if self.person_state == STATE_BYPASSING:
            return

        # STATIONARY lock: refresh timeout clock but don't change motion vars
        self.last_detection_time = self.get_clock().now()
        if self.person_state == STATE_STATIONARY:
            return

        if not msg.people:
            self._clear_person_detection()
            return

        best_cx  = 0.5
        max_area = 0.0
        for person in msg.people:
            cx   = person.roi.x_offset + person.roi.width  / 2.0
            area = person.roi.width * person.roi.height
            if area > max_area:
                max_area = area
                best_cx  = cx

        self.target_cx   = best_cx
        self.target_area = max_area

        now_s = self.get_clock().now().nanoseconds / 1e9
        self.cx_history.append((now_s, best_cx))
        self._update_person_velocity()

    # ─────────────────────────────────────────────────────────────────────
    # Internal helpers
    # ─────────────────────────────────────────────────────────────────────

    def _update_person_velocity(self) -> None:
        if len(self.cx_history) < 2:
            self.smoothed_velocity = 0.0
            return

        t_old, cx_old = self.cx_history[0]
        t_new, cx_new = self.cx_history[-1]
        dt = t_new - t_old
        if dt <= 0.0:
            return

        raw_vel     = (cx_new - cx_old) / dt
        yaw_rate    = (self.robot_yaw_rate
                       if abs(self.robot_yaw_rate) > 0.001
                       else self.last_cmd_angular_z)
        compensated = raw_vel - yaw_rate / self.hfov_rad
        self.smoothed_velocity = (
            self.vel_ema_alpha * compensated
            + (1.0 - self.vel_ema_alpha) * self.smoothed_velocity
        )

    def _clear_person_detection(self) -> None:
        self.target_cx         = None
        self.target_area       = 0.0
        self.cx_history.clear()
        self.smoothed_velocity = 0.0
        self.candidate_state   = None
        self.candidate_timer   = 0.0

    def _reset_person_state(self) -> None:
        """Full reset of the person sub-state machine (called on nav-state transitions)."""
        self._clear_person_detection()
        self.person_state     = STATE_UNKNOWN
        self.stationary_timer = 0.0
        self.candidate_state  = None
        self.candidate_timer  = 0.0

    def _publish(self, cmd: Twist) -> None:
        self.last_cmd_angular_z = cmd.angular.z
        self._cmd_pub.publish(cmd)

    def _log_state(self, extra: str = "") -> None:
        self.get_logger().info(
            f"[NAV={self.nav_state:<11} | PERSON={self.person_state:<12}] "
            f"vel={self.smoothed_velocity:+.3f}  "
            f"cand={str(self.candidate_state):<12}({self.candidate_timer:.2f}s)  "
            f"obs={self.stationary_timer:.2f}s  {extra}"
        )

    def _update_person_state(self, current_time) -> None:
        """Hysteresis state machine: UNKNOWN ↔ MOVING ↔ STATIONARY."""
        vel = abs(self.smoothed_velocity)

        in_cooldown = False
        if self.last_bypass_end_time is not None:
            elapsed = (current_time - self.last_bypass_end_time).nanoseconds / 1e9
            if elapsed < self.bypass_cooldown_time:
                in_cooldown = True

        if vel > self.moving_threshold:
            suggested = STATE_MOVING
        elif vel < self.stationary_threshold:
            suggested = STATE_UNKNOWN if in_cooldown else STATE_STATIONARY
        else:
            suggested = None   # in hysteresis band — hold current state

        if suggested is None or suggested == self.person_state:
            self.candidate_state = None
            self.candidate_timer = 0.0
            return

        if suggested != self.candidate_state:
            self.candidate_state = suggested
            self.candidate_timer = 0.0
        else:
            self.candidate_timer += self.current_dt

        confirm_needed = (
            self.moving_confirm_time
            if self.candidate_state == STATE_MOVING
            else self.stationary_confirm_time
        )

        if self.candidate_timer >= confirm_needed:
            prev = self.person_state
            self.person_state    = self.candidate_state
            self.candidate_state = None
            self.candidate_timer = 0.0

            if self.person_state == STATE_MOVING:
                self.stationary_timer = 0.0

            self.get_logger().info(
                f"Person state: {prev} → {self.person_state} "
                f"(vel={self.smoothed_velocity:+.3f})"
            )

    # ─────────────────────────────────────────────────────────────────────
    # Main control loop  (10 Hz)
    # ─────────────────────────────────────────────────────────────────────

    def _control_loop(self) -> None:
        cmd = Twist()

        current_time    = self.get_clock().now()
        self.current_dt = (current_time - self.last_loop_time).nanoseconds / 1e9
        self.last_loop_time = current_time

        # ── SECTION 1 · HARDWARE SAFETY ──────────────────────────────────
        # Advance bypass_timer BEFORE the early-return so a transient ToF
        # hit cannot freeze the S-curve state machine mid-maneuver (v11 fix 1).
        if self.person_state == STATE_BYPASSING:
            self.bypass_timer += self.current_dt

        active_stop_dist = (
            self.emergency_stop_dist * 0.5
            if self.person_state == STATE_BYPASSING
            else self.emergency_stop_dist
        )

        if 0.0 < self.current_distance < active_stop_dist:
            self.get_logger().warn(
                f"ToF EMERGENCY STOP — obstacle at {self.current_distance:.2f} m"
            )
            self._publish(Twist())
            return

        # ── SECTION 2 · COMMITTED BYPASS ─────────────────────────────────
        # A bypass maneuver must complete before marker tracking resumes.
        if self.person_state == STATE_BYPASSING:
            p1_end = self.bypass_curve_time
            p2_end = p1_end + self.bypass_straight_time
            p3_end = p2_end + self.bypass_curve_time

            cmd.linear.x = self.forward_speed

            if self.bypass_timer < p1_end:
                cmd.angular.z = self.bypass_direction * self.bypass_turn_speed
            elif self.bypass_timer < p2_end:
                cmd.angular.z = 0.0
            elif self.bypass_timer < p3_end:
                cmd.angular.z = -self.bypass_direction * self.bypass_turn_speed
            else:
                self.person_state         = STATE_UNKNOWN
                self.last_bypass_end_time = current_time
                self.get_logger().info(
                    "Bypass complete — resuming marker tracking."
                )
                cmd.angular.z = 0.0

            self.accumulated_yaw += cmd.angular.z * self.current_dt
            self._publish(cmd)
            return

        # ── SECTION 3 · PERSON DETECTION TIMEOUT ─────────────────────────
        # Only suppress person state when we are actively approaching.
        age_s = (current_time - self.last_detection_time).nanoseconds / 1e9
        if age_s > self.detection_timeout:
            # v11 fix 4: don't clobber STATIONARY or BYPASSING
            if self.person_state not in (STATE_STATIONARY, STATE_BYPASSING):
                self._clear_person_detection()

        # ── SECTION 4 · MARKER FRESHNESS → resolve nav_state ─────────────
        marker_age_s = (
            (current_time - self.last_marker_time).nanoseconds / 1e9
            if self.last_marker_time is not None
            else float("inf")
        )
        marker_fresh = marker_age_s < self.marker_timeout

        if not marker_fresh:
            # ── SEARCHING ─────────────────────────────────────────────────
            if self.nav_state != NAV_SEARCHING:
                self.get_logger().info(
                    "Marker lost — entering SEARCHING (spin mode)."
                )
                self._reset_person_state()
                self.accumulated_yaw = 0.0
            self.nav_state = NAV_SEARCHING

        else:
            cx_error = self.marker_cx - 0.5   # > 0: marker is right of centre
            if abs(cx_error) > self.center_threshold:
                # ── CENTERING ─────────────────────────────────────────────
                if self.nav_state != NAV_CENTERING:
                    self.get_logger().info(
                        f"Marker detected (cx={self.marker_cx:.3f}, "
                        f"area={self.marker_area:.4f}) — entering CENTERING."
                    )
                    self._reset_person_state()
                    self.accumulated_yaw = 0.0
                self.nav_state = NAV_CENTERING
            else:
                # ── APPROACHING ───────────────────────────────────────────
                if self.nav_state != NAV_APPROACHING:
                    self.get_logger().info(
                        f"Marker centred (cx={self.marker_cx:.3f}) — "
                        "entering APPROACHING."
                    )
                self.nav_state = NAV_APPROACHING

        # ── SECTION 5 · EXECUTE NAV STATE ────────────────────────────────

        # ·· 5a. SEARCHING — spin in place ·································
        if self.nav_state == NAV_SEARCHING:
            cmd.linear.x  = 0.0
            cmd.angular.z = self.search_spin_speed   # CCW
            self._publish(cmd)
            return

        # ·· 5b. CENTERING — proportional alignment, no forward motion ·····
        if self.nav_state == NAV_CENTERING:
            cx_error      = self.marker_cx - 0.5
            # Positive error (marker right) → negative angular.z (turn right/CW)
            raw_z         = -self.center_kp * cx_error
            cmd.angular.z = float(np.clip(raw_z,
                                          -self.center_turn_speed,
                                           self.center_turn_speed))
            cmd.linear.x  = 0.0
            self.get_logger().info(
                f"[CENTERING] cx={self.marker_cx:.3f}  "
                f"err={cx_error:+.3f}  ω={cmd.angular.z:+.3f} rad/s"
            )
            self._publish(cmd)
            return

        # ·· 5c. APPROACHING — drive toward marker + person avoidance ······
        #
        # The person sub-state machine runs here exactly as in v11.
        # When no person is obstructing, the robot drives forward with a soft
        # heading-hold correction to keep the marker centred.

        if self.target_cx is not None and self.target_area > self.area_threshold:
            # Person detected — run person sub-state machine
            # v11 fix 3: skip _update_person_state while STATIONARY
            if self.person_state != STATE_STATIONARY:
                self._update_person_state(current_time)

            if self.person_state == STATE_MOVING:
                cmd.linear.x = self.forward_speed
                if abs(self.smoothed_velocity) > self.stationary_threshold:
                    cmd.angular.z = (
                        self.turn_speed if self.smoothed_velocity > 0
                        else -self.turn_speed
                    )
                else:
                    cmd.angular.z = (
                        -self.turn_speed if self.target_cx < 0.5
                        else  self.turn_speed
                    )
                self.accumulated_yaw += cmd.angular.z * self.current_dt
                self._log_state()

            elif self.person_state == STATE_STATIONARY:
                cmd.linear.x  = 0.0
                cmd.angular.z = 0.0
                self.stationary_timer += self.current_dt

                if self.stationary_timer >= self.obs_time:
                    self.person_state     = STATE_BYPASSING
                    self.bypass_timer     = 0.0
                    self.bypass_direction = -1.0 if self.target_cx < 0.5 else 1.0
                    self.stationary_timer = 0.0
                    self._clear_person_detection()
                    side = "RIGHT" if self.bypass_direction == -1.0 else "LEFT"
                    self.get_logger().info(
                        f"Bypass triggered → {side} "
                        f"(stationary for {self.obs_time:.1f} s)"
                    )
                else:
                    self._log_state(
                        f"obs={self.stationary_timer:.1f}/{self.obs_time}s"
                    )

            else:  # UNKNOWN / post-bypass cooldown
                cmd.linear.x  = self.forward_speed
                cmd.angular.z = 0.0
                self._log_state("classifying / cooldown …")

        elif abs(self.accumulated_yaw) > 0.02:
            # Heading recovery after bypass turns
            self.stationary_timer = 0.0
            self.candidate_state  = None
            self.candidate_timer  = 0.0

            correction = (
                -self.bypass_turn_speed if self.accumulated_yaw > 0
                else  self.bypass_turn_speed
            )
            yaw_step = correction * self.current_dt

            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                cmd.angular.z        = 0.0
            else:
                cmd.angular.z        = correction
                self.accumulated_yaw += yaw_step

            cmd.linear.x = self.forward_speed

        else:
            # ── Clear path: drive forward, hold marker centred ────────────
            self.stationary_timer = 0.0
            self.accumulated_yaw  = 0.0

            # Soft heading-hold: proportional correction capped at half the
            # centering speed to avoid over-steering while moving.
            cx_error      = self.marker_cx - 0.5
            correction    = float(np.clip(
                -self.center_kp * cx_error,
                -self.center_turn_speed * 0.5,
                 self.center_turn_speed * 0.5,
            ))
            cmd.linear.x  = self.forward_speed
            cmd.angular.z = correction
            self.get_logger().debug(
                f"[APPROACHING] cx={self.marker_cx:.3f}  "
                f"hold_correction={correction:+.3f} rad/s"
            )

        self._publish(cmd)


# ─────────────────────────────────────────────────────────────────────────────

def main(args=None):
    rclpy.init(args=args)
    node = AvoidanceNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node._publish(Twist())   # zero velocity on clean exit
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()