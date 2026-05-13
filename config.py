"""
config.py — Project-wide constants and tunable parameters.

Adjust these values during testing on the real robot.
"""

# =============================================================================
# ArUco Marker IDs
# =============================================================================
# Print these from: https://chev.me/arucogen/
# Dictionary: 4x4_50, marker size: ~8cm recommended

OBJECT_MARKER_ID = 0            # Marker attached to the object at Point A
WAYPOINT_MARKER_IDS = [1, 2, 4]  # Markers along the route, in order
DEST_MARKER_ID = 10             # Marker at destination Point B

ARUCO_DICT_TYPE = "DICT_4X4_50"  # Small dictionary, fast detection
MARKER_SIZE = 0.08              # Physical marker side length in meters (8cm)

# =============================================================================
# Camera Parameters
# =============================================================================
# Approximate intrinsics for EP Core camera at 640x360 (360p)
# These are rough estimates — calibrate for better accuracy
# The EP Core has a 120° FOV lens

CAMERA_TOPIC = "camera/image_color"
CAMERA_WIDTH = 640
CAMERA_HEIGHT = 360

# Focal length estimate for 120° FOV at 640px width:
#   fx = (width / 2) / tan(FOV_horizontal / 2)
#   With 120° horizontal FOV: fx ≈ 640 / (2 * tan(60°)) ≈ 185
# These are approximate — adjust after testing
CAMERA_FX = 185.0
CAMERA_FY = 185.0
CAMERA_CX = 320.0  # principal point x (image center)
CAMERA_CY = 180.0  # principal point y (image center)

# =============================================================================
# State Machine States
# =============================================================================
STATE_SEARCH_OBJECT = "SEARCH_OBJECT"
STATE_APPROACH_OBJECT = "APPROACH_OBJECT"
STATE_PICK_UP = "PICK_UP"
STATE_NAVIGATE = "NAVIGATE"
STATE_OBSTACLE_AVOID = "OBSTACLE_AVOID"
STATE_BYPASS_OBSTACLE = "BYPASS_OBSTACLE"
STATE_SEARCH_DEST = "SEARCH_DEST"
STATE_APPROACH_DEST = "APPROACH_DEST"
STATE_PLACE = "PLACE"
STATE_DONE = "DONE"

# =============================================================================
# Approach & Navigation Thresholds
# =============================================================================
APPROACH_DISTANCE = 1.5        # meters — stop this far from object marker
WAYPOINT_SWITCH_DISTANCE = 1.50 # meters — close enough to switch to next waypoint
DEST_APPROACH_DISTANCE = 1.5   # meters — stop this far from destination marker
PERSON_BLOCK_AREA = 0.25        # min bounding-box area (fraction of frame) to trigger avoidance
PERSON_CENTER_BLOCK_HALF_WIDTH = 0.28  # central frame band treated as blocking the robot path
BLIND_APPROACH_THRESHOLD = 0.30
# If marker not detected for this many seconds, rotate to search
MARKER_LOST_TIMEOUT = 3.0       # seconds
REACQUIRE_TURN_TIME = 2.0       # seconds to bias search toward last-seen marker side

# If a waypoint marker can't be found after rotating for this long, skip it
WAYPOINT_SEARCH_TIMEOUT = 15.0  # seconds

# =============================================================================
# Obstacle Avoidance (ToF Sensor)
# =============================================================================
TOF_TOPIC = "range_0"                # forward-facing ToF topic
TOF_LEFT_TOPIC  = "range_1"         # left side ToF — verify with: ros2 topic list
TOF_RIGHT_TOPIC = "range_2"         # right side ToF — verify with: ros2 topic list
OBSTACLE_THRESHOLD = 0.40           # meters — trigger avoidance
OBSTACLE_CLEAR_THRESHOLD = 0.60     # meters — safe to resume
# During the U-detour forward leg, the side TOF on the obstacle side reads
# short while the obstacle is beside the robot.  When it jumps above this
# threshold the robot has moved past the obstacle and can strafe back.
DETOUR_SIDE_CLEAR_THRESHOLD = 1.0   # meters
STRAFE_DIRECTION = 1.0              # +1.0 = strafe left, -1.0 = strafe right
STRAFE_DURATION = 1.5               # seconds to strafe before rechecking
# When a marker is lost, if something is within this distance on TOF the robot
# treats the obstacle as the cause and initiates a U-detour instead of spinning.
# Larger than OBSTACLE_THRESHOLD so we catch obstacles that block the camera
# before the robot gets close enough to trigger the normal TOF avoidance.
MARKER_BLOCKED_THRESHOLD = 1.5      # meters

# =============================================================================
# Chassis Speeds
# =============================================================================
SEARCH_ROTATE_SPEED = 0.3           # rad/s — rotation while searching for marker
APPROACH_MAX_SPEED = 0.25           # m/s — max forward speed during approach
APPROACH_MIN_SPEED = 0.08           # m/s — min forward speed (don't stall)
STRAFE_SPEED = 0.15                 # m/s — sideways speed during avoidance
NAVIGATE_SPEED = 0.25               # m/s — forward speed between waypoints

# =============================================================================
# Visual Servoing Gains
# =============================================================================
# Proportional gain for steering toward marker center
STEER_KP = 0.45                     # angular.z = KP * lateral_error
STRAFE_KP = 0.3
# Proportional gain for speed based on distance
SPEED_KP = 0.5                     # linear.x = KP * distance (clamped)

# Lateral error dead zone — don't steer if error is smaller than this
STEER_DEADZONE = 0.05              # meters

