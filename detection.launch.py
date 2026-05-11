import os
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription, GroupAction
from launch.launch_description_sources import AnyLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PythonExpression, PathJoinSubstitution
from launch.conditions import IfCondition
from launch_ros.actions import Node, PushRosNamespace
from launch_ros.substitutions import FindPackageShare

def generate_launch_description() -> LaunchDescription:

    # ── 1. Declare Launch Arguments ───────────────────────────────────────────
    
    # Custom args
    verbose_arg = DeclareLaunchArgument("verbose", default_value="true", description="Log each detection to the console")
    
    # Official driver args
    name_arg = DeclareLaunchArgument("name", default_value="", description="A name used as ROS namespace")
    model_arg = DeclareLaunchArgument("model", default_value="ep", description="The type of robot: ep or s1")
    with_model_desc_arg = DeclareLaunchArgument("with_model_description", default_value="true", description="Enables publishing the robot model")
    serial_number_arg = DeclareLaunchArgument("serial_number", default_value="", description="The serial number of the robot")
    conn_type_arg = DeclareLaunchArgument("conn_type", default_value="sta", description="The type of connection: sta, ap, or rndis")
    
    # ToF Sensors
    tof_0_arg = DeclareLaunchArgument("tof_0", default_value="True", description="Enable ToF sensor #0")
    tof_1_arg = DeclareLaunchArgument("tof_1", default_value="False", description="Enable ToF sensor #1")
    tof_2_arg = DeclareLaunchArgument("tof_2", default_value="False", description="Enable ToF sensor #2")
    tof_3_arg = DeclareLaunchArgument("tof_3", default_value="False", description="Enable ToF sensor #3")

    # ── 2. Create Launch Configurations ───────────────────────────────────────
    verbose = LaunchConfiguration("verbose")
    name = LaunchConfiguration("name")
    model = LaunchConfiguration("model")
    with_model_description = LaunchConfiguration("with_model_description")
    serial_number = LaunchConfiguration("serial_number")
    conn_type = LaunchConfiguration("conn_type")
    
    tof_0 = LaunchConfiguration("tof_0")
    tof_1 = LaunchConfiguration("tof_1")
    tof_2 = LaunchConfiguration("tof_2")
    tof_3 = LaunchConfiguration("tof_3")

    # Evaluate if *any* ToF sensor is enabled to turn on the driver module
    tof_enabled_expr = PythonExpression([
        "'", tof_0, "' in ['true', 'True'] or ",
        "'", tof_1, "' in ['true', 'True'] or ",
        "'", tof_2, "' in ['true', 'True'] or ",
        "'", tof_3, "' in ['true', 'True']"
    ])

    # Dynamic hardware switching (EP has arm/gripper, S1 has blaster/gimbal)
    is_ep = PythonExpression(["'", model, "' == 'ep'"])
    is_s1 = PythonExpression(["'", model, "' == 's1'"])

    # ── 3. Define Nodes and Includes ──────────────────────────────────────────

    # Official RoboMaster Driver Node with full nested parameter map
    driver_node = Node(
        package="robomaster_ros",
        executable="robomaster_driver",
        name="robomaster_driver",
        output="screen",
        parameters=[
            {
                "tf_prefix": name,
                "serial_number": serial_number,
                "conn_type": conn_type,
                "lib_log_level": "ERROR",
                "mode": 2, # GIMBAL_LEAD
                
                # Hardware Modules
                "arm.enabled": is_ep,
                "arm.rate": 10,
                "arm.right_motor.zero": 1024,
                "arm.left_motor.zero": 1024,
                "arm.right_motor.direction": -1,
                "arm.left_motor.direction": 1,
                "arm.right_motor.angle": -0.274016,
                "arm.left_motor.angle": 0.073304,
                "arm.right_motor.index": 1,
                "arm.left_motor.index": 0,

                "armor.enabled": False,
                "armor.sensitivity": 0.5,
                "battery.enabled": True,
                "blaster.enabled": is_s1,
                
                "camera.enabled": True,
                "camera.video.raw": 2,     # ON_DEMAND
                "camera.video.h264": 2,    # ON_DEMAND
                "camera.video.ffmpeg": 2,  # ON_DEMAND
                "camera.video.resolution": 540,
                "camera.video.rate": -1.0,
                "camera.audio.raw": 0,
                "camera.audio.opus": 0,
                "camera.audio.level": 0,
                
                "chassis.enabled": True,
                "chassis.rate": 10,
                "chassis.status.rate": 1,
                "chassis.timeout": 0.0,
                "chassis.twist_to_wheel_speeds": False,
                "chassis.force_level": False,
                "chassis.odom_twist_in_odom": False,
                "chassis.imu_includes_orientation": True,
                "chassis.error.linear_velocity.xy": 0.005,
                "chassis.error.angular_velocity.x": 0.01,
                "chassis.error.angular_velocity.z": 0.03,
                "chassis.error.linear_acceleration.xyz": 0.1,

                "gripper.enabled": is_ep,
                "gripper.open": False,
                
                "led.enabled": True,
                "pwm.enabled": False,
                "sbus.enabled": False,
                "servo.enabled": False,
                "speaker.enabled": True,
                
                "tof.enabled": tof_enabled_expr,
                "tof.rate": 10,
                
                "uart.enabled": False,
                
                "vision.enabled": True,
                # IMPORTANT: The DJI vision SDK is single-color for markers.
                # Each call to sub_detect_info("marker", color=…) OVERWRITES the
                # previous color filter, so listing multiple "marker:<color>"
                # entries leaves only the LAST one active. Pick the one that
                # matches the physical color of your markers:
                #   "marker:red"   — official DJI Robomaster vision markers
                #                    (heart, sword, digits, letters, arrows) are
                #                    printed in red — this is almost always the
                #                    correct setting.
                #   "marker:green" — green-printed markers (rare)
                #   "marker:blue"  — blue-printed markers (rare)
                # If detection fails, try switching the color one at a time and
                # re-launching.
                "vision.targets": ["person", "marker:red"],
                
                "gimbal.enabled": is_s1,
                "gimbal.rate": 10,
                
                "sensor_adapter.enabled": False,
            }
        ],
    )

    # Official Robot URDF/TF Publisher
    robot_description_launch = IncludeLaunchDescription(
        AnyLaunchDescriptionSource(
            PathJoinSubstitution([FindPackageShare('robomaster_description'), 'launch', 'main.launch'])
        ),
        condition=IfCondition(with_model_description),
        launch_arguments={
            'name': name,
            'model': model,
            'tof_0': tof_0,
            'tof_1': tof_1,
            'tof_2': tof_2,
            'tof_3': tof_3,
            'camera_yaw': '0.0',
        }.items()
    )

    # Your Custom Vision Pipeline
    detection_node = Node(
        package="robomaster_detection",
        executable="detection_node",
        name="robomaster_detection_node",
        output="screen",
        parameters=[{"verbose": verbose}],
    )

    # Your Custom Kinematic Avoidance FSM
    avoidance_node = Node(
        package="robomaster_detection",
        executable="avoidance_node",
        name="robomaster_avoidance_node",
        output="screen",
    )

    # ── 4. Group into a Namespace (Matches the XML <group>) ───────────────────
    launch_group = GroupAction([
        PushRosNamespace(name),
        driver_node,
        robot_description_launch,
        detection_node,
        avoidance_node,
    ])

    return LaunchDescription(
        [
            verbose_arg,
            name_arg,
            model_arg,
            with_model_desc_arg,
            serial_number_arg,
            conn_type_arg,
            tof_0_arg,
            tof_1_arg,
            tof_2_arg,
            tof_3_arg,
            launch_group,
        ]
    )