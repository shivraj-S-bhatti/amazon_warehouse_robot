# robomaster_detection

A ROS 2 package that subscribes to the RoboMaster driver's built-in
vision system and provides clean, re-published detection topics plus an
annotated camera image overlay.

---

## Package structure

```
robomaster_detection/
├── launch/
│   └── detection.launch.py       ← starts driver + both nodes
├── robomaster_detection/
│   ├── detection_node.py         ← core detection subscriber / re-publisher
│   └── visualizer_node.py        ← draws bounding boxes on the camera image
├── resource/
│   └── robomaster_detection
├── package.xml
├── setup.cfg
└── setup.py
```

---

## Prerequisites

| Dependency | Notes |
|---|---|
| ROS 2 (Humble or newer) | Tested on Humble |
| [`robomaster_ros`](https://github.com/jeguzzi/robomaster_ros) | Driver + msgs |
| `python3-opencv` | For the visualizer node |

---

## Installation

```bash
# 1. Place this package inside your ROS 2 workspace
cd ~/ros2_ws/src
cp -r /path/to/robomaster_detection .

# 2. Install Python deps
pip install opencv-python   # if not installed via apt

# 3. Build
cd ~/ros2_ws
colcon build --packages-select robomaster_detection
source install/setup.bash
```

---

## Quick start

```bash
# Wi-Fi (STA mode, robot already joined your network)
ros2 launch robomaster_detection detection.launch.py

# Robot's own access-point mode
ros2 launch robomaster_detection detection.launch.py conn_type:=ap

# USB (RNDIS) with a specific serial number, no visualizer
ros2 launch robomaster_detection detection.launch.py \
    conn_type:=rndis \
    serial_number:=3JKDH12345678 \
    visualize:=false
```

---

## Topics

### Subscribed (from the driver)

| Topic | Type | Description |
|---|---|---|
| `vision` | `robomaster_msgs/msg/Detection` | All detections from the robot |
| `camera/image_color` | `sensor_msgs/msg/Image` | Raw camera frame (visualizer only) |

### Published

| Topic | Type | Description |
|---|---|---|
| `~/people` | `robomaster_msgs/msg/Detection` | Person-only detection frames |
| `~/markers` | `robomaster_msgs/msg/Detection` | Marker-only detection frames |
| `~/detection_info` | `std_msgs/msg/String` | Human-readable detection summary |
| `~/annotated_image` | `sensor_msgs/msg/Image` | Camera image with bounding boxes drawn |

---

## Parameters

### detection_node

| Parameter | Type | Default | Description |
|---|---|---|---|
| `verbose` | bool | `true` | Print each detection to the console |

### Launch arguments

| Argument | Default | Description |
|---|---|---|
| `conn_type` | `sta` | Connection mode: `sta`, `ap`, `rndis` |
| `serial_number` | `""` | Robot serial (empty = auto-discover) |
| `visualize` | `true` | Launch the visualizer node |
| `verbose` | `true` | Console logging in detection_node |

---

## Detection targets

The driver's `vision.targets` parameter controls what the robot's
onboard AI looks for. The launch file requests:

```python
["person", "marker:red", "marker:blue", "marker:green"]
```

You can also add `"robot"` or `"gesture"` — edit the launch file or
override via `ros2 param set`.

---

## Viewing the annotated image

```bash
# rqt
rqt_image_view /robomaster_visualizer_node/annotated_image

# or with image_view
ros2 run image_view image_view \
    --ros-args -r image:=/robomaster_visualizer_node/annotated_image
```

### Bounding-box colours

| Colour | Detected object |
|---|---|
| 🟢 Green | Person |
| 🔵 Blue | Marker (kind shown in label) |
| 🟡 Yellow | Other RoboMaster robot |

---

## How it works

```
RoboMaster robot
      │  Wi-Fi / USB
      ▼
robomaster_driver (robomaster_ros)
      │
      ├─ /vision  ──────────────────────────► detection_node
      │                                           ├─ ~/people
      │                                           ├─ ~/markers
      │                                           └─ ~/detection_info
      │
      └─ /camera/image_color ──────────────► visualizer_node
                                                 └─ ~/annotated_image
```

The driver's onboard vision module runs on the robot's intelligent
controller. It publishes normalised bounding boxes (all coordinates are
fractions of the image width/height in [0, 1]) together with metadata
such as the marker kind string.

`detection_node` filters those into per-class topics and publishes a
plain-text summary.  `visualizer_node` fuses the latest detection with
each incoming camera frame and draws labelled rectangles using OpenCV.
