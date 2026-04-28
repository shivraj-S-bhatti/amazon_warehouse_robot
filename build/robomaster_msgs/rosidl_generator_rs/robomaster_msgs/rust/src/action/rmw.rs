
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_Goal__init(msg: *mut Move_Goal) -> bool;
    fn robomaster_msgs__action__Move_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_Goal>);
    fn robomaster_msgs__action__Move_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_Goal {
    /// target relative position
    /// (longitudinal component with positive towards front)
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,

    /// target relative angle in rad
    pub theta: f32,

    /// maximal linear speed in m/s
    pub linear_speed: f32,

    /// maximal angular speed in rad/s (default is 30 deg/s)
    pub angular_speed: f32,

}



impl Default for Move_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_Result__init(msg: *mut Move_Result) -> bool;
    fn robomaster_msgs__action__Move_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_Result>);
    fn robomaster_msgs__action__Move_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Move_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_Feedback__init(msg: *mut Move_Feedback) -> bool;
    fn robomaster_msgs__action__Move_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_Feedback>);
    fn robomaster_msgs__action__Move_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_Feedback {
    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for Move_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_FeedbackMessage__init(msg: *mut Move_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__Move_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_FeedbackMessage>);
    fn robomaster_msgs__action__Move_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Move_Feedback,

}



impl Default for Move_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_Goal__init(msg: *mut MoveArm_Goal) -> bool;
    fn robomaster_msgs__action__MoveArm_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Goal>);
    fn robomaster_msgs__action__MoveArm_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_Goal {
    /// target x-coordinate in meter (x-axis points forwards)
    pub x: f32,

    /// target z-coordinate in meter (z-axis points upwards)
    pub z: f32,

    /// whenever the coordinates are relative or absolute.
    pub relative: bool,

}



impl Default for MoveArm_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_Result__init(msg: *mut MoveArm_Result) -> bool;
    fn robomaster_msgs__action__MoveArm_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Result>);
    fn robomaster_msgs__action__MoveArm_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MoveArm_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_Feedback__init(msg: *mut MoveArm_Feedback) -> bool;
    fn robomaster_msgs__action__MoveArm_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Feedback>);
    fn robomaster_msgs__action__MoveArm_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_Feedback {
    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for MoveArm_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_FeedbackMessage__init(msg: *mut MoveArm_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_FeedbackMessage>);
    fn robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveArm_Feedback,

}



impl Default for MoveArm_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_Goal__init(msg: *mut GripperControl_Goal) -> bool;
    fn robomaster_msgs__action__GripperControl_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Goal>);
    fn robomaster_msgs__action__GripperControl_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Goal {
    /// the target gripper state:
    /// one of ``PAUSE``, ``OPEN``, or ``CLOSE``
    pub target_state: u8,

    /// the relative amount of power in [0, 1]
    pub power: f32,

}

impl GripperControl_Goal {
    /// the gripper state when manually stopped
    pub const PAUSE: u8 = 0;

    /// the open gripper state
    pub const OPEN: u8 = 1;

    /// the closed gripper state
    pub const CLOSE: u8 = 2;

}


impl Default for GripperControl_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_Result__init(msg: *mut GripperControl_Result) -> bool;
    fn robomaster_msgs__action__GripperControl_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Result>);
    fn robomaster_msgs__action__GripperControl_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Result {
    /// the duration of the action
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for GripperControl_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_Feedback__init(msg: *mut GripperControl_Feedback) -> bool;
    fn robomaster_msgs__action__GripperControl_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Feedback>);
    fn robomaster_msgs__action__GripperControl_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Feedback {
    /// the current gripper state:
    /// one of ``PAUSE``, ``OPEN``, or ``CLOSE``
    pub current_state: u8,

}



impl Default for GripperControl_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_FeedbackMessage__init(msg: *mut GripperControl_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_FeedbackMessage>);
    fn robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::GripperControl_Feedback,

}



impl Default for GripperControl_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_Goal__init(msg: *mut PlaySound_Goal) -> bool;
    fn robomaster_msgs__action__PlaySound_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Goal>);
    fn robomaster_msgs__action__PlaySound_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Goal {
    /// ID of the sound to play
    pub sound_id: i32,

    /// how many times to play the same sound
    pub times: u8,

    /// if not empty, the local file to be played.
    pub file: rosidl_runtime_rs::String,

}



impl Default for PlaySound_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_Result__init(msg: *mut PlaySound_Result) -> bool;
    fn robomaster_msgs__action__PlaySound_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Result>);
    fn robomaster_msgs__action__PlaySound_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for PlaySound_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_Feedback__init(msg: *mut PlaySound_Feedback) -> bool;
    fn robomaster_msgs__action__PlaySound_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Feedback>);
    fn robomaster_msgs__action__PlaySound_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_Feedback {
    /// the current count of sound repetitions
    pub time: u8,

    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for PlaySound_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_FeedbackMessage__init(msg: *mut PlaySound_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_FeedbackMessage>);
    fn robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::PlaySound_Feedback,

}



impl Default for PlaySound_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_Goal__init(msg: *mut MoveServo_Goal) -> bool;
    fn robomaster_msgs__action__MoveServo_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Goal>);
    fn robomaster_msgs__action__MoveServo_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_Goal {
    /// ID of the servo (between 0 and 2)
    pub index: u8,

    /// absolute target angle in rad
    pub angle: f32,

}



impl Default for MoveServo_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_Result__init(msg: *mut MoveServo_Result) -> bool;
    fn robomaster_msgs__action__MoveServo_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Result>);
    fn robomaster_msgs__action__MoveServo_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MoveServo_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_Feedback__init(msg: *mut MoveServo_Feedback) -> bool;
    fn robomaster_msgs__action__MoveServo_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Feedback>);
    fn robomaster_msgs__action__MoveServo_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_Feedback {
    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for MoveServo_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_FeedbackMessage__init(msg: *mut MoveServo_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_FeedbackMessage>);
    fn robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveServo_Feedback,

}



impl Default for MoveServo_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_Goal__init(msg: *mut MoveGimbal_Goal) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Goal>);
    fn robomaster_msgs__action__MoveGimbal_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_Goal {
    /// target yaw in rad
    pub yaw: f32,

    /// target pitch in rad
    pub pitch: f32,

    /// maximal yaw rotation speed in rad/s
    pub yaw_speed: f32,

    /// maximal pitch rotation speed in rad/s
    pub pitch_speed: f32,

    /// the frame in which target orientation is specified.
    /// One of ``FIXED``, ``GIMBAL``, ``CHASSIS_GIMBAL_PITCH``,
    /// or ``CHASSIS_FIXED_PITCH``
    pub frame: u8,

}

impl MoveGimbal_Goal {
    /// Frame: gravity aligned, origin set at boot
    /// (same frame as chassis odometry and imu)
    pub const FIXED: u8 = 0;

    /// Frame: attached to gimbal, useful to send relative movements.
    pub const GIMBAL: u8 = 1;

    /// Frame: yaw aligned to chassis, pitch as in GIMBAL
    pub const CHASSIS_GIMBAL_PITCH: u8 = 2;

    /// Frame: yaw aligned to chassis, pitch as in FIXED
    pub const CHASSIS_FIXED_PITCH: u8 = 3;

}


impl Default for MoveGimbal_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_Result__init(msg: *mut MoveGimbal_Result) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Result>);
    fn robomaster_msgs__action__MoveGimbal_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MoveGimbal_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_Feedback__init(msg: *mut MoveGimbal_Feedback) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Feedback>);
    fn robomaster_msgs__action__MoveGimbal_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_Feedback {
    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for MoveGimbal_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_FeedbackMessage__init(msg: *mut MoveGimbal_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_FeedbackMessage>);
    fn robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MoveGimbal_Feedback,

}



impl Default for MoveGimbal_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_FeedbackMessage() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Goal() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_Goal__init(msg: *mut RecenterGimbal_Goal) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Goal>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Goal>);
    fn robomaster_msgs__action__RecenterGimbal_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Goal>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_Goal {
    /// maximal yaw rotation speed in rad/s
    pub yaw_speed: f32,

    /// maximal pitch rotation speed in rad/s
    pub pitch_speed: f32,

}



impl Default for RecenterGimbal_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_Goal__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Goal() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Result() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_Result__init(msg: *mut RecenterGimbal_Result) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Result>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Result>);
    fn robomaster_msgs__action__RecenterGimbal_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Result>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RecenterGimbal_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_Result__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_Result where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Result() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_Feedback__init(msg: *mut RecenterGimbal_Feedback) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Feedback>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Feedback>);
    fn robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_Feedback>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_Feedback {
    /// the relative progress towards goal from 0 to 1
    pub progress: f32,

}



impl Default for RecenterGimbal_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_Feedback__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_Feedback() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_FeedbackMessage__init(msg: *mut RecenterGimbal_FeedbackMessage) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_FeedbackMessage>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_FeedbackMessage>);
    fn robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_FeedbackMessage>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::RecenterGimbal_Feedback,

}



impl Default for RecenterGimbal_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_FeedbackMessage() }
  }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_SendGoal_Request__init(msg: *mut Move_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__Move_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Request>);
    fn robomaster_msgs__action__Move_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Move_Goal,

}



impl Default for Move_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_SendGoal_Response__init(msg: *mut Move_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__Move_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Response>);
    fn robomaster_msgs__action__Move_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Move_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_GetResult_Request__init(msg: *mut Move_GetResult_Request) -> bool;
    fn robomaster_msgs__action__Move_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Request>);
    fn robomaster_msgs__action__Move_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Move_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__Move_GetResult_Response__init(msg: *mut Move_GetResult_Response) -> bool;
    fn robomaster_msgs__action__Move_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__Move_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Response>);
    fn robomaster_msgs__action__Move_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Move_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Move_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__Move_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Move_Result,

}



impl Default for Move_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__Move_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__Move_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Move_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__Move_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Move_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Move_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/Move_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__Move_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_SendGoal_Request__init(msg: *mut MoveArm_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Request>);
    fn robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveArm_Goal,

}



impl Default for MoveArm_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_SendGoal_Response__init(msg: *mut MoveArm_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Response>);
    fn robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveArm_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_GetResult_Request__init(msg: *mut MoveArm_GetResult_Request) -> bool;
    fn robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Request>);
    fn robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveArm_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveArm_GetResult_Response__init(msg: *mut MoveArm_GetResult_Response) -> bool;
    fn robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Response>);
    fn robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveArm_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveArm_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveArm_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveArm_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveArm_Result,

}



impl Default for MoveArm_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveArm_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveArm_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveArm_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveArm_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveArm_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveArm_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveArm_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveArm_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_SendGoal_Request__init(msg: *mut GripperControl_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Request>);
    fn robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::GripperControl_Goal,

}



impl Default for GripperControl_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_SendGoal_Response__init(msg: *mut GripperControl_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Response>);
    fn robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for GripperControl_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_GetResult_Request__init(msg: *mut GripperControl_GetResult_Request) -> bool;
    fn robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Request>);
    fn robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for GripperControl_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__GripperControl_GetResult_Response__init(msg: *mut GripperControl_GetResult_Response) -> bool;
    fn robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Response>);
    fn robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__GripperControl_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::GripperControl_Result,

}



impl Default for GripperControl_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__GripperControl_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__GripperControl_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__GripperControl_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/GripperControl_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__GripperControl_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_SendGoal_Request__init(msg: *mut PlaySound_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Request>);
    fn robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::PlaySound_Goal,

}



impl Default for PlaySound_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_SendGoal_Response__init(msg: *mut PlaySound_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Response>);
    fn robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for PlaySound_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_GetResult_Request__init(msg: *mut PlaySound_GetResult_Request) -> bool;
    fn robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Request>);
    fn robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for PlaySound_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__PlaySound_GetResult_Response__init(msg: *mut PlaySound_GetResult_Response) -> bool;
    fn robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Response>);
    fn robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PlaySound_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<PlaySound_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__PlaySound_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaySound_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::PlaySound_Result,

}



impl Default for PlaySound_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__PlaySound_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__PlaySound_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PlaySound_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__PlaySound_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PlaySound_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PlaySound_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/PlaySound_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__PlaySound_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_SendGoal_Request__init(msg: *mut MoveServo_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Request>);
    fn robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveServo_Goal,

}



impl Default for MoveServo_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_SendGoal_Response__init(msg: *mut MoveServo_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Response>);
    fn robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveServo_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_GetResult_Request__init(msg: *mut MoveServo_GetResult_Request) -> bool;
    fn robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Request>);
    fn robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveServo_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveServo_GetResult_Response__init(msg: *mut MoveServo_GetResult_Response) -> bool;
    fn robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Response>);
    fn robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveServo_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveServo_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveServo_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveServo_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveServo_Result,

}



impl Default for MoveServo_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveServo_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveServo_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveServo_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveServo_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveServo_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveServo_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveServo_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveServo_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Request__init(msg: *mut MoveGimbal_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Request>);
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MoveGimbal_Goal,

}



impl Default for MoveGimbal_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Response__init(msg: *mut MoveGimbal_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Response>);
    fn robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MoveGimbal_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_GetResult_Request__init(msg: *mut MoveGimbal_GetResult_Request) -> bool;
    fn robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Request>);
    fn robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MoveGimbal_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__MoveGimbal_GetResult_Response__init(msg: *mut MoveGimbal_GetResult_Response) -> bool;
    fn robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Response>);
    fn robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveGimbal_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveGimbal_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MoveGimbal_Result,

}



impl Default for MoveGimbal_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__MoveGimbal_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__MoveGimbal_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveGimbal_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__MoveGimbal_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveGimbal_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveGimbal_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/MoveGimbal_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Request__init(msg: *mut RecenterGimbal_SendGoal_Request) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Request>);
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::RecenterGimbal_Goal,

}



impl Default for RecenterGimbal_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Response__init(msg: *mut RecenterGimbal_SendGoal_Response) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Response>);
    fn robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_SendGoal_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for RecenterGimbal_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Request__init(msg: *mut RecenterGimbal_GetResult_Request) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Request>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Request>);
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Request>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for RecenterGimbal_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Response__init(msg: *mut RecenterGimbal_GetResult_Response) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Response>, size: usize) -> bool;
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Response>);
    fn robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RecenterGimbal_GetResult_Response>) -> bool;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RecenterGimbal_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::RecenterGimbal_Result,

}



impl Default for RecenterGimbal_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__action__RecenterGimbal_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__action__RecenterGimbal_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RecenterGimbal_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__action__RecenterGimbal_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RecenterGimbal_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RecenterGimbal_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/action/RecenterGimbal_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult_Response() }
  }
}






#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__Move_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__Move_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Move_SendGoal;

impl rosidl_runtime_rs::Service for Move_SendGoal {
    type Request = Move_SendGoal_Request;
    type Response = Move_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__Move_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__Move_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__Move_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Move_GetResult;

impl rosidl_runtime_rs::Service for Move_GetResult {
    type Request = Move_GetResult_Request;
    type Response = Move_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__Move_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveArm_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveArm_SendGoal;

impl rosidl_runtime_rs::Service for MoveArm_SendGoal {
    type Request = MoveArm_SendGoal_Request;
    type Response = MoveArm_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveArm_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveArm_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveArm_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveArm_GetResult;

impl rosidl_runtime_rs::Service for MoveArm_GetResult {
    type Request = MoveArm_GetResult_Request;
    type Response = MoveArm_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveArm_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__GripperControl_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperControl_SendGoal;

impl rosidl_runtime_rs::Service for GripperControl_SendGoal {
    type Request = GripperControl_SendGoal_Request;
    type Response = GripperControl_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__GripperControl_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__GripperControl_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__GripperControl_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperControl_GetResult;

impl rosidl_runtime_rs::Service for GripperControl_GetResult {
    type Request = GripperControl_GetResult_Request;
    type Response = GripperControl_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__GripperControl_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__PlaySound_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct PlaySound_SendGoal;

impl rosidl_runtime_rs::Service for PlaySound_SendGoal {
    type Request = PlaySound_SendGoal_Request;
    type Response = PlaySound_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__PlaySound_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__PlaySound_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__PlaySound_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct PlaySound_GetResult;

impl rosidl_runtime_rs::Service for PlaySound_GetResult {
    type Request = PlaySound_GetResult_Request;
    type Response = PlaySound_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__PlaySound_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveServo_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveServo_SendGoal;

impl rosidl_runtime_rs::Service for MoveServo_SendGoal {
    type Request = MoveServo_SendGoal_Request;
    type Response = MoveServo_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveServo_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveServo_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveServo_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveServo_GetResult;

impl rosidl_runtime_rs::Service for MoveServo_GetResult {
    type Request = MoveServo_GetResult_Request;
    type Response = MoveServo_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveServo_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGimbal_SendGoal;

impl rosidl_runtime_rs::Service for MoveGimbal_SendGoal {
    type Request = MoveGimbal_SendGoal_Request;
    type Response = MoveGimbal_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveGimbal_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__MoveGimbal_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveGimbal_GetResult;

impl rosidl_runtime_rs::Service for MoveGimbal_GetResult {
    type Request = MoveGimbal_GetResult_Request;
    type Response = MoveGimbal_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__MoveGimbal_GetResult() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct RecenterGimbal_SendGoal;

impl rosidl_runtime_rs::Service for RecenterGimbal_SendGoal {
    type Request = RecenterGimbal_SendGoal_Request;
    type Response = RecenterGimbal_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__RecenterGimbal_SendGoal() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__action__RecenterGimbal_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct RecenterGimbal_GetResult;

impl rosidl_runtime_rs::Service for RecenterGimbal_GetResult {
    type Request = RecenterGimbal_GetResult_Request;
    type Response = RecenterGimbal_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__action__RecenterGimbal_GetResult() }
    }
}


