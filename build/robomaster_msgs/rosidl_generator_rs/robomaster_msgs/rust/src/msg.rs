#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to robomaster_msgs__msg__WheelSpeeds
/// A Command to set linear speeds for each wheel of the robot.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelSpeeds {
    /// front left wheel tangential speed in m/s
    pub front_left: f32,

    /// front right wheel tangential speed in m/s
    pub front_right: f32,

    /// rear left wheel tangential speed in m/s
    pub rear_left: f32,

    /// rear right wheel tangential speed in m/s
    pub rear_right: f32,

}



impl Default for WheelSpeeds {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelSpeeds::default())
  }
}

impl rosidl_runtime_rs::Message for WheelSpeeds {
  type RmwMsg = super::msg::rmw::WheelSpeeds;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        front_left: msg.front_left,
        front_right: msg.front_right,
        rear_left: msg.rear_left,
        rear_right: msg.rear_right,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      front_left: msg.front_left,
      front_right: msg.front_right,
      rear_left: msg.rear_left,
      rear_right: msg.rear_right,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      front_left: msg.front_left,
      front_right: msg.front_right,
      rear_left: msg.rear_left,
      rear_right: msg.rear_right,
    }
  }
}


// Corresponds to robomaster_msgs__msg__LEDEffect
/// Control commands for all LEDs

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LEDEffect {
    /// bitmask mask to select which LEDs to control
    pub mask: u8,

    /// bitmask to select which portions of the gimbal LED to control.
    /// The 7 sub LEDs are enumerate clockwise
    pub submask: u8,

    /// One of the effect enums
    pub effect: u8,

    /// The desired LED color (not relevant for ``effect=OFF``)
    pub color: std_msgs::msg::ColorRGBA,

    /// On interval duration, only relevant for effects ``PULSE`` and ``FLASH``.
    pub t1: f32,

    /// Off interval duration, only relevant for effects ``PULSE`` and ``FLASH``.
    pub t2: f32,

}

impl LEDEffect {
    /// mask: chassis rear LED
    pub const BOTTOM_BACK: u8 = 1;

    /// mask: chassis front LED
    pub const BOTTOM_FRONT: u8 = 2;

    /// mask: chassis left LED
    pub const BOTTOM_LEFT: u8 = 4;

    /// mask: chassis right LED
    pub const BOTTOM_RIGHT: u8 = 8;

    /// mask: all chassis LEDs
    pub const BOTTOM: u8 = 15;

    /// mask: gimbal left LED
    pub const TOP_LEFT: u8 = 16;

    /// mask: gimbal right LED
    pub const TOP_RIGHT: u8 = 32;

    /// mask: all gimbal LEDs
    pub const TOP: u8 = 48;

    /// mask: all LEDs
    pub const ALL: u8 = 63;

    /// effect: switch off LED
    pub const OFF: u8 = 0;

    /// effect: switch LED to solid color
    pub const ON: u8 = 1;

    /// effect: breath
    pub const BREATH: u8 = 2;

    /// effect: flash LED
    pub const FLASH: u8 = 3;

    /// effect: scrolling
    pub const SCROLLING: u8 = 4;

    /// effect: pulse LED
    pub const PULSE: u8 = 5;

}


impl Default for LEDEffect {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LEDEffect::default())
  }
}

impl rosidl_runtime_rs::Message for LEDEffect {
  type RmwMsg = super::msg::rmw::LEDEffect;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mask: msg.mask,
        submask: msg.submask,
        effect: msg.effect,
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Owned(msg.color)).into_owned(),
        t1: msg.t1,
        t2: msg.t2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mask: msg.mask,
      submask: msg.submask,
      effect: msg.effect,
        color: std_msgs::msg::ColorRGBA::into_rmw_message(std::borrow::Cow::Borrowed(&msg.color)).into_owned(),
      t1: msg.t1,
      t2: msg.t2,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mask: msg.mask,
      submask: msg.submask,
      effect: msg.effect,
      color: std_msgs::msg::ColorRGBA::from_rmw_message(msg.color),
      t1: msg.t1,
      t2: msg.t2,
    }
  }
}


// Corresponds to robomaster_msgs__msg__ChassisStatus
/// High-level chassis state provided by the robot through :py:meth:`robomaster.chassis.Chassis.sub_status`
/// TODO(Jerome): check

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisStatus {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// whenever the robot is still
    pub is_static: bool,

    /// whenever the robot is facing uphill
    pub up_hill: bool,

    /// whenever the robot is facing downhill
    pub down_hill: bool,

    /// whenever the robot is on a lateral slope
    pub on_slope: bool,

    /// whenever the robot has been picked up
    pub is_pick_up: bool,

    /// whenever wheels are slipping
    pub slip: bool,

    /// whenever the accelerometer has registered a collision longitudinally
    pub impact_x: bool,

    /// whenever the accelerometer has registered a collision laterally
    pub impact_y: bool,

    /// whenever the accelerometer has registered a collision vertically
    pub impact_z: bool,

    /// whenever the robot is capsized
    pub roll_over: bool,

    /// whenever the robot is facing uphill and is still.
    pub hill: bool,

}



impl Default for ChassisStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ChassisStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ChassisStatus {
  type RmwMsg = super::msg::rmw::ChassisStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_static: msg.is_static,
        up_hill: msg.up_hill,
        down_hill: msg.down_hill,
        on_slope: msg.on_slope,
        is_pick_up: msg.is_pick_up,
        slip: msg.slip,
        impact_x: msg.impact_x,
        impact_y: msg.impact_y,
        impact_z: msg.impact_z,
        roll_over: msg.roll_over,
        hill: msg.hill,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_static: msg.is_static,
      up_hill: msg.up_hill,
      down_hill: msg.down_hill,
      on_slope: msg.on_slope,
      is_pick_up: msg.is_pick_up,
      slip: msg.slip,
      impact_x: msg.impact_x,
      impact_y: msg.impact_y,
      impact_z: msg.impact_z,
      roll_over: msg.roll_over,
      hill: msg.hill,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_static: msg.is_static,
      up_hill: msg.up_hill,
      down_hill: msg.down_hill,
      on_slope: msg.on_slope,
      is_pick_up: msg.is_pick_up,
      slip: msg.slip,
      impact_x: msg.impact_x,
      impact_y: msg.impact_y,
      impact_z: msg.impact_z,
      roll_over: msg.roll_over,
      hill: msg.hill,
    }
  }
}


// Corresponds to robomaster_msgs__msg__SBus
/// Values recevied by the SBUS interface

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SBus {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// values received by the 16 channels
    pub channels: [i16; 16],

}



impl Default for SBus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SBus::default())
  }
}

impl rosidl_runtime_rs::Message for SBus {
  type RmwMsg = super::msg::rmw::SBus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        channels: msg.channels,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        channels: msg.channels,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      channels: msg.channels,
    }
  }
}


// Corresponds to robomaster_msgs__msg__PWM
/// Commands to control the 6 PWM interfaces

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PWM {
    /// Fraction of duty cycle in [0, 1]. Negative value signal to ignore the interface.
    pub fraction_of_duty_cycle: [f32; 6],

}



impl Default for PWM {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PWM::default())
  }
}

impl rosidl_runtime_rs::Message for PWM {
  type RmwMsg = super::msg::rmw::PWM;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        fraction_of_duty_cycle: msg.fraction_of_duty_cycle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        fraction_of_duty_cycle: msg.fraction_of_duty_cycle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      fraction_of_duty_cycle: msg.fraction_of_duty_cycle,
    }
  }
}


// Corresponds to robomaster_msgs__msg__GripperState
/// State of the gripper

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperState {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// one of ``PAUSE``. ``OPEN``. or ``CLOSE``
    pub state: u8,

}

impl GripperState {
    /// Gripper inactive
    pub const PAUSE: u8 = 0;

    /// Gripper open
    pub const OPEN: u8 = 1;

    /// Gripper closed
    pub const CLOSE: u8 = 2;

}


impl Default for GripperState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GripperState::default())
  }
}

impl rosidl_runtime_rs::Message for GripperState {
  type RmwMsg = super::msg::rmw::GripperState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      state: msg.state,
    }
  }
}


// Corresponds to robomaster_msgs__msg__CameraConfig
/// Command to configure the camera

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CameraConfig {
    /// relative zoom
    pub zoom: f32,

}



impl Default for CameraConfig {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CameraConfig::default())
  }
}

impl rosidl_runtime_rs::Message for CameraConfig {
  type RmwMsg = super::msg::rmw::CameraConfig;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        zoom: msg.zoom,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      zoom: msg.zoom,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      zoom: msg.zoom,
    }
  }
}


// Corresponds to robomaster_msgs__msg__ArmorHit
/// A hit event detected by one of the sensors placed behind the LEDs.
/// The Robomaster detects two kind of hits: collision using microphones,
/// and infrared beams using infrared receivers.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmorHit {
    /// header with the time stamp of the hit
    pub header: std_msgs::msg::Header,

    /// type of hit; one of ``COLLISION`` or ``IR``
    pub type_: u8,

    /// a bit mask for which sensors have detected the hit
    pub location: u8,

    /// the hit impact strength as measured by the sensor microphone.
    /// Only applies to events of ``COLLISION`` type
    pub level: u16,

}

impl ArmorHit {
    /// sensor location: on back of the chassis
    pub const BOTTOM_BACK: u8 = 1;

    /// sensor location: on front of the chassis
    pub const BOTTOM_FRONT: u8 = 2;

    /// sensor location: on left of the chassis
    pub const BOTTOM_LEFT: u8 = 4;

    /// sensor location: on right of the chassis
    pub const BOTTOM_RIGHT: u8 = 8;

    /// sensor location: on left of the gimbal
    pub const TOP_LEFT: u8 = 16;

    /// sensor location: on right of the gimbal
    pub const TOP_RIGHT: u8 = 32;

    /// type: detected by a microphone
    pub const COLLISION: u8 = 0;

    /// type: detected by the IR sensor
    pub const IR: u8 = 1;

}


impl Default for ArmorHit {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ArmorHit::default())
  }
}

impl rosidl_runtime_rs::Message for ArmorHit {
  type RmwMsg = super::msg::rmw::ArmorHit;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        type_: msg.type_,
        location: msg.location,
        level: msg.level,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      type_: msg.type_,
      location: msg.location,
      level: msg.level,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      type_: msg.type_,
      location: msg.location,
      level: msg.level,
    }
  }
}


// Corresponds to robomaster_msgs__msg__DetectedLine
/// Information about a line detected in the camera frame

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedLine {
    /// relative horizontal position of the line in the image (between 0 and 1)
    pub x: f32,

    /// relative vertical position of the line in the image (between 0 and 1)
    pub y: f32,

    /// slope
    pub angle: f32,

    /// curvature
    pub curvature: f32,

}



impl Default for DetectedLine {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedLine::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedLine {
  type RmwMsg = super::msg::rmw::DetectedLine;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        angle: msg.angle,
        curvature: msg.curvature,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      angle: msg.angle,
      curvature: msg.curvature,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      angle: msg.angle,
      curvature: msg.curvature,
    }
  }
}


// Corresponds to robomaster_msgs__msg__DetectedGesture
/// Information about an gesture detected using vision

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedGesture {
    /// bounding box
    pub roi: super::msg::RegionOfInterest,

    /// the name of the gesture
    pub kind: std::string::String,

}



impl Default for DetectedGesture {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedGesture::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedGesture {
  type RmwMsg = super::msg::rmw::DetectedGesture;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Owned(msg.roi)).into_owned(),
        kind: msg.kind.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.roi)).into_owned(),
        kind: msg.kind.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      roi: super::msg::RegionOfInterest::from_rmw_message(msg.roi),
      kind: msg.kind.to_string(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__DetectedMarker
/// Information about a marker detected using vision

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedMarker {
    /// bounding box
    pub roi: super::msg::RegionOfInterest,

    /// the name of the marker
    pub kind: std::string::String,

}



impl Default for DetectedMarker {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedMarker::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedMarker {
  type RmwMsg = super::msg::rmw::DetectedMarker;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Owned(msg.roi)).into_owned(),
        kind: msg.kind.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.roi)).into_owned(),
        kind: msg.kind.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      roi: super::msg::RegionOfInterest::from_rmw_message(msg.roi),
      kind: msg.kind.to_string(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__DetectedPerson
/// Information about a person detected using vision

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedPerson {
    /// bounding box
    pub roi: super::msg::RegionOfInterest,

}



impl Default for DetectedPerson {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedPerson::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedPerson {
  type RmwMsg = super::msg::rmw::DetectedPerson;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Owned(msg.roi)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.roi)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      roi: super::msg::RegionOfInterest::from_rmw_message(msg.roi),
    }
  }
}


// Corresponds to robomaster_msgs__msg__DetectedRobot
/// Information about a robot detected using vision

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedRobot {
    /// bounding box
    pub roi: super::msg::RegionOfInterest,

}



