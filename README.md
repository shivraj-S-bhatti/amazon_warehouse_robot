# Autonomous Pick-and-Transport — DJI RoboMaster EP Core

> COMPSCI-603 Group Project · ROS 2 Humble · Jeet Patel · Linfeng Lyu · Piyush Maheshwari · Shivraj Bhatti

The robot autonomously locates an object marked with an ArUco tag, picks it up, navigates through a sequence of waypoint markers while avoiding static obstacles and pedestrians, and places the object at a destination marker.

```
SEARCH → APPROACH → PICK UP → NAVIGATE (waypoints) → APPROACH DEST → PLACE → DONE
                                    ↕
                             OBSTACLE AVOID / BYPASS
```

---

## How it works

| Layer | File | What it does |
|---|---|---|
| Perception | `vision.py` | Detects ArUco markers every frame, estimates 6-DOF pose with `solvePnP` |
| Safety | `obstacle.py` | Reads the ToF sensor; debounced blocked/clear state with hysteresis |
| People | `people_avoidance.py` | EMA velocity classifier over person bounding-box; MOVING vs STATIONARY |
| Control | `chassis_control.py` | Visual servoing (yaw + strafe + forward), LED status |
| Manipulation | `arm_control.py` | Blocking pick/carry/place sequences via ROS 2 action servers |
| Brain | `main.py` | 10-state FSM at 10 Hz — coordinates everything above |
| Config | `config.py` | All tunable constants in one place |

---

## Requirements

- Ubuntu 22.04 + **ROS 2 Humble**
- Python 3.10
- `opencv-contrib-python` (for ArUco support)
- [`robomaster_ros`](https://github.com/jeguzzi/robomaster_ros) driver built in the same workspace

Install Python dependencies:
```bash
pip install opencv-contrib-python numpy cv_bridge
```

---

## Workspace setup

```bash
# 1. Create workspace and clone the driver
mkdir -p ~/robomaster_ws/src && cd ~/robomaster_ws/src
git clone https://github.com/jeguzzi/robomaster_ros.git

# 2. Install RoboMaster SDK
pip install git+https://github.com/jeguzzi/RoboMasterSDK.git
pip install git+https://github.com/jeguzzi/RoboMasterSDK.git#"egg=libmedia_codec&subdirectory=lib/libmedia_codec"

# 3. Build
cd ~/robomaster_ws
colcon build

# 4. Source
source ~/robomaster_ws/install/setup.bash
```

---

## Connecting the robot

The robot connects in **Wi-Fi Access Point (AP) mode** — your laptop joins the robot's own hotspot.

1. Power on the EP Core (hold the battery button until all green LEDs light up).
2. On the robot's intelligent controller, slide the network selector to the **AP icon**.
3. Connect your laptop to the robot's Wi-Fi network.
4. Verify the connection:
```bash
ros2 run robomaster_ros discover
# Expected: "Discovered 1 robots ... Connected to robot"
```

---

## Running the full task

Open **two terminals**, both with the workspace sourced (`source install/setup.bash`).

**Terminal 1 — launch the driver:**
```bash
ros2 launch robomaster_ros main.launch model:=ep conn_type:=ap \
  --ros-args --params-file ~/robomaster_ws/src/robomaster_ros/scripts/config.yaml
```
OR

```
ros2 launch robomaster_ros main.launch model:=ep conn_type:=ap tof_0:=True vision_targets:="['person']"

```

**Terminal 2 — run the state machine:**
```bash
cd ~/robomaster_ws/src/robomaster_ros/scripts
python3 main.py
```

Press **Ctrl+C** in Terminal 2 at any time — the robot stops immediately and the arm retracts.

---

## Marker setup

Print markers from [chev.me/arucogen](https://chev.me/arucogen/) using **Dictionary: 4x4\_50**.  
Physical side length must match `MARKER_SIZE` in `config.py` (default **8 cm**).

| Marker ID | Role | Placement |
|---|---|---|
| `0` | Object | Attached to the object at Point A |
| `1` – `4` | Waypoints | Placed along the route in order |
| `10` | Destination | Placed at the drop zone (Point B) |

---

## Standalone module tests

Each module can be tested independently without running the full task.

```bash
cd ~/robomaster_ws/src/robomaster_ros/scripts

# Test camera + ArUco detection (hold a marker in front of the camera)
python3 vision.py

# Test ToF sensor (place objects in front of the robot)
python3 obstacle.py

# Test chassis movement (robot will move — clear the area!)
python3 chassis_control.py

# Test arm + gripper sequences (robot arm will move!)
python3 arm_control.py
```

---

## Configuration

All tunable parameters live in [`config.py`](src/robomaster_ros/scripts/config.py). The most commonly adjusted values:

```python
# Approach distances
APPROACH_DISTANCE       = 0.40   # m — stop this far from the object marker
DEST_APPROACH_DISTANCE  = 0.20   # m — stop this far from the destination

# ToF obstacle thresholds
OBSTACLE_THRESHOLD      = 0.50   # m — triggers avoidance
OBSTACLE_CLEAR_THRESHOLD= 0.80   # m — resumes navigation (hysteresis gap)

# Visual servoing gains
STEER_KP                = 1.5    # yaw proportional gain
SPEED_KP                = 0.5    # forward speed proportional gain
APPROACH_STRAFE_KP      = 0.25   # lateral strafe proportional gain

# Arm poses (x = forward reach, z = height, in meters)
ARM_PICK_X, ARM_PICK_Z     = 0.15, 0.10   # lower to grasp
ARM_CARRY_X, ARM_CARRY_Z   = 0.09, 0.07   # lifted carry position
ARM_PLACE_X, ARM_PLACE_Z   = 0.18, 0.05   # lower to release
ARM_RETRACT_X, ARM_RETRACT_Z = 0.09, 0.10 # safe idle position
```

The driver's hardware enables are set in [`config.yaml`](src/robomaster_ros/scripts/config.yaml) — make sure `arm`, `gripper`, `camera`, and `tof` are all `enabled: true`.

---

## LED status codes

The robot's LEDs communicate its current state at a glance:

| Color | State |
|---|---|
| Blue | Searching for a marker |
| Yellow | Approaching a marker |
| Orange | Picking up or placing |
| Cyan | Navigating between waypoints |
| Red | Obstacle avoidance active |
| Green | Task complete |

---

## Project structure

```
robomaster_ws/
├── src/
│   └── robomaster_ros/
│       └── scripts/
│           ├── main.py              # FSM entry point — run this
│           ├── config.py            # All tunable constants
│           ├── config.yaml          # Driver hardware enable flags
│           ├── vision.py            # ArUco detection + solvePnP
│           ├── obstacle.py          # ToF sensor + debounce logic
│           ├── people_avoidance.py  # EMA pedestrian classifier
│           ├── chassis_control.py   # cmd_vel + LED control
│           ├── arm_control.py       # pick / carry / place sequences
│           └── arm_gripper_test.py  # Quick hardware sanity check
├── root.tex                         # IEEE paper (Deliverable 4)
└── README.md
```

---

## Known limitations

- **Camera intrinsics are estimated.** The 120° FOV focal length estimate (`fx = fy ≈ 185 px`) introduces 10–20% distance error beyond 1.5 m. Run a checkerboard calibration and update `CAMERA_FX/FY/CX/CY` in `config.py` for better approach accuracy.
- **Strafe-sign probe.** On the first approach, the controller takes up to 0.6 s to detect and correct the lateral strafe direction. If the robot initially drifts away from the marker, this is expected behavior.
- **Pedestrian bypass heading.** The heading-correction counter-rotation after a moving-person maneuver relies on open-loop timing. Large Wi-Fi jitter can cause overshoot; re-acquiring the next waypoint marker before issuing the correction is the recommended fix.
