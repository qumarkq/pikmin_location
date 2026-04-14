use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)] // 新增 Serialize
pub enum ConnectionType {
    Usb,
    Network,
    Unknown,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConnectionType::Usb => "USB 連線",
            ConnectionType::Network => "網路連線",
            ConnectionType::Unknown => "未知連線",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize)] // 新增 Serialize
pub struct IosDevice {
    pub udid: String,
    pub connection_type: ConnectionType,
}