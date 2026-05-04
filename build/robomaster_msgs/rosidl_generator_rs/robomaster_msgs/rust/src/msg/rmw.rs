#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__WheelSpeeds() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__WheelSpeeds__init(msg: *mut WheelSpeeds) -> bool;
    fn robomaster_msgs__msg__WheelSpeeds__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>, size: usize) -> bool;
    fn robomaster_msgs__msg__WheelSpeeds__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>);
    fn robomaster_msgs__msg__WheelSpeeds__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelSpeeds>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelSpeeds>) -> bool;
}

// Corresponds to robomaster_msgs__msg__WheelSpeeds
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A Command to set linear speeds for each wheel of the robot.

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__WheelSpeeds__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__WheelSpeeds__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelSpeeds {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__WheelSpeeds__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__WheelSpeeds__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__WheelSpeeds__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelSpeeds {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelSpeeds where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/WheelSpeeds";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__WheelSpeeds() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__LEDEffect() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__LEDEffect__init(msg: *mut LEDEffect) -> bool;
    fn robomaster_msgs__msg__LEDEffect__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LEDEffect>, size: usize) -> bool;
    fn robomaster_msgs__msg__LEDEffect__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LEDEffect>);
    fn robomaster_msgs__msg__LEDEffect__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LEDEffect>, out_seq: *mut rosidl_runtime_rs::Sequence<LEDEffect>) -> bool;
}

// Corresponds to robomaster_msgs__msg__LEDEffect
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Control commands for all LEDs

#[repr(C)]
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
    pub color: std_msgs::msg::rmw::ColorRGBA,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__LEDEffect__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__LEDEffect__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LEDEffect {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__LEDEffect__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__LEDEffect__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__LEDEffect__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LEDEffect {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LEDEffect where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/LEDEffect";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__LEDEffect() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ChassisStatus() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__ChassisStatus__init(msg: *mut ChassisStatus) -> bool;
    fn robomaster_msgs__msg__ChassisStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ChassisStatus>, size: usize) -> bool;
    fn robomaster_msgs__msg__ChassisStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ChassisStatus>);
    fn robomaster_msgs__msg__ChassisStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ChassisStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ChassisStatus>) -> bool;
}

// Corresponds to robomaster_msgs__msg__ChassisStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// High-level chassis state provided by the robot through :py:meth:`robomaster.chassis.Chassis.sub_status`
/// TODO(Jerome): check

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ChassisStatus {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__ChassisStatus__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__ChassisStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ChassisStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ChassisStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ChassisStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ChassisStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ChassisStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ChassisStatus where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/ChassisStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ChassisStatus() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SBus() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__SBus__init(msg: *mut SBus) -> bool;
    fn robomaster_msgs__msg__SBus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SBus>, size: usize) -> bool;
    fn robomaster_msgs__msg__SBus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SBus>);
    fn robomaster_msgs__msg__SBus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SBus>, out_seq: *mut rosidl_runtime_rs::Sequence<SBus>) -> bool;
}

// Corresponds to robomaster_msgs__msg__SBus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Values recevied by the SBUS interface

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SBus {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

    /// values received by the 16 channels
    pub channels: [i16; 16],

}



impl Default for SBus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__SBus__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__SBus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SBus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SBus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SBus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SBus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SBus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SBus where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/SBus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SBus() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__PWM() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__PWM__init(msg: *mut PWM) -> bool;
    fn robomaster_msgs__msg__PWM__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PWM>, size: usize) -> bool;
    fn robomaster_msgs__msg__PWM__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PWM>);
    fn robomaster_msgs__msg__PWM__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PWM>, out_seq: *mut rosidl_runtime_rs::Sequence<PWM>) -> bool;
}

// Corresponds to robomaster_msgs__msg__PWM
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Commands to control the 6 PWM interfaces

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PWM {
    /// Fraction of duty cycle in [0, 1]. Negative value signal to ignore the interface.
    pub fraction_of_duty_cycle: [f32; 6],

}