impl Default for DetectedRobot {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DetectedRobot::default())
  }
}

impl rosidl_runtime_rs::Message for DetectedRobot {
  type RmwMsg = super::msg::rmw::DetectedRobot;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Owned(msg.roi)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        roi: super::msg::RegionOfInterest::into_rmw_message(std::borrow::Cow::Borrowed(&msg.roi)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      roi: super::msg::RegionOfInterest::from_rmw_message(msg.roi),
    }
  }
}


// Corresponds to robomaster_msgs__msg__Detection
/// Summary of vision detections

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Detection {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// A list of people detections
    pub people: Vec<super::msg::DetectedPerson>,

    /// A list of robots detections
    pub robots: Vec<super::msg::DetectedRobot>,

    /// A list of gestures detections
    pub gestures: Vec<super::msg::DetectedGesture>,

    /// A list of lines detections
    pub lines: Vec<super::msg::DetectedLine>,

    /// A list of markers detections
    pub markers: Vec<super::msg::DetectedMarker>,

}



impl Default for Detection {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Detection::default())
  }
}

impl rosidl_runtime_rs::Message for Detection {
  type RmwMsg = super::msg::rmw::Detection;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        people: msg.people
          .into_iter()
          .map(|elem| super::msg::DetectedPerson::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        robots: msg.robots
          .into_iter()
          .map(|elem| super::msg::DetectedRobot::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        gestures: msg.gestures
          .into_iter()
          .map(|elem| super::msg::DetectedGesture::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        lines: msg.lines
          .into_iter()
          .map(|elem| super::msg::DetectedLine::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        markers: msg.markers
          .into_iter()
          .map(|elem| super::msg::DetectedMarker::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        people: msg.people
          .iter()
          .map(|elem| super::msg::DetectedPerson::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        robots: msg.robots
          .iter()
          .map(|elem| super::msg::DetectedRobot::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        gestures: msg.gestures
          .iter()
          .map(|elem| super::msg::DetectedGesture::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        lines: msg.lines
          .iter()
          .map(|elem| super::msg::DetectedLine::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        markers: msg.markers
          .iter()
          .map(|elem| super::msg::DetectedMarker::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      people: msg.people
          .into_iter()
          .map(super::msg::DetectedPerson::from_rmw_message)
          .collect(),
      robots: msg.robots
          .into_iter()
          .map(super::msg::DetectedRobot::from_rmw_message)
          .collect(),
      gestures: msg.gestures
          .into_iter()
          .map(super::msg::DetectedGesture::from_rmw_message)
          .collect(),
      lines: msg.lines
          .into_iter()
          .map(super::msg::DetectedLine::from_rmw_message)
          .collect(),
      markers: msg.markers
          .into_iter()
          .map(super::msg::DetectedMarker::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__RegionOfInterest
/// A region of interest (ROS) in an image
/// All values are in [0, 1] and are relative to image width and height

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RegionOfInterest {
    /// horizontal position of the ROI center
    /// as fraction of the image width
    /// (0 if the ROI includes the left edge of the image)
    pub x_offset: f32,

    /// vertical position of the ROI center
    /// as fraction of the image height
    /// (0 if the ROI includes the top edge of the image)
    pub y_offset: f32,

    /// height of ROI
    pub height: f32,

    /// width of ROI
    pub width: f32,

}



impl Default for RegionOfInterest {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::RegionOfInterest::default())
  }
}

impl rosidl_runtime_rs::Message for RegionOfInterest {
  type RmwMsg = super::msg::rmw::RegionOfInterest;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x_offset: msg.x_offset,
        y_offset: msg.y_offset,
        height: msg.height,
        width: msg.width,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x_offset: msg.x_offset,
      y_offset: msg.y_offset,
      height: msg.height,
      width: msg.width,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x_offset: msg.x_offset,
      y_offset: msg.y_offset,
      height: msg.height,
      width: msg.width,
    }
  }
}


// Corresponds to robomaster_msgs__msg__Serial
/// Encapsulates serial communication received or to be sent through UART

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Serial {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// the payload
    pub data: Vec<u8>,

}



impl Default for Serial {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Serial::default())
  }
}

impl rosidl_runtime_rs::Message for Serial {
  type RmwMsg = super::msg::rmw::Serial;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__ServoCommand
/// Command to control a servo in speed

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoCommand {
    /// ID of the servo (between 0 and 2)
    pub index: u8,

    /// whenever to engage or disengage the servo
    pub enable: bool,

    /// target angular speed in rad/m
    pub angular_speed: f32,

}



impl Default for ServoCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ServoCommand::default())
  }
}

impl rosidl_runtime_rs::Message for ServoCommand {
  type RmwMsg = super::msg::rmw::ServoCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
        enable: msg.enable,
        angular_speed: msg.angular_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      enable: msg.enable,
      angular_speed: msg.angular_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
      enable: msg.enable,
      angular_speed: msg.angular_speed,
    }
  }
}


// Corresponds to robomaster_msgs__msg__GimbalCommand
/// Command to control the gripper in speed

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GimbalCommand {
    /// target yaw angular speed in rad/m
    pub yaw_speed: f32,

    /// target pitch angular speed in rad/m
    pub pitch_speed: f32,

}



impl Default for GimbalCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GimbalCommand::default())
  }
}

