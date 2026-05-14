# Plan / TODOs

## Goal
Make marker centering use **strafe + yaw together** (mecanum advantage) so the robot keeps the ArUco in view and converges faster, while also reducing annoying shutdown/import flakiness observed in the terminal logs.

## Todo
- [ ] 1) Confirm current marker error signals
  - Check what fields `VisionNode.get_marker()` provides (we¡¯ve seen `lateral`, and `chassis_control.py` expects `horizontal_error`).
- [x] 2) Implement yaw+strafe centering in `StateMachine.handle_approach_object()`
  - When lateral error is outside deadzone, command `cmd_vel` with:
    - `linear_y` set by sign of lateral (strafe toward center)
    - `angular_z` proportional to lateral (gentle yaw toward marker)
    - `linear_x = 0` while centering (don¡¯t drive forward until centered)
  - Clamp `angular_z` to a safe small max (avoid spin).
- [ ] 3) Fix shutdown to be idempotent (avoid `InvalidHandle` on Ctrl+C)
  - Ensure `shutdown()` can be called multiple times safely.
  - Avoid publishing after node destruction; wrap final `stop()` in try/except.
  - Avoid double-calling `shutdown()` from both signal handler and `finally`.
- [ ] 4) Re-run `python3 main.py` and verify
  - Logs show both strafing and yawing during centering.
  - Ctrl+C exits without traceback.

## Review (fill in after)
- Changes made: While approaching the object marker, if `abs(lateral) > STEER_DEADZONE` the robot now commands **strafe + yaw simultaneously** with `linear_x=0.0` until centered.
- Files touched: `src/robomaster_node/main.py`
- How to test: Run the driver + `python3 main.py`, then present marker off-center; you should see the ¡°Centering marker (strafe+yaw)¡± log and observe yaw + lateral motion.
- Notes / follow-ups: Ctrl+C shutdown tracebacks and intermittent `ArmController` import errors are observed in logs but not addressed in this change (kept scope minimal).