impl Default for PWM {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__PWM__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__PWM__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PWM {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__PWM__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__PWM__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__PWM__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PWM {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PWM where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/PWM";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__PWM() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__GripperState() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__GripperState__init(msg: *mut GripperState) -> bool;
    fn robomaster_msgs__msg__GripperState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperState>, size: usize) -> bool;
    fn robomaster_msgs__msg__GripperState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperState>);
    fn robomaster_msgs__msg__GripperState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperState>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperState>) -> bool;
}

// Corresponds to robomaster_msgs__msg__GripperState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// State of the gripper

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperState {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__GripperState__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__GripperState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GripperState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GripperState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GripperState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperState where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/GripperState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__GripperState() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__CameraConfig() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__CameraConfig__init(msg: *mut CameraConfig) -> bool;
    fn robomaster_msgs__msg__CameraConfig__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CameraConfig>, size: usize) -> bool;
    fn robomaster_msgs__msg__CameraConfig__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CameraConfig>);
    fn robomaster_msgs__msg__CameraConfig__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CameraConfig>, out_seq: *mut rosidl_runtime_rs::Sequence<CameraConfig>) -> bool;
}

// Corresponds to robomaster_msgs__msg__CameraConfig
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Command to configure the camera

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CameraConfig {
    /// relative zoom
    pub zoom: f32,

}



impl Default for CameraConfig {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__CameraConfig__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__CameraConfig__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CameraConfig {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__CameraConfig__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__CameraConfig__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__CameraConfig__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CameraConfig {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CameraConfig where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/CameraConfig";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__CameraConfig() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ArmorHit() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__ArmorHit__init(msg: *mut ArmorHit) -> bool;
    fn robomaster_msgs__msg__ArmorHit__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmorHit>, size: usize) -> bool;
    fn robomaster_msgs__msg__ArmorHit__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmorHit>);
    fn robomaster_msgs__msg__ArmorHit__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmorHit>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmorHit>) -> bool;
}

// Corresponds to robomaster_msgs__msg__ArmorHit
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A hit event detected by one of the sensors placed behind the LEDs.
/// The Robomaster detects two kind of hits: collision using microphones,
/// and infrared beams using infrared receivers.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmorHit {
    /// header with the time stamp of the hit
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__ArmorHit__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__ArmorHit__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmorHit {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ArmorHit__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ArmorHit__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ArmorHit__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmorHit {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmorHit where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/ArmorHit";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ArmorHit() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedLine() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__DetectedLine__init(msg: *mut DetectedLine) -> bool;
    fn robomaster_msgs__msg__DetectedLine__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedLine>, size: usize) -> bool;
    fn robomaster_msgs__msg__DetectedLine__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedLine>);
    fn robomaster_msgs__msg__DetectedLine__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedLine>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedLine>) -> bool;
}

// Corresponds to robomaster_msgs__msg__DetectedLine
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Information about a line detected in the camera frame

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__DetectedLine__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__DetectedLine__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedLine {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedLine__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedLine__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedLine__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedLine {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedLine where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/DetectedLine";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedLine() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedGesture() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__DetectedGesture__init(msg: *mut DetectedGesture) -> bool;
    fn robomaster_msgs__msg__DetectedGesture__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedGesture>, size: usize) -> bool;
    fn robomaster_msgs__msg__DetectedGesture__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedGesture>);
    fn robomaster_msgs__msg__DetectedGesture__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedGesture>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedGesture>) -> bool;
}

// Corresponds to robomaster_msgs__msg__DetectedGesture
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Information about an gesture detected using vision

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedGesture {
    /// bounding box
    pub roi: super::super::msg::rmw::RegionOfInterest,

    /// the name of the gesture
    pub kind: rosidl_runtime_rs::String,

}