impl rosidl_runtime_rs::Message for GimbalCommand {
  type RmwMsg = super::msg::rmw::GimbalCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        yaw_speed: msg.yaw_speed,
        pitch_speed: msg.pitch_speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      yaw_speed: msg.yaw_speed,
      pitch_speed: msg.pitch_speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      yaw_speed: msg.yaw_speed,
      pitch_speed: msg.pitch_speed,
    }
  }
}


// Corresponds to robomaster_msgs__msg__BlasterLED
/// Command to control the blaster LED

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlasterLED {
    /// relative intensity of the LED: 0.0: off, 1.0: full brightness
    pub brightness: f32,

}



impl Default for BlasterLED {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BlasterLED::default())
  }
}

impl rosidl_runtime_rs::Message for BlasterLED {
  type RmwMsg = super::msg::rmw::BlasterLED;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        brightness: msg.brightness,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      brightness: msg.brightness,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      brightness: msg.brightness,
    }
  }
}


// Corresponds to robomaster_msgs__msg__Mode
/// Coupling control model between gimbal and chassis.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Mode {
    /// coupling mode, one of ``FREE``, ``GIMBAL_LEAD``, ``CHASSIS_LEAD``
    pub mode: u8,

}

impl Mode {
    /// mode: gimbal and chassis are decoupled
    pub const FREE: u8 = 0;

    /// mode: chassis follows gimbal movements
    /// to maintain the same relative orientation
    pub const GIMBAL_LEAD: u8 = 1;

    /// mode: gimbal follows gimbal movements
    /// to maintain the same relative orientation
    pub const CHASSIS_LEAD: u8 = 2;

}


impl Default for Mode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Mode::default())
  }
}

impl rosidl_runtime_rs::Message for Mode {
  type RmwMsg = super::msg::rmw::Mode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mode: msg.mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      mode: msg.mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mode: msg.mode,
    }
  }
}


// Corresponds to robomaster_msgs__msg__AudioData
/// Raw audio samples
/// .. note::
///   Unlike ROS1, ROS2 has no ``audio_common_msgs``, so for now we define our own format
///   to publish the raw 16-bit 48Khz audio stream from the camera mono microphone

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioData {
    /// the timestamp and frame of the microphone recording
    pub header: std_msgs::msg::Header,

    /// an array of 16-bit audio samples
    pub data: Vec<i16>,

}



impl Default for AudioData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioData::default())
  }
}

impl rosidl_runtime_rs::Message for AudioData {
  type RmwMsg = super::msg::rmw::AudioData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__AudioLevel
/// Sound level
/// .. note::
///   Unlike ROS1, ROS2 has no ``audio_common_msgs``, so we define our own format
///   for sound level in dBFS

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioLevel {
    /// the timestamp and frame of the microphone recording
    pub header: std_msgs::msg::Header,

    /// sound level in dBFS
    pub level: f32,

}



impl Default for AudioLevel {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioLevel::default())
  }
}

impl rosidl_runtime_rs::Message for AudioLevel {
  type RmwMsg = super::msg::rmw::AudioLevel;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        level: msg.level,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      level: msg.level,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      level: msg.level,
    }
  }
}


// Corresponds to robomaster_msgs__msg__AudioOpus
/// Encapsulate a packet from an OPUS encoded audio stream

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioOpus {
    /// header with the time stamp and origin of the packet
    pub header: std_msgs::msg::Header,

    /// sequential number of the packet
    pub seq: u64,

    /// Opus data
    pub buffer: Vec<u8>,

}



