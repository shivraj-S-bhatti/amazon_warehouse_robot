#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to robomaster_msgs__srv__GetADC_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetADC_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetADC_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetADC_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetADC_Request {
  type RmwMsg = super::srv::rmw::GetADC_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        port: msg.port,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      port: msg.port,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      port: msg.port,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetADC_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetADC_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the raw value read by the ADC
    pub value: u16,

}



impl Default for GetADC_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetADC_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetADC_Response {
  type RmwMsg = super::srv::rmw::GetADC_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      value: msg.value,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetIO_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetIO_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetIO_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetIO_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetIO_Request {
  type RmwMsg = super::srv::rmw::GetIO_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        port: msg.port,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      port: msg.port,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      port: msg.port,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetIO_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetIO_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the pin state (0 or 1)
    pub value: u8,

}



impl Default for GetIO_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetIO_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetIO_Response {
  type RmwMsg = super::srv::rmw::GetIO_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      value: msg.value,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetPulse_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPulse_Request {
    /// 0 or 1: ID of one of two interfaces of a sensor adapter
    pub id: u8,

    /// 0, 1, 2, or 3: ID of one of the 4 sensor adapter that can be connected to the robot
    pub port: u8,

}



impl Default for GetPulse_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPulse_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPulse_Request {
  type RmwMsg = super::srv::rmw::GetPulse_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        port: msg.port,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      port: msg.port,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      port: msg.port,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetPulse_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPulse_Response {
    /// whenever the sensor adapter is enabled
    pub valid: bool,

    /// the pulse duration in milliseconds
    pub time_ms: u32,

}



impl Default for GetPulse_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPulse_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPulse_Response {
  type RmwMsg = super::srv::rmw::GetPulse_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        valid: msg.valid,
        time_ms: msg.time_ms,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      valid: msg.valid,
      time_ms: msg.time_ms,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      valid: msg.valid,
      time_ms: msg.time_ms,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetServoAngle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetServoAngle_Request {
    /// ID of the servo (between 0 and 2)
    pub index: u8,

}



impl Default for GetServoAngle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetServoAngle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetServoAngle_Request {
  type RmwMsg = super::srv::rmw::GetServoAngle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        index: msg.index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      index: msg.index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      index: msg.index,
    }
  }
}


// Corresponds to robomaster_msgs__srv__GetServoAngle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetServoAngle_Response {
    /// absolute angle in rad
    pub angle: f32,

}



impl Default for GetServoAngle_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetServoAngle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetServoAngle_Response {
  type RmwMsg = super::srv::rmw::GetServoAngle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        angle: msg.angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      angle: msg.angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      angle: msg.angle,
    }
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