impl Default for DetectedGesture {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__DetectedGesture__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__DetectedGesture__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedGesture {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedGesture__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedGesture__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedGesture__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedGesture {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedGesture where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/DetectedGesture";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedGesture() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedMarker() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__DetectedMarker__init(msg: *mut DetectedMarker) -> bool;
    fn robomaster_msgs__msg__DetectedMarker__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedMarker>, size: usize) -> bool;
    fn robomaster_msgs__msg__DetectedMarker__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedMarker>);
    fn robomaster_msgs__msg__DetectedMarker__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedMarker>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedMarker>) -> bool;
}

// Corresponds to robomaster_msgs__msg__DetectedMarker
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Information about a marker detected using vision

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedMarker {
    /// bounding box
    pub roi: super::super::msg::rmw::RegionOfInterest,

    /// the name of the marker
    pub kind: rosidl_runtime_rs::String,

}



impl Default for DetectedMarker {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__DetectedMarker__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__DetectedMarker__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedMarker {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedMarker__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedMarker__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedMarker__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedMarker {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedMarker where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/DetectedMarker";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedMarker() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedPerson() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__DetectedPerson__init(msg: *mut DetectedPerson) -> bool;
    fn robomaster_msgs__msg__DetectedPerson__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedPerson>, size: usize) -> bool;
    fn robomaster_msgs__msg__DetectedPerson__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedPerson>);
    fn robomaster_msgs__msg__DetectedPerson__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedPerson>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedPerson>) -> bool;
}

// Corresponds to robomaster_msgs__msg__DetectedPerson
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Information about a person detected using vision

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedPerson {
    /// bounding box
    pub roi: super::super::msg::rmw::RegionOfInterest,

}



impl Default for DetectedPerson {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__DetectedPerson__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__DetectedPerson__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedPerson {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedPerson__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedPerson__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedPerson__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedPerson {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedPerson where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/DetectedPerson";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedPerson() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedRobot() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__DetectedRobot__init(msg: *mut DetectedRobot) -> bool;
    fn robomaster_msgs__msg__DetectedRobot__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DetectedRobot>, size: usize) -> bool;
    fn robomaster_msgs__msg__DetectedRobot__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DetectedRobot>);
    fn robomaster_msgs__msg__DetectedRobot__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DetectedRobot>, out_seq: *mut rosidl_runtime_rs::Sequence<DetectedRobot>) -> bool;
}

// Corresponds to robomaster_msgs__msg__DetectedRobot
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Information about a robot detected using vision

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DetectedRobot {
    /// bounding box
    pub roi: super::super::msg::rmw::RegionOfInterest,

}



impl Default for DetectedRobot {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__DetectedRobot__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__DetectedRobot__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DetectedRobot {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedRobot__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedRobot__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__DetectedRobot__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DetectedRobot {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DetectedRobot where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/DetectedRobot";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__DetectedRobot() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Detection() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__Detection__init(msg: *mut Detection) -> bool;
    fn robomaster_msgs__msg__Detection__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Detection>, size: usize) -> bool;
    fn robomaster_msgs__msg__Detection__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Detection>);
    fn robomaster_msgs__msg__Detection__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Detection>, out_seq: *mut rosidl_runtime_rs::Sequence<Detection>) -> bool;
}

// Corresponds to robomaster_msgs__msg__Detection
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Summary of vision detections

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Detection {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

    /// A list of people detections
    pub people: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedPerson>,

    /// A list of robots detections
    pub robots: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedRobot>,

    /// A list of gestures detections
    pub gestures: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedGesture>,

    /// A list of lines detections
    pub lines: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedLine>,

    /// A list of markers detections
    pub markers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::DetectedMarker>,

}



