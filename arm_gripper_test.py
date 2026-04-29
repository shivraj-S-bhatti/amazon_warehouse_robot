"""
Arm and Gripper test for DJI RoboMaster EP Core.

Sequence:
  1. Open gripper
  2. Lower arm (reach down)
  3. Close gripper (simulate grab)
  4. Raise arm (lift object)
  5. Open gripper (release)
  6. Return arm to starting position

Run after launching the ROS node:
  ros2 launch robomaster_ros main.launch model:=ep

Then in a separate terminal (with workspace sourced):
  python3 arm_gripper_test.py
"""

import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node

from robomaster_msgs.action import GripperControl, MoveArm


class ArmGripperTest(Node):

    def __init__(self):
        super().__init__('arm_gripper_test')
        self._gripper_client = ActionClient(self, GripperControl, 'gripper')
        self._arm_client = ActionClient(self, MoveArm, 'move_arm')

    # ---------- gripper helpers ----------

    def _send_gripper(self, state: int, power: float = 0.5) -> bool:
        """Send a gripper goal and block until it completes. Returns True on success."""
        self._gripper_client.wait_for_server()
        goal = GripperControl.Goal()
        goal.target_state = state
        goal.power = power

        state_name = {
            GripperControl.Goal.OPEN: 'OPEN',
            GripperControl.Goal.CLOSE: 'CLOSE',
            GripperControl.Goal.PAUSE: 'PAUSE',
        }.get(state, str(state))
        self.get_logger().info(f'Gripper -> {state_name} (power={power})')

        future = self._gripper_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future)
        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().error('Gripper goal rejected')
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        self.get_logger().info('Gripper done')
        return True

    # ---------- arm helpers ----------

    def _send_arm(self, x: float, z: float, relative: bool = True) -> bool:
        """
        Move arm end-effector.
          x: forward/backward in metres (positive = forward)
          z: up/down in metres         (positive = up)
          relative: True  -> relative to current position
                    False -> absolute from arm_base_link
        """
        self._arm_client.wait_for_server()
        goal = MoveArm.Goal()
        goal.x = x
        goal.z = z
        goal.relative = relative

        label = 'relative' if relative else 'absolute'
        self.get_logger().info(f'Arm move ({label}): x={x:.3f} m, z={z:.3f} m')

        future = self._arm_client.send_goal_async(goal)
        rclpy.spin_until_future_complete(self, future)
        goal_handle = future.result()

        if not goal_handle.accepted:
            self.get_logger().error('Arm goal rejected (another action may be running)')
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        self.get_logger().info('Arm move done')
        return True

    # ---------- main demo ----------

    def run(self):
        self.get_logger().info('=== Arm & Gripper Test Start ===')

        # Step 1: Open gripper before moving arm so nothing is squeezed
        self.get_logger().info('Step 1: Open gripper')
        self._send_gripper(GripperControl.Goal.OPEN, power=0.5)

        # Step 2: Move arm forward and down slightly (reach toward an object)
        self.get_logger().info('Step 2: Lower arm to grab position')
        self._send_arm(x=0.05, z=-0.04, relative=True)   # 5 cm forward, 4 cm down

        # Step 3: Close gripper (grab the object)
        self.get_logger().info('Step 3: Close gripper (grab)')
        self._send_gripper(GripperControl.Goal.CLOSE, power=0.7)

        # Step 4: Raise arm while holding
        self.get_logger().info('Step 4: Raise arm (lift object)')
        self._send_arm(x=0.0, z=0.06, relative=True)      # 6 cm up

        # Step 5: Release the object
        self.get_logger().info('Step 5: Open gripper (release)')
        self._send_gripper(GripperControl.Goal.OPEN, power=0.5)

        # Step 6: Return arm to roughly where it started
        self.get_logger().info('Step 6: Return arm to start position')
        self._send_arm(x=-0.05, z=-0.02, relative=True)   # undo the net displacement

        self.get_logger().info('=== Arm & Gripper Test Complete ===')


def main():
    rclpy.init()
    node = ArmGripperTest()
    try:
        node.run()
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
