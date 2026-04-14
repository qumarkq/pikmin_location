#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Usb,
    Network, // Wi-Fi 或 iOS 17 的虛擬網路
    Unknown
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

/// 代表一台已連接的 iOS 設備
#[derive(Debug, Clone)]
pub struct IosDevice {
    pub udid: String,
    pub connection_type: ConnectionType,
}