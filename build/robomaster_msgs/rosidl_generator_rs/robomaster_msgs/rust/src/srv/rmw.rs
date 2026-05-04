#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetADC_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetADC_Request__init(msg: *mut GetADC_Request) -> bool;
    fn robomaster_msgs__srv__GetADC_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetADC_Request>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetADC_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetADC_Request>);
    fn robomaster_msgs__srv__GetADC_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetADC_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetADC_Request>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetADC_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetADC_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetADC_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetADC_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetADC_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetADC_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetADC_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetADC_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetADC_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetADC_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetADC_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetADC_Response__init(msg: *mut GetADC_Response) -> bool;
    fn robomaster_msgs__srv__GetADC_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetADC_Response>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetADC_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetADC_Response>);
    fn robomaster_msgs__srv__GetADC_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetADC_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetADC_Response>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetADC_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetADC_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the raw value read by the ADC
    pub value: u16,

}



impl Default for GetADC_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetADC_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetADC_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetADC_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetADC_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetADC_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetADC_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetADC_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetADC_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetIO_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetIO_Request__init(msg: *mut GetIO_Request) -> bool;
    fn robomaster_msgs__srv__GetIO_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetIO_Request>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetIO_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetIO_Request>);
    fn robomaster_msgs__srv__GetIO_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetIO_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetIO_Request>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetIO_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetIO_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetIO_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetIO_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetIO_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetIO_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetIO_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetIO_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetIO_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetIO_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetIO_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetIO_Response__init(msg: *mut GetIO_Response) -> bool;
    fn robomaster_msgs__srv__GetIO_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetIO_Response>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetIO_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetIO_Response>);
    fn robomaster_msgs__srv__GetIO_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetIO_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetIO_Response>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetIO_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetIO_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the pin state (0 or 1)
    pub value: u8,

}



impl Default for GetIO_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetIO_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetIO_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetIO_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetIO_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetIO_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetIO_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetIO_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetIO_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetPulse_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetPulse_Request__init(msg: *mut GetPulse_Request) -> bool;
    fn robomaster_msgs__srv__GetPulse_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Request>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetPulse_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Request>);
    fn robomaster_msgs__srv__GetPulse_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPulse_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Request>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetPulse_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPulse_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetPulse_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetPulse_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetPulse_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPulse_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPulse_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPulse_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetPulse_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetPulse_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetPulse_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetPulse_Response__init(msg: *mut GetPulse_Response) -> bool;
    fn robomaster_msgs__srv__GetPulse_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Response>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetPulse_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Response>);
    fn robomaster_msgs__srv__GetPulse_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPulse_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPulse_Response>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetPulse_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPulse_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the pulse duration in milliseconds
    pub time_ms: u32,

}



impl Default for GetPulse_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetPulse_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetPulse_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPulse_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetPulse_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPulse_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPulse_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetPulse_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetPulse_Response() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetServoAngle_Request() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetServoAngle_Request__init(msg: *mut GetServoAngle_Request) -> bool;
    fn robomaster_msgs__srv__GetServoAngle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Request>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetServoAngle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Request>);
    fn robomaster_msgs__srv__GetServoAngle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetServoAngle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Request>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetServoAngle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetServoAngle_Request {
    /// ID of the servo (between 0 and 2)
    pub index: u8,

}



impl Default for GetServoAngle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetServoAngle_Request__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetServoAngle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetServoAngle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetServoAngle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetServoAngle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetServoAngle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetServoAngle_Request() }
  }
}


#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetServoAngle_Response() -> *const std::ffi::c_void;
}

#[link(name = "robomaster_msgs__rosidl_generator_c")]
extern "C" {
    fn robomaster_msgs__srv__GetServoAngle_Response__init(msg: *mut GetServoAngle_Response) -> bool;
    fn robomaster_msgs__srv__GetServoAngle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Response>, size: usize) -> bool;
    fn robomaster_msgs__srv__GetServoAngle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Response>);
    fn robomaster_msgs__srv__GetServoAngle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetServoAngle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetServoAngle_Response>) -> bool;
}

// Corresponds to robomaster_msgs__srv__GetServoAngle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetServoAngle_Response {
    /// absolute angle in rad
    pub angle: f32,

}



impl Default for GetServoAngle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !robomaster_msgs__srv__GetServoAngle_Response__init(&mut msg as *mut _) {
        panic!("Call to robomaster_msgs__srv__GetServoAngle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetServoAngle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { robomaster_msgs__srv__GetServoAngle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetServoAngle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetServoAngle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "robomaster_msgs/srv/GetServoAngle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__robomaster_msgs__srv__GetServoAngle_Response() }
  }
}






#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetADC() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__srv__GetADC
#[allow(missing_docs, non_camel_case_types)]
pub struct GetADC;

impl rosidl_runtime_rs::Service for GetADC {
    type Request = GetADC_Request;
    type Response = GetADC_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetADC() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetIO() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__srv__GetIO
#[allow(missing_docs, non_camel_case_types)]
pub struct GetIO;

impl rosidl_runtime_rs::Service for GetIO {
    type Request = GetIO_Request;
    type Response = GetIO_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetIO() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetPulse() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__srv__GetPulse
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPulse;

impl rosidl_runtime_rs::Service for GetPulse {
    type Request = GetPulse_Request;
    type Response = GetPulse_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetPulse() }
    }
}




#[link(name = "robomaster_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetServoAngle() -> *const std::ffi::c_void;
}

// Corresponds to robomaster_msgs__srv__GetServoAngle
#[allow(missing_docs, non_camel_case_types)]
pub struct GetServoAngle;

impl rosidl_runtime_rs::Service for GetServoAngle {
    type Request = GetServoAngle_Request;
    type Response = GetServoAngle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__robomaster_msgs__srv__GetServoAngle() }
    }
}