# =============================================================================
# Arm Positions (meters, relative to arm_base_link)
# =============================================================================
# These need tuning on the real robot!
ARM_PICK_X = 0.15                  # forward reach for picking
ARM_PICK_Z = 0.10             # absolute height floor (hardware limit)
ARM_PICK_LOWER = 0.10           # relative drop from floor to reach object (negative = down)
ARM_CARRY_X = 0.09                 # forward position while carrying
ARM_CARRY_Z = 0.07                 # lifted height while carrying (match retract to keep camera level)
ARM_PLACE_X = 0.18                 # forward reach for placing
ARM_PLACE_Z = 0.05                 # height for placing
ARM_RETRACT_X = 0.09               # retracted position
ARM_RETRACT_Z = 0.10               # retracted height (lowered for better camera angle)

# =============================================================================
# Gripper
# =============================================================================
GRIPPER_OPEN = 1                   # GripperControl target_state for open
GRIPPER_CLOSE = 2                  # GripperControl target_state for close
GRIPPER_PAUSE = 0                  # GripperControl target_state for pause
GRIPPER_POWER = 0.5                # Grip force [0.0 - 1.0]

# =============================================================================
# LED Colors (RGBA, values 0.0 to 1.0)
# =============================================================================
LED_SEARCHING = (0.0, 0.0, 1.0, 1.0)    # Blue — searching
LED_APPROACHING = (1.0, 1.0, 0.0, 1.0)  # Yellow — approaching
LED_PICKING = (1.0, 0.5, 0.0, 1.0)      # Orange — picking/placing
LED_NAVIGATING = (0.0, 1.0, 1.0, 1.0)   # Cyan — navigating
LED_AVOIDING = (1.0, 0.0, 0.0, 1.0)     # Red — obstacle avoidance
LED_DONE = (0.0, 1.0, 0.0, 1.0)         # Green — task complete

# =============================================================================
# Timing
# =============================================================================
CONTROL_LOOP_RATE = 10              # Hz — main control loop frequency
GRIPPER_WAIT_TIME = 1.5             # seconds — wait after gripper command
ARM_WAIT_TIME = 1.0                 # seconds — wait after arm movement

# =============================================================================
# People Bypass (S-curve maneuver around a stationary person)
# =============================================================================
BYPASS_FORWARD_SPEED = 0.2          # m/s during all bypass phases
BYPASS_TURN_SPEED    = 0.8          # rad/s during S-curve bypass phases
BYPASS_CURVE_TIME    = 1.5          # seconds for each curve phase
BYPASS_STRAIGHT_TIME = 2.5          # seconds for the straight phase

# Dynamic (moving-person) avoidance turn speed — gentler than bypass S-curve
AVOID_TURN_SPEED     = 0.4          # rad/s while sidestepping a moving person

# =============================================================================
# U-Shape Detour (around a stationary obstacle — person or non-person)
# =============================================================================
# Pure-translation maneuver — no rotation, so the robot heading stays locked
# to the original line of travel. When the detour completes the robot has
# rejoined the same heading line at a point further along, where the marker
# should be re-acquired without any spin-search.
#
#   Phase 1 — STRAFE_OUT:  slide laterally for DETOUR_STRAFE_DURATION
#                          (lateral offset ≈ speed × duration).
#   Phase 2 — FORWARD:     drive forward for DETOUR_FORWARD_DURATION
#                          to pass the obstacle's depth.
#   Phase 3 — STRAFE_BACK: slide back the same offset to rejoin the line.
DETOUR_STRAFE_SPEED     = 0.18      # m/s lateral speed during strafe legs
DETOUR_STRAFE_DURATION  = 1.8       # s   → ~0.32 m lateral offset
DETOUR_FORWARD_SPEED    = 0.20      # m/s forward speed during middle leg
# Safety timeout for the forward leg.  In practice the robot exits Phase 2
# early once the obstacle is cleared (marker visible or person at frame edge).
# This value is the hard upper bound so the robot doesn't drive forever if
# neither sensor signal fires.
DETOUR_FORWARD_DURATION = 5.0       # s   (was 2.5 — dynamic exit now handles early stop)
DETOUR_DEFAULT_DIR      = 1.0       # +1 = detour LEFT, -1 = detour RIGHT
                                    # (used when obstacle side is unknown,
                                    #  e.g. a ToF-only detection)
# =============================================================================
# Recentering — two-phase yaw → strafe alignment before advancing
# =============================================================================
# Phase 1 — YAW:
#   If horizontal_error > RECENTER_YAW_THRESHOLD the robot is too far off-axis
#   to strafe efficiently.  Rotate slowly in-place until the marker is roughly
#   centred (robot is now squarely facing the marker plane).
RECENTER_YAW_THRESHOLD = 0.20      # horizontal_error (tx/tz) — ~11° off-axis
RECENTER_YAW_SPEED     = 0.12      # rad/s — slow enough not to overshoot

# Phase 2 — STRAFE:
#   Once roughly centred by yaw, slide laterally to fine-centre the marker.
#   No rotation → marker stays in frame the whole time.
#   Strafe exits to SETTLED when error drops below RECENTER_STRAFE_THRESHOLD.
RECENTER_STRAFE_THRESHOLD = 0.08   # horizontal_error (tx/tz) — ~5° residual
RECENTER_STRAFE_SPEED     = 0.08   # m/s

# Phase 3 — SETTLED:
#   Must hold within STRAFE_THRESHOLD for this many consecutive frames before
#   resuming forward motion.  Lets the EMA smoother confirm true alignment.
RECENTER_SETTLED_COUNT = 3         # frames