impl Default for Detection {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__Detection__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__Detection__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Detection {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Detection__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Detection__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Detection__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Detection {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Detection where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/Detection";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Detection() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__RegionOfInterest() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__RegionOfInterest__init(msg: *mut RegionOfInterest) -> bool;
    fn robomaster_msgs__msg__RegionOfInterest__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RegionOfInterest>, size: usize) -> bool;
    fn robomaster_msgs__msg__RegionOfInterest__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RegionOfInterest>);
    fn robomaster_msgs__msg__RegionOfInterest__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RegionOfInterest>, out_seq: *mut rosidl_runtime_rs::Sequence<RegionOfInterest>) -> bool;
}

// Corresponds to robomaster_msgs__msg__RegionOfInterest
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A region of interest (ROS) in an image
/// All values are in [0, 1] and are relative to image width and height

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__RegionOfInterest__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__RegionOfInterest__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RegionOfInterest {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__RegionOfInterest__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__RegionOfInterest__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__RegionOfInterest__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RegionOfInterest {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RegionOfInterest where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/RegionOfInterest";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__RegionOfInterest() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Serial() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__Serial__init(msg: *mut Serial) -> bool;
    fn robomaster_msgs__msg__Serial__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Serial>, size: usize) -> bool;
    fn robomaster_msgs__msg__Serial__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Serial>);
    fn robomaster_msgs__msg__Serial__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Serial>, out_seq: *mut rosidl_runtime_rs::Sequence<Serial>) -> bool;
}

// Corresponds to robomaster_msgs__msg__Serial
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Encapsulates serial communication received or to be sent through UART

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Serial {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

    /// the payload
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for Serial {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__Serial__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__Serial__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Serial {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Serial__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Serial__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Serial__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Serial {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Serial where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/Serial";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Serial() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ServoCommand() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__ServoCommand__init(msg: *mut ServoCommand) -> bool;
    fn robomaster_msgs__msg__ServoCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoCommand>, size: usize) -> bool;
    fn robomaster_msgs__msg__ServoCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoCommand>);
    fn robomaster_msgs__msg__ServoCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoCommand>) -> bool;
}

// Corresponds to robomaster_msgs__msg__ServoCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Command to control a servo in speed

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__ServoCommand__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__ServoCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoCommand where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/ServoCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ServoCommand() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__GimbalCommand() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__GimbalCommand__init(msg: *mut GimbalCommand) -> bool;
    fn robomaster_msgs__msg__GimbalCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GimbalCommand>, size: usize) -> bool;
    fn robomaster_msgs__msg__GimbalCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GimbalCommand>);
    fn robomaster_msgs__msg__GimbalCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GimbalCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<GimbalCommand>) -> bool;
}

// Corresponds to robomaster_msgs__msg__GimbalCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Command to control the gripper in speed

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GimbalCommand {
    /// target yaw angular speed in rad/m
    pub yaw_speed: f32,

    /// target pitch angular speed in rad/m
    pub pitch_speed: f32,

}



impl Default for GimbalCommand {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__GimbalCommand__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__GimbalCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GimbalCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GimbalCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GimbalCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__GimbalCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GimbalCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GimbalCommand where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/GimbalCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__GimbalCommand() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__BlasterLED() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__BlasterLED__init(msg: *mut BlasterLED) -> bool;
    fn robomaster_msgs__msg__BlasterLED__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BlasterLED>, size: usize) -> bool;
    fn robomaster_msgs__msg__BlasterLED__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BlasterLED>);
    fn robomaster_msgs__msg__BlasterLED__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BlasterLED>, out_seq: *mut rosidl_runtime_rs::Sequence<BlasterLED>) -> bool;
}

// Corresponds to robomaster_msgs__msg__BlasterLED
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Command to control the blaster LED

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BlasterLED {
    /// relative intensity of the LED: 0.0: off, 1.0: full brightness
    pub brightness: f32,

}



impl Default for BlasterLED {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__BlasterLED__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__BlasterLED__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BlasterLED {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__BlasterLED__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__BlasterLED__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__BlasterLED__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BlasterLED {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BlasterLED where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/BlasterLED";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__BlasterLED() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Mode() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__Mode__init(msg: *mut Mode) -> bool;
    fn robomaster_msgs__msg__Mode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Mode>, size: usize) -> bool;
    fn robomaster_msgs__msg__Mode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Mode>);
    fn robomaster_msgs__msg__Mode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Mode>, out_seq: *mut rosidl_runtime_rs::Sequence<Mode>) -> bool;
}

// Corresponds to robomaster_msgs__msg__Mode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Coupling control model between gimbal and chassis.

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__Mode__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__Mode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Mode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Mode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Mode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__Mode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Mode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Mode where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/Mode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__Mode() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioData() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__AudioData__init(msg: *mut AudioData) -> bool;
    fn robomaster_msgs__msg__AudioData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioData>, size: usize) -> bool;
    fn robomaster_msgs__msg__AudioData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioData>);
    fn robomaster_msgs__msg__AudioData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioData>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioData>) -> bool;
}

// Corresponds to robomaster_msgs__msg__AudioData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Raw audio samples
/// .. note::
///   Unlike ROS1, ROS2 has no ``audio_common_msgs``, so for now we define our own format
///   to publish the raw 16-bit 48Khz audio stream from the camera mono microphone

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioData {
    /// the timestamp and frame of the microphone recording
    pub header: std_msgs::msg::rmw::Header,

    /// an array of 16-bit audio samples
    pub data: rosidl_runtime_rs::Sequence<i16>,

}



impl Default for AudioData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__AudioData__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__AudioData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioData where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/AudioData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioData() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioLevel() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__AudioLevel__init(msg: *mut AudioLevel) -> bool;
    fn robomaster_msgs__msg__AudioLevel__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioLevel>, size: usize) -> bool;
    fn robomaster_msgs__msg__AudioLevel__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioLevel>);
    fn robomaster_msgs__msg__AudioLevel__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioLevel>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioLevel>) -> bool;
}

// Corresponds to robomaster_msgs__msg__AudioLevel
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Sound level
/// .. note::
///   Unlike ROS1, ROS2 has no ``audio_common_msgs``, so we define our own format
///   for sound level in dBFS

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioLevel {
    /// the timestamp and frame of the microphone recording
    pub header: std_msgs::msg::rmw::Header,

    /// sound level in dBFS
    pub level: f32,

}



impl Default for AudioLevel {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__AudioLevel__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__AudioLevel__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioLevel {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioLevel__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioLevel__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioLevel__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioLevel {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioLevel where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/AudioLevel";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioLevel() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioOpus() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__AudioOpus__init(msg: *mut AudioOpus) -> bool;
    fn robomaster_msgs__msg__AudioOpus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioOpus>, size: usize) -> bool;
    fn robomaster_msgs__msg__AudioOpus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioOpus>);
    fn robomaster_msgs__msg__AudioOpus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioOpus>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioOpus>) -> bool;
}

// Corresponds to robomaster_msgs__msg__AudioOpus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Encapsulate a packet from an OPUS encoded audio stream

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioOpus {
    /// header with the time stamp and origin of the packet
    pub header: std_msgs::msg::rmw::Header,

    /// sequential number of the packet
    pub seq: u64,

    /// Opus data
    pub buffer: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for AudioOpus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__AudioOpus__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__AudioOpus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioOpus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioOpus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioOpus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__AudioOpus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioOpus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioOpus where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/AudioOpus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__AudioOpus() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__H264Packet() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__H264Packet__init(msg: *mut H264Packet) -> bool;
    fn robomaster_msgs__msg__H264Packet__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<H264Packet>, size: usize) -> bool;
    fn robomaster_msgs__msg__H264Packet__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<H264Packet>);
    fn robomaster_msgs__msg__H264Packet__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<H264Packet>, out_seq: *mut rosidl_runtime_rs::Sequence<H264Packet>) -> bool;
}

// Corresponds to robomaster_msgs__msg__H264Packet
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Encapsulate a packet from a H264 encoded video stream

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct H264Packet {
    /// header with the time stamp and origin of the packet
    pub header: std_msgs::msg::rmw::Header,

    /// sequential number of the packet
    pub seq: u64,

    /// H264 data
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for H264Packet {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__H264Packet__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__H264Packet__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for H264Packet {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__H264Packet__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__H264Packet__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__H264Packet__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for H264Packet {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for H264Packet where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/H264Packet";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__H264Packet() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SensorAdapter() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__SensorAdapter__init(msg: *mut SensorAdapter) -> bool;
    fn robomaster_msgs__msg__SensorAdapter__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SensorAdapter>, size: usize) -> bool;
    fn robomaster_msgs__msg__SensorAdapter__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SensorAdapter>);
    fn robomaster_msgs__msg__SensorAdapter__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SensorAdapter>, out_seq: *mut rosidl_runtime_rs::Sequence<SensorAdapter>) -> bool;
}

// Corresponds to robomaster_msgs__msg__SensorAdapter
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Summarizes the state of up to 6 sensor adapters (each with 2 interfaces)

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SensorAdapter {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__SensorAdapter__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__SensorAdapter__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SensorAdapter {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SensorAdapter__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SensorAdapter__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SensorAdapter__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SensorAdapter {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SensorAdapter where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/SensorAdapter";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SensorAdapter() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ServoRawState() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__ServoRawState__init(msg: *mut ServoRawState) -> bool;
    fn robomaster_msgs__msg__ServoRawState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ServoRawState>, size: usize) -> bool;
    fn robomaster_msgs__msg__ServoRawState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ServoRawState>);
    fn robomaster_msgs__msg__ServoRawState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ServoRawState>, out_seq: *mut rosidl_runtime_rs::Sequence<ServoRawState>) -> bool;
}

// Corresponds to robomaster_msgs__msg__ServoRawState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Raw state from the servos on ports {0, 1, 2, 3}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ServoRawState {
    /// header with the time stamp of the update
    pub header: std_msgs::msg::rmw::Header,

    /// whenever a servo in connected to a port
    pub valid: [bool; 4],

    /// the angle of the servos (1024 corresponds to 180 degrees)
    pub value: [i32; 4],

    /// the speed of the servos (1024 corresponds to about 36 degrees per seconds)
    pub speed: [i32; 4],

}



impl Default for ServoRawState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__ServoRawState__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__ServoRawState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ServoRawState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoRawState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoRawState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__ServoRawState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ServoRawState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ServoRawState where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/ServoRawState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__ServoRawState() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SpeakerCommand() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__msg__SpeakerCommand__init(msg: *mut SpeakerCommand) -> bool;
    fn robomaster_msgs__msg__SpeakerCommand__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpeakerCommand>, size: usize) -> bool;
    fn robomaster_msgs__msg__SpeakerCommand__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpeakerCommand>);
    fn robomaster_msgs__msg__SpeakerCommand__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpeakerCommand>, out_seq: *mut rosidl_runtime_rs::Sequence<SpeakerCommand>) -> bool;
}

// Corresponds to robomaster_msgs__msg__SpeakerCommand
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// An command to stop or play a predefined sound or an audio file from the speaker.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeakerCommand {
    /// ID of the sound to play
    pub sound_id: i32,

    /// how many times to play the same sound (0: infinite loop)
    pub times: u8,

    /// Control command: one of ``CONTROL_STOP``, ``CONTROL_PLAY``, ``CONTROL_ADD``
    pub control: u8,

    /// if not empry, the local file to be played.
    pub file: rosidl_runtime_rs::String,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__msg__SpeakerCommand__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__msg__SpeakerCommand__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpeakerCommand {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SpeakerCommand__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SpeakerCommand__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__msg__SpeakerCommand__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpeakerCommand {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpeakerCommand where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/msg/SpeakerCommand";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__msg__SpeakerCommand() }
  }
}


