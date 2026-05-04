#!/usr/bin/env python3
"""
RoboMaster Avoidance Node — v11 (Bypass Timer + Stationary Lock Fixes)
=======================================================================

Fixes vs v10:
  1. bypass_timer now advances BEFORE the emergency-stop early-return, so a
     momentary ToF hit can no longer freeze the S-curve state machine mid-turn.
  2. _people_cb now returns early for STATE_STATIONARY (but still refreshes
     last_detection_time so the detection-timeout clock doesn't reset the state).
  3. _control_loop skips _update_person_state() while STATE_STATIONARY, locking
     the state until the observation window expires and bypass is triggered.
  4. Detection-timeout _clear_detection() is suppressed while STATE_STATIONARY
     or STATE_BYPASSING so neither state can be silently clobbered.
"""

from std_msgs.msg import String, Bool, Int32  # Added Bool and Int32
from robomaster_msgs.msg import Detection

import numpy as np
from collections import deque

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy

from geometry_msgs.msg import Twist
from sensor_msgs.msg import Range, Imu
from robomaster_msgs.msg import Detection

SENSOR_QOS = QoSProfile(
    reliability=ReliabilityPolicy.BEST_EFFORT,
    history=HistoryPolicy.KEEP_LAST,
    depth=10,
)

STATE_UNKNOWN    = "UNKNOWN"
STATE_MOVING     = "MOVING"
STATE_STATIONARY = "STATIONARY"
STATE_BYPASSING  = "BYPASSING"