impl Default for AudioOpus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioOpus::default())
  }
}

impl rosidl_runtime_rs::Message for AudioOpus {
  type RmwMsg = super::msg::rmw::AudioOpus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        seq: msg.seq,
        buffer: msg.buffer.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      seq: msg.seq,
        buffer: msg.buffer.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      seq: msg.seq,
      buffer: msg.buffer
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__H264Packet
/// Encapsulate a packet from a H264 encoded video stream

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct H264Packet {
    /// header with the time stamp and origin of the packet
    pub header: std_msgs::msg::Header,

    /// sequential number of the packet
    pub seq: u64,

    /// H264 data
    pub data: Vec<u8>,

}



impl Default for H264Packet {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::H264Packet::default())
  }
}

impl rosidl_runtime_rs::Message for H264Packet {
  type RmwMsg = super::msg::rmw::H264Packet;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        seq: msg.seq,
        data: msg.data.into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      seq: msg.seq,
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      seq: msg.seq,
      data: msg.data
          .into_iter()
          .collect(),
    }
  }
}


// Corresponds to robomaster_msgs__msg__SensorAdapter
/// Summarizes the state of up to 6 sensor adapters (each with 2 interfaces)

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorAdapter {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// input pin state (0 or 1)
    pub io: [u8; 12],

    /// adc state
    pub adc: [i16; 12],

    /// interface intentifier
    pub port: [u8; 12],

    /// sensor adapter identifier
    pub id: [u8; 12],

}



impl Default for SensorAdapter {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SensorAdapter::default())
  }
}

impl rosidl_runtime_rs::Message for SensorAdapter {
  type RmwMsg = super::msg::rmw::SensorAdapter;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        io: msg.io,
        adc: msg.adc,
        port: msg.port,
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        io: msg.io,
        adc: msg.adc,
        port: msg.port,
        id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      io: msg.io,
      adc: msg.adc,
      port: msg.port,
      id: msg.id,
    }
  }
}


// Corresponds to robomaster_msgs__msg__ServoRawState
/// Raw state from the servos on ports {0, 1, 2, 3}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoRawState {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::Header,

    /// whenever a servo in connected to a port
    pub valid: [bool; 4],

    /// the angle of the servos (1024 corresponds to 180 degrees)
    pub value: [i32; 4],

    /// the speed of the servos (1024 corresponds to about 36 degrees per seconds)
    pub speed: [i32; 4],

}



impl Default for ServoRawState {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ServoRawState::default())
  }
}

impl rosidl_runtime_rs::Message for ServoRawState {
  type RmwMsg = super::msg::rmw::ServoRawState;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        valid: msg.valid,
        value: msg.value,
        speed: msg.speed,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        valid: msg.valid,
        value: msg.value,
        speed: msg.speed,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      valid: msg.valid,
      value: msg.value,
      speed: msg.speed,
    }
  }
}


// Corresponds to robomaster_msgs__msg__SpeakerCommand
/// An command to stop or play a predefined sound or an audio file from the speaker.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeakerCommand {
    /// ID of the sound to play
    pub sound_id: i32,

    /// how many times to play the same sound (0: infinite loop)
    pub times: u8,

    /// Control command: one of ``CONTROL_STOP``, ``CONTROL_PLAY``, ``CONTROL_ADD``
    pub control: u8,

    /// if not empry, the local file to be played.
    pub file: std::string::String,

}

impl SpeakerCommand {
    /// Stop playing sound
    pub const CONTROL_STOP: u8 = 0;

    /// Start playing sound after stopping currently played sounds
    pub const CONTROL_PLAY: u8 = 1;

    /// Start playing sound, without stopping currently played sounds
    pub const CONTROL_ADD: u8 = 2;

}


impl Default for SpeakerCommand {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SpeakerCommand::default())
  }
}

impl rosidl_runtime_rs::Message for SpeakerCommand {
  type RmwMsg = super::msg::rmw::SpeakerCommand;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sound_id: msg.sound_id,
        times: msg.times,
        control: msg.control,
        file: msg.file.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      sound_id: msg.sound_id,
      times: msg.times,
      control: msg.control,
        file: msg.file.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sound_id: msg.sound_id,
      times: msg.times,
      control: msg.control,
      file: msg.file.to_string(),
    }
  }
}


