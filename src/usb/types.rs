//! USB 类型定义 - 用于 Lua 传输的数据结构

use std::time::Duration;

/// USB 设备信息（可序列化给 Lua）
#[derive(Debug, Clone)]
pub struct UsbDeviceInfo {
    pub bus_number: u8,
    pub address: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub protocol_code: u8,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
    pub speed: String,
    pub num_configurations: u8,
}

/// USB 配置信息
#[derive(Debug, Clone)]
pub struct UsbConfigInfo {
    pub number: u8,
    pub num_interfaces: u8,
    pub max_power_ma: u16,
    pub self_powered: bool,
    pub remote_wakeup: bool,
    pub description: Option<String>,
}

/// USB 接口信息
#[derive(Debug, Clone)]
pub struct UsbInterfaceInfo {
    pub number: u8,
    pub class_code: u8,
    pub subclass_code: u8,
    pub protocol_code: u8,
    pub num_endpoints: u8,
    pub description: Option<String>,
    pub alternate_settings: Vec<UsbAltSettingInfo>,
}

/// USB 备用设置信息
#[derive(Debug, Clone)]
pub struct UsbAltSettingInfo {
    pub number: u8,
    pub class_code: u8,
    pub subclass_code: u8,
    pub protocol_code: u8,
    pub num_endpoints: u8,
    pub endpoints: Vec<UsbEndpointInfo>,
}

/// USB 端点信息
#[derive(Debug, Clone)]
pub struct UsbEndpointInfo {
    pub address: u8,
    pub number: u8,
    pub direction: String,      // "in" or "out"
    pub transfer_type: String,  // "control", "isochronous", "bulk", "interrupt"
    pub sync_type: String,      // "none", "async", "adaptive", "sync"
    pub usage_type: String,     // "data", "feedback", "implicit_feedback"
    pub max_packet_size: u16,
    pub interval: u8,
}

/// USB 传输方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDirection {
    In,
    Out,
}

impl UsbDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            UsbDirection::In => "in",
            UsbDirection::Out => "out",
        }
    }
}

/// USB 传输类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbTransferType {
    Control,
    Isochronous,
    Bulk,
    Interrupt,
}

impl UsbTransferType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UsbTransferType::Control => "control",
            UsbTransferType::Isochronous => "isochronous",
            UsbTransferType::Bulk => "bulk",
            UsbTransferType::Interrupt => "interrupt",
        }
    }
}

/// 控制传输请求类型
#[derive(Debug, Clone, Copy)]
pub struct UsbRequestType {
    pub direction: UsbDirection,
    pub request_type: u8,  // 0=Standard, 1=Class, 2=Vendor
    pub recipient: u8,     // 0=Device, 1=Interface, 2=Endpoint, 3=Other
}

impl UsbRequestType {
    pub fn to_byte(&self) -> u8 {
        let dir = match self.direction {
            UsbDirection::In => 0x80,
            UsbDirection::Out => 0x00,
        };
        dir | ((self.request_type & 0x03) << 5) | (self.recipient & 0x1f)
    }
}

/// 默认超时时间
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(1);

/// USB 错误类型
#[derive(Debug, Clone)]
pub enum UsbError {
    Io(String),
    InvalidParam(String),
    Access(String),
    NoDevice,
    NotFound,
    Busy,
    Timeout,
    Overflow,
    Pipe,
    Interrupted,
    NoMem,
    NotSupported,
    BadDescriptor,
    Other(String),
}

impl std::fmt::Display for UsbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UsbError::Io(s) => write!(f, "IO error: {}", s),
            UsbError::InvalidParam(s) => write!(f, "Invalid parameter: {}", s),
            UsbError::Access(s) => write!(f, "Access denied: {}", s),
            UsbError::NoDevice => write!(f, "No such device"),
            UsbError::NotFound => write!(f, "Entity not found"),
            UsbError::Busy => write!(f, "Resource busy"),
            UsbError::Timeout => write!(f, "Operation timed out"),
            UsbError::Overflow => write!(f, "Overflow"),
            UsbError::Pipe => write!(f, "Pipe error"),
            UsbError::Interrupted => write!(f, "Interrupted"),
            UsbError::NoMem => write!(f, "Insufficient memory"),
            UsbError::NotSupported => write!(f, "Operation not supported"),
            UsbError::BadDescriptor => write!(f, "Bad descriptor"),
            UsbError::Other(s) => write!(f, "USB error: {}", s),
        }
    }
}