class AvoidanceNode(Node):
    def __init__(self):
        super().__init__("robomaster_avoidance_node")

        self.declare_parameter("forward_speed",            0.2)
        self.declare_parameter("turn_speed",               0.4)
        self.declare_parameter("reaction_area_threshold",  0.05)
        self.declare_parameter("detection_timeout",        0.5)
        self.declare_parameter("emergency_stop_dist",      0.6)

        self.declare_parameter("vel_window_size",          5)
        self.declare_parameter("vel_ema_alpha",            0.4)
        self.declare_parameter("moving_threshold",         0.15)
        self.declare_parameter("stationary_threshold",     0.06)
        self.declare_parameter("moving_confirm_time",      0.2)
        self.declare_parameter("stationary_confirm_time",  0.5)
        self.declare_parameter("stationary_observation_time", 1.2)

        self.declare_parameter("bypass_turn_speed",        0.8)
        self.declare_parameter("bypass_curve_time",        1.5)
        self.declare_parameter("bypass_straight_time",     2.5)
        self.declare_parameter("post_bypass_cooldown",     2.5)

        self.declare_parameter("camera_hfov_deg",          80.0)
        self.declare_parameter("use_cmd_vel_compensation", True)

        self.forward_speed           = self.get_parameter("forward_speed").value
        self.turn_speed              = self.get_parameter("turn_speed").value
        self.bypass_turn_speed       = self.get_parameter("bypass_turn_speed").value
        self.area_threshold          = self.get_parameter("reaction_area_threshold").value
        self.detection_timeout       = self.get_parameter("detection_timeout").value
        self.emergency_stop_dist     = self.get_parameter("emergency_stop_dist").value

        self.vel_window_size         = self.get_parameter("vel_window_size").value
        self.vel_ema_alpha           = self.get_parameter("vel_ema_alpha").value
        self.moving_threshold        = self.get_parameter("moving_threshold").value
        self.stationary_threshold    = self.get_parameter("stationary_threshold").value
        self.moving_confirm_time     = self.get_parameter("moving_confirm_time").value
        self.stationary_confirm_time = self.get_parameter("stationary_confirm_time").value

        self.obs_time                = self.get_parameter("stationary_observation_time").value
        self.bypass_curve_time       = self.get_parameter("bypass_curve_time").value
        self.bypass_straight_time    = self.get_parameter("bypass_straight_time").value
        self.bypass_cooldown_time    = self.get_parameter("post_bypass_cooldown").value

        hfov_deg                     = self.get_parameter("camera_hfov_deg").value
        self.hfov_rad                = np.radians(hfov_deg)
        self.use_cmd_compensation    = self.get_parameter("use_cmd_vel_compensation").value

        # State variables
        self.current_distance        = float("inf")
        self.target_cx               = None
        self.target_area             = 0.0
        self.cx_history              = deque(maxlen=self.vel_window_size)
        self.smoothed_velocity       = 0.0
        self.last_detection_time     = self.get_clock().now()
        self.robot_yaw_rate          = 0.0
        self.last_cmd_angular_z      = 0.0

        self.person_state            = STATE_UNKNOWN
        self.candidate_state         = None
        self.candidate_timer         = 0.0
        self.stationary_timer        = 0.0

        self.bypass_timer            = 0.0
        self.bypass_direction        = 1.0
        self.last_bypass_end_time    = None
        self.accumulated_yaw         = 0.0

        self.last_loop_time          = self.get_clock().now()
        self.current_dt              = 0.1

        self.is_enabled = False
        self.target_marker_id = 0
        
        self.create_subscription(Bool, "/smart_movement/enable", self._enable_cb, 10)
        self.create_subscription(Int32, "/smart_movement/target_id", self._target_cb, 10)

        self.create_subscription(Range,     "/range_0",         self._tof_cb,    SENSOR_QOS)
        self.create_subscription(Imu,       "/imu",             self._imu_cb,    SENSOR_QOS)
        self.create_subscription(Detection, "/robomaster_detection_node/markers", self._marker_cb, SENSOR_QOS)

        self._cmd_pub = self.create_publisher(Twist, "/cmd_vel", 10)
        self.create_timer(0.1, self._control_loop)

        self.get_logger().info("Avoidance node v11 (Bypass Timer + Stationary Lock Fixes) started.")

    # ── Sensor callbacks ──────────────────────────────────────────────────

    def _tof_cb(self, msg: Range):
        dist = msg.range
        if dist > 10.0:
            dist = dist / 1000.0
        self.current_distance = dist if dist > 0.0 else float("inf")

    def _imu_cb(self, msg: Imu):
        self.robot_yaw_rate = msg.angular_velocity.z

    def _enable_cb(self, msg: Bool):
        self.is_enabled = msg.data
        if not self.is_enabled:
            self._clear_detection()

    def _target_cb(self, msg: Int32):
        self.target_marker_id = msg.data

    def _marker_cb(self, msg: Detection):
        # Stop processing if main.py turned us off
        if not self.is_enabled or self.person_state == STATE_BYPASSING:
            return

        self.last_detection_time = self.get_clock().now()
        if self.person_state == STATE_STATIONARY:
            return

        if not msg.markers:
            self._clear_detection()
            return

        best_cx, max_area = 0.5, 0.0
        found_target = False

        for marker in msg.markers:
            # Filter: Only track the specific ID main.py asked for
            try:
                if int(marker.kind) != self.target_marker_id:
                    continue
            except ValueError:
                continue

            found_target = True
            cx   = marker.roi.x_offset + marker.roi.width / 2.0
            area = marker.roi.width * marker.roi.height
            if area > max_area:
                max_area = area
                best_cx  = cx

        if not found_target:
            self._clear_detection()
            return

        self.target_cx   = best_cx
        self.target_area = max_area

        now_s = self.get_clock().now().nanoseconds / 1e9
        self.cx_history.append((now_s, best_cx))
        self._update_velocity()

    # ── Internal helpers ──────────────────────────────────────────────────

    def _update_velocity(self):
        if len(self.cx_history) < 2:
            self.smoothed_velocity = 0.0
            return

        t_old, cx_old = self.cx_history[0]
        t_new, cx_new = self.cx_history[-1]
        dt = t_new - t_old
        if dt <= 0.0:
            return

        raw_vel      = (cx_new - cx_old) / dt
        yaw_rate     = (self.robot_yaw_rate if abs(self.robot_yaw_rate) > 0.001 else self.last_cmd_angular_z)
        compensated  = raw_vel - yaw_rate / self.hfov_rad
        self.smoothed_velocity = (self.vel_ema_alpha * compensated) + ((1.0 - self.vel_ema_alpha) * self.smoothed_velocity)

    def _clear_detection(self):
        self.target_cx         = None
        self.target_area       = 0.0
        self.cx_history.clear()
        self.smoothed_velocity = 0.0
        self.candidate_state   = None
        self.candidate_timer   = 0.0
        self.stationary_timer  = 0.0

        if self.person_state not in (STATE_BYPASSING, STATE_STATIONARY):
            self.person_state = STATE_UNKNOWN

    def _publish(self, cmd: Twist):
        self.last_cmd_angular_z = cmd.angular.z
        self._cmd_pub.publish(cmd)

    def _log_state(self, extra: str = ""):
        self.get_logger().info(
            f"[{self.person_state:<12}] "
            f"vel={self.smoothed_velocity:+.3f}  "
            f"cand={str(self.candidate_state):<12}({self.candidate_timer:.2f}s)  "
            f"obs={self.stationary_timer:.2f}s  {extra}"
        )

    def _update_person_state(self, current_time):
        vel = abs(self.smoothed_velocity)

        in_cooldown = False
        if self.last_bypass_end_time is not None:
            time_since_bypass = (current_time - self.last_bypass_end_time).nanoseconds / 1e9
            if time_since_bypass < self.bypass_cooldown_time:
                in_cooldown = True

        if vel > self.moving_threshold:
            suggested = STATE_MOVING
        elif vel < self.stationary_threshold:
            suggested = STATE_UNKNOWN if in_cooldown else STATE_STATIONARY
        else:
            suggested = None

        if suggested is None or suggested == self.person_state:
            self.candidate_state = None
            self.candidate_timer = 0.0
            return

        if suggested != self.candidate_state:
            self.candidate_state = suggested
            self.candidate_timer = 0.0
        else:
            self.candidate_timer += self.current_dt

        confirm_needed = (self.moving_confirm_time if self.candidate_state == STATE_MOVING else self.stationary_confirm_time)

        if self.candidate_timer >= confirm_needed:
            prev = self.person_state
            self.person_state    = self.candidate_state
            self.candidate_state = None
            self.candidate_timer = 0.0

            if self.person_state == STATE_MOVING:
                self.stationary_timer = 0.0

            self.get_logger().info(f"State committed: {prev} -> {self.person_state} (vel={self.smoothed_velocity:+.3f})")

    # ── Main control loop ─────────────────────────────────────────────────

    def _control_loop(self):

        if not self.is_enabled:
            return
        
        cmd = Twist()

        current_time    = self.get_clock().now()
        self.current_dt = (current_time - self.last_loop_time).nanoseconds / 1e9
        self.last_loop_time = current_time

        # FIX 4 — Don't let detection-timeout clobber STATIONARY or BYPASSING.
        age_s = (current_time - self.last_detection_time).nanoseconds / 1e9
        if age_s > self.detection_timeout:
            if self.person_state not in (STATE_STATIONARY, STATE_BYPASSING):
                self._clear_detection()

        # ── 1. HARDWARE SAFETY ────────────────────────────────────────────
        # FIX 1 — Advance bypass_timer BEFORE the early-return so a transient
        # ToF hit cannot freeze the S-curve state machine mid-maneuver.
        if self.person_state == STATE_BYPASSING:
            self.bypass_timer += self.current_dt

        active_stop_dist = (self.emergency_stop_dist * 0.5
                            if self.person_state == STATE_BYPASSING
                            else self.emergency_stop_dist)

        if 0.0 < self.current_distance < active_stop_dist:
            self.get_logger().warn(f"ToF EMERGENCY STOP — obstacle at {self.current_distance:.2f} m")
            self._publish(Twist())
            return

        # ── 2. COMMITTED BYPASS ───────────────────────────────────────────
        if self.person_state == STATE_BYPASSING:
            # bypass_timer was already incremented above in section 1.
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
                self.person_state     = STATE_UNKNOWN
                self.last_bypass_end_time = current_time
                self.get_logger().info("Bypass complete — returning to navigation & initiating cooldown.")
                cmd.angular.z = 0.0

            self.accumulated_yaw += cmd.angular.z * self.current_dt
            self._publish(cmd)
            return

        # ── 3. VISION LAYER ───────────────────────────────────────────────
        if self.target_cx is not None and self.target_area > self.area_threshold:

            # FIX 3 — While STATIONARY, skip _update_person_state entirely so
            # no incoming detection data can flip us back to MOVING mid-observation.
            if self.person_state != STATE_STATIONARY:
                self._update_person_state(current_time)

            if self.person_state == STATE_MOVING:
                cmd.linear.x = self.forward_speed
                if abs(self.smoothed_velocity) > self.stationary_threshold:
                    cmd.angular.z = self.turn_speed if self.smoothed_velocity > 0 else -self.turn_speed
                else:
                    cmd.angular.z = -self.turn_speed if self.target_cx < 0.5 else self.turn_speed

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

                    self._clear_detection()

                    side = "RIGHT" if self.bypass_direction == -1.0 else "LEFT"
                    self.get_logger().info(f"Bypass triggered -> {side} (observed stationary for {self.obs_time}s)")
                else:
                    self._log_state(f"obs={self.stationary_timer:.1f}/{self.obs_time}s")

            else:  # UNKNOWN / cooldown
                cmd.linear.x  = self.forward_speed
                cmd.angular.z = 0.0
                self._log_state("classifying / cooldown active...")

        # ── 4. HEADING RECOVERY ───────────────────────────────────────────
        elif abs(self.accumulated_yaw) > 0.02:
            self.stationary_timer = 0.0
            self.candidate_state  = None
            self.candidate_timer  = 0.0

            correction = -self.bypass_turn_speed if self.accumulated_yaw > 0 else self.bypass_turn_speed
            yaw_step   = correction * self.current_dt

            if abs(self.accumulated_yaw) <= abs(yaw_step):
                self.accumulated_yaw = 0.0
                cmd.angular.z        = 0.0
            else:
                cmd.angular.z        = correction
                self.accumulated_yaw += yaw_step

            cmd.linear.x = self.forward_speed

        # ── 5. CLEAR PATH ─────────────────────────────────────────────────
        else:
            self.stationary_timer = 0.0
            self.accumulated_yaw  = 0.0
            cmd.linear.x          = self.forward_speed
            cmd.angular.z         = 0.0

        self._publish(cmd)


def main(args=None):
    rclpy.init(args=args)
    node = AvoidanceNode()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node._publish(Twist())
    finally:
        node.destroy_node()
        rclpy.shutdown()

if __name__ == "__main__":
    main()