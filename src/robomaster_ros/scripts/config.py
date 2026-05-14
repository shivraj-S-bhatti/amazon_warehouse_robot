"""
config.py — Project-wide constants and tunable parameters.

Adjust these values during testing on the real robot.
"""

# =============================================================================
# ArUco Marker IDs
# =============================================================================
# Print these from: https://chev.me/arucogen/
# Dictionary: 4x4_50, marker size: ~8cm recommended

OBJECT_MARKER_ID = 0  # Marker attached to the object at Point A
WAYPOINT_MARKER_IDS = [1, 2, 3, 4]  # Markers along the route, in order
DEST_MARKER_ID = 10  # Marker at destination Point B

ARUCO_DICT_TYPE = "DICT_4X4_50"  # Small dictionary, fast detection
MARKER_SIZE = 0.08  # Physical marker side length in meters (8cm)

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
APPROACH_DISTANCE = 0.40  # meters — stop this far from object marker
WAYPOINT_SWITCH_DISTANCE = 0.60  # meters — close enough to switch to next waypoint
DEST_APPROACH_DISTANCE = 0.20  # meters — stop this far from destination marker
PERSON_BLOCK_AREA = (
    0.10  # min bounding-box area (fraction of frame) to trigger avoidance
)

# If marker not detected for this many seconds, rotate to search
MARKER_LOST_TIMEOUT = 3.0  # seconds

# If a waypoint marker can't be found after rotating for this long, skip it
WAYPOINT_SEARCH_TIMEOUT = 15.0  # seconds

# =============================================================================
# Obstacle Avoidance (ToF Sensor)
# =============================================================================
TOF_TOPIC = "range_0"  # ToF sensor topic
OBSTACLE_THRESHOLD = 0.50  # meters — trigger avoidance
OBSTACLE_CLEAR_THRESHOLD = 0.80  # meters — safe to resume
OBSTACLE_DEBOUNCE_COUNT = 3  # consecutive ToF readings required to change state
STRAFE_DIRECTION = 1.0  # +1.0 = strafe left, -1.0 = strafe right
STRAFE_DURATION = 1.5  # seconds to strafe before rechecking

# =============================================================================
# Chassis Speeds
# =============================================================================
SEARCH_ROTATE_SPEED = 0.3  # rad/s — rotation while searching for marker
APPROACH_MAX_SPEED = 0.25  # m/s — max forward speed during approach
APPROACH_MIN_SPEED = 0.08  # m/s — min forward speed (don't stall)
STRAFE_SPEED = 0.15  # m/s — sideways speed during avoidance
NAVIGATE_SPEED = 0.25  # m/s — forward speed between waypoints

# =============================================================================
# Visual Servoing Gains
# =============================================================================
# Proportional gain for steering toward marker center
STEER_KP = 1.5  # angular.z = KP * lateral_error

# Proportional gain for speed based on distance
SPEED_KP = 0.5  # linear.x = KP * distance (clamped)

# Lateral error dead zone — don't steer if error is smaller than this
STEER_DEADZONE = 0.02  # meters

# Continuous object approach visual servoing
APPROACH_CENTER_OK_H_ERR = 0.18
APPROACH_RECENTER_H_ERR = 0.32
APPROACH_CENTER_OK_LATERAL = 0.08
APPROACH_STRAFE_KP = 0.25
APPROACH_STRAFE_MAX_SPEED = 0.15
APPROACH_STRAFE_SIGN = -1.0
APPROACH_STRAFE_PROBE_TIME = 0.6
APPROACH_STRAFE_PROBE_IMPROVE_EPS = 0.03
APPROACH_YAW_KP = 0.6
APPROACH_YAW_MAX_SPEED = 0.20
APPROACH_YAW_SIGN = -1.0
APPROACH_CREEP_SPEED = 0.08
APPROACH_CLOSE_DISTANCE = 0.80
APPROACH_CLOSE_MAX_SPEED = 0.12
APPROACH_IGNORE_TOF_DISTANCE = 1.20

# =============================================================================
# Arm Positions (meters, relative to arm_base_link)
# =============================================================================
# These need tuning on the real robot!
ARM_PICK_X = 0.15  # forward reach for picking
ARM_PICK_Z = 0.10  # absolute height floor (hardware limit)
ARM_PICK_LOWER = 0.10  # relative drop from floor to reach object (negative = down)
ARM_CARRY_X = 0.09  # forward position while carrying
ARM_CARRY_Z = 0.07  # lifted height while carrying (match retract to keep camera level)
ARM_PLACE_X = 0.18  # forward reach for placing
ARM_PLACE_Z = 0.05  # height for placing
ARM_RETRACT_X = 0.09  # retracted position
ARM_RETRACT_Z = 0.10  # retracted height (lowered for better camera angle)

# =============================================================================
# Gripper
# =============================================================================
GRIPPER_OPEN = 1  # GripperControl target_state for open
GRIPPER_CLOSE = 2  # GripperControl target_state for close
GRIPPER_PAUSE = 0  # GripperControl target_state for pause
GRIPPER_POWER = 0.5  # Grip force [0.0 - 1.0]

# =============================================================================
# LED Colors (RGBA, values 0.0 to 1.0)
# =============================================================================
LED_SEARCHING = (0.0, 0.0, 1.0, 1.0)  # Blue — searching
LED_APPROACHING = (1.0, 1.0, 0.0, 1.0)  # Yellow — approaching
LED_PICKING = (1.0, 0.5, 0.0, 1.0)  # Orange — picking/placing
LED_NAVIGATING = (0.0, 1.0, 1.0, 1.0)  # Cyan — navigating
LED_AVOIDING = (1.0, 0.0, 0.0, 1.0)  # Red — obstacle avoidance
LED_DONE = (0.0, 1.0, 0.0, 1.0)  # Green — task complete

# =============================================================================
# Timing
# =============================================================================
CONTROL_LOOP_RATE = 10  # Hz — main control loop frequency
GRIPPER_WAIT_TIME = 1.5  # seconds — wait after gripper command
ARM_WAIT_TIME = 1.0  # seconds — wait after arm movement

# =============================================================================
# People Bypass (S-curve maneuver around a stationary person)
# =============================================================================
BYPASS_FORWARD_SPEED = 0.2  # m/s during all bypass phases
BYPASS_TURN_SPEED = 0.8  # rad/s during S-curve bypass phases
BYPASS_CURVE_TIME = 1.5  # seconds for each curve phase
BYPASS_STRAIGHT_TIME = 2.5  # seconds for the straight phase

# Dynamic (moving-person) avoidance turn speed — gentler than bypass S-curve
AVOID_TURN_SPEED = 0.4  # rad/s while sidestepping a moving person
