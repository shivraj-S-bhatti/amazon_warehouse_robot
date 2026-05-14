"""
arm_control.py — Robotic arm and gripper control.

Uses ROS 2 actions to move the arm and open/close the gripper.
Provides high-level functions for pick-up and place-down sequences.

Usage:
    from arm_control import ArmController
    arm = ArmController()

    arm.open_gripper()
    arm.move_arm_to(0.18, -0.04, relative=False)
    arm.close_gripper()
    arm.move_arm_to(0.12, 0.08, relative=False)
"""

import rclpy
from rclpy.node import Node
from rclpy.action import ActionClient
from robomaster_msgs.action import MoveArm, GripperControl, RecenterGimbal
import time
import threading

from config import (
    ARM_PICK_X,
    ARM_PICK_Z,
    ARM_CARRY_X,
    ARM_CARRY_Z,
    ARM_PLACE_X,
    ARM_PLACE_Z,
    ARM_RETRACT_X,
    ARM_RETRACT_Z,
    GRIPPER_OPEN,
    GRIPPER_CLOSE,
    GRIPPER_PAUSE,
    GRIPPER_POWER,
    GRIPPER_WAIT_TIME,
    ARM_WAIT_TIME,
)


class ArmController(Node):
    """
    ROS 2 node for controlling the EP Core's arm and gripper.

    Uses action clients to send goals and wait for completion.
    All public methods are BLOCKING — they wait until the action finishes.
    """

    def __init__(self):
        super().__init__("arm_controller")

        # Action clients
        self.arm_client = ActionClient(self, MoveArm, "move_arm")
        self.gripper_client = ActionClient(self, GripperControl, "gripper")
        self.gimbal_client = ActionClient(self, RecenterGimbal, "recenter_gimbal")

        # Track state
        self.lock = threading.Lock()
        self._gripper_state = None  # 0=PAUSE, 1=OPEN, 2=CLOSE
        self._is_carrying = False

        self.get_logger().info("ArmController initialized")

    # ─────────────────────────────────────────────────────────────
    # Wait for action servers to come online
    # ─────────────────────────────────────────────────────────────

    def wait_for_servers(self, timeout_sec=10.0):
        """
        Wait for arm and gripper action servers to be available.
        Call this once after launching the driver.

        Returns:
            bool: True if both servers are ready.
        """
        self.get_logger().info("Waiting for arm action server...")
        arm_ready = self.arm_client.wait_for_server(timeout_sec=timeout_sec)
        if not arm_ready:
            self.get_logger().error("Arm action server not available!")
            return False

        self.get_logger().info("Waiting for gripper action server...")
        grip_ready = self.gripper_client.wait_for_server(timeout_sec=timeout_sec)
        if not grip_ready:
            self.get_logger().error("Gripper action server not available!")
            return False

        self.get_logger().info("Arm and gripper action servers ready!")
        return True

    # ─────────────────────────────────────────────────────────────
    # Low-level: Arm movement
    # ─────────────────────────────────────────────────────────────

    def move_arm_to(self, x, z, relative=False):
        """
        Move the arm end-effector to a target position.
        BLOCKS until the arm reaches the target or times out.

        Args:
            x (float): Forward distance in meters from arm base.
            z (float): Height in meters from arm base.
            relative (bool): If True, x and z are relative to current position.

        Returns:
            bool: True if the arm reached the target.
        """
        goal = MoveArm.Goal()
        goal.x = float(x)
        goal.z = float(z)
        goal.relative = relative

        mode = "relative" if relative else "absolute"
        self.get_logger().info(f"Moving arm ({mode}): x={x:.3f}, z={z:.3f}")

        # Send goal
        future = self.arm_client.send_goal_async(goal)

        # Wait for goal acceptance
        rclpy.spin_until_future_complete(self, future, timeout_sec=5.0)
        goal_handle = future.result()

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().warn("Arm goal rejected!")
            return False

        # Wait for result
        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future, timeout_sec=10.0)

        if not result_future.done():
            self.get_logger().warn("Arm movement timed out!")
            return False

        # Small delay for arm to settle
        time.sleep(ARM_WAIT_TIME)

        self.get_logger().info("Arm movement complete")
        return True

    # ─────────────────────────────────────────────────────────────
    # Low-level: Gripper control
    # ─────────────────────────────────────────────────────────────

    def _send_gripper_goal(self, target_state, power=None):
        """
        Send a gripper command and wait for completion.

        Args:
            target_state (int): 0=PAUSE, 1=OPEN, 2=CLOSE
            power (float): Grip force [0.0, 1.0]. Default: GRIPPER_POWER.

        Returns:
            bool: True if gripper reached the target state.
        """
        if power is None:
            power = GRIPPER_POWER

        names = {0: "PAUSE", 1: "OPEN", 2: "CLOSE"}
        self.get_logger().info(
            f'Gripper: {names.get(target_state, "?")} (power={power})'
        )

        goal = GripperControl.Goal()
        goal.target_state = target_state
        goal.power = float(power)

        # Send goal
        future = self.gripper_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future, timeout_sec=5.0)
        goal_handle = future.result()

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().warn("Gripper goal rejected!")
            return False

        # Wait for result
        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future, timeout_sec=10.0)

        if not result_future.done():
            self.get_logger().warn("Gripper action timed out!")
            return False

        # Wait for gripper to finish mechanical movement
        time.sleep(GRIPPER_WAIT_TIME)

        with self.lock:
            self._gripper_state = target_state

        self.get_logger().info("Gripper action complete")
        return True

    def open_gripper(self, power=None):
        """Open the gripper. BLOCKING."""
        return self._send_gripper_goal(GRIPPER_OPEN, power)

    def close_gripper(self, power=None):
        """Close the gripper. BLOCKING."""
        return self._send_gripper_goal(GRIPPER_CLOSE, power)

    def pause_gripper(self):
        """
        Pause the gripper — hold current position.
        Useful after closing on an object to prevent further closing
        if the object is removed.
        """
        return self._send_gripper_goal(GRIPPER_PAUSE)

    # ─────────────────────────────────────────────────────────────
    # High-level: Pick up sequence
    # ─────────────────────────────────────────────────────────────

    def pick_up(self):
        """
        Execute the full pick-up sequence:
        1. Open gripper
        2. Lower arm to pick position
        3. Close gripper
        4. Pause gripper (hold position)
        5. Lift arm to carry position

        Returns:
            bool: True if all steps succeeded.
        """
        self.get_logger().info("=== PICK UP SEQUENCE START ===")

        # Step 1: Open gripper
        if not self.open_gripper():
            self.get_logger().error("Failed to open gripper")
            return False

        # Step 2: Lower arm to pick position
        if not self.move_arm_to(ARM_PICK_X, ARM_PICK_Z, relative=False):
            self.get_logger().error("Failed to lower arm")
            return False

        # Step 3: Close gripper to grab object
        if not self.close_gripper():
            self.get_logger().error("Failed to close gripper")
            return False

        # Step 4: Pause gripper to hold position
        if not self.pause_gripper():
            self.get_logger().error("Failed to pause gripper — object may not be held")
            return False

        # Step 5: Lift arm to carry position
        if not self.move_arm_to(ARM_CARRY_X, ARM_CARRY_Z, relative=False):
            self.get_logger().error("Failed to lift arm")
            return False

        with self.lock:
            self._is_carrying = True

        self.get_logger().info("=== PICK UP SEQUENCE COMPLETE ===")
        return True

    # ─────────────────────────────────────────────────────────────
    # High-level: Place down sequence
    # ─────────────────────────────────────────────────────────────

    def place_down(self):
        """
        Execute the full place-down sequence:
        1. Lower arm to place position
        2. Open gripper
        3. Retract arm

        Returns:
            bool: True if all steps succeeded.
        """
        self.get_logger().info("=== PLACE DOWN SEQUENCE START ===")

        # Step 1: Lower arm to place position
        if not self.move_arm_to(ARM_PLACE_X, ARM_PLACE_Z, relative=False):
            self.get_logger().error("Failed to lower arm")
            return False

        # Step 2: Open gripper to release object
        if not self.open_gripper():
            self.get_logger().error("Failed to open gripper")
            return False

        # Step 3: Retract arm
        if not self.move_arm_to(ARM_RETRACT_X, ARM_RETRACT_Z, relative=False):
            self.get_logger().error("Failed to retract arm")
            return False

        with self.lock:
            self._is_carrying = False

        self.get_logger().info("=== PLACE DOWN SEQUENCE COMPLETE ===")
        return True

    # ─────────────────────────────────────────────────────────────
    # High-level: Arm positioning helpers
    # ─────────────────────────────────────────────────────────────

    def recenter_gimbal(self):
        """
        Level the camera by recentering the gimbal to 0° pitch/yaw.
        Silently skips if the gimbal action server is not available
        (requires gimbal:=True in the launch command).

        Returns:
            bool: True if gimbal was recentered, False if unavailable.
        """
        if not self.gimbal_client.wait_for_server(timeout_sec=2.0):
            self.get_logger().warn(
                "Gimbal server not available — add gimbal:=True to launch command"
            )
            return False

        goal = RecenterGimbal.Goal()
        goal.pitch_speed = 1.04
        goal.yaw_speed = 1.04

        future = self.gimbal_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future, timeout_sec=5.0)
        goal_handle = future.result()

        if goal_handle is None or not goal_handle.accepted:
            self.get_logger().warn("Gimbal recenter goal rejected")
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future, timeout_sec=10.0)

        self.get_logger().info("Gimbal recentered — camera now level")
        return True

    def carry_position(self):
        """Move arm to the carry position (lifted, out of camera view)."""
        return self.move_arm_to(ARM_CARRY_X, ARM_CARRY_Z, relative=False)

    def retract(self):
        """Retract the arm fully."""
        return self.move_arm_to(ARM_RETRACT_X, ARM_RETRACT_Z, relative=False)

    def is_carrying(self):
        """Check if the robot is currently carrying an object."""
        with self.lock:
            return self._is_carrying


# =============================================================================
# Standalone test
# =============================================================================
def main():
    """
    Test arm and gripper standalone.
    The arm will: open gripper → lower → close → lift → lower → open → retract.

    WARNING: Arm will move! Clear the area around the robot.

    Usage:
        # Terminal 1: launch driver with arm and gripper enabled
        # Terminal 2:
        cd ~/robomaster_ws/my_project
        python3 arm_control.py
    """
    rclpy.init()
    node = ArmController()

    try:
        if not node.wait_for_servers():
            node.get_logger().error("Action servers not available — exiting")
            return

        node.get_logger().info("Starting arm test in 3 seconds...")
        time.sleep(3)

        # Test pick sequence
        node.get_logger().info("Testing pick-up sequence...")
        node.pick_up()

        time.sleep(2)

        # Test place sequence
        node.get_logger().info("Testing place-down sequence...")
        node.place_down()

        node.get_logger().info("Arm test complete!")

    except KeyboardInterrupt:
        pass
    finally:
        node.retract()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == "__main__":
    main()
