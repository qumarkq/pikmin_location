use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum ConnectionType {
    Usb,
    Network,
}

#[derive(Debug, Serialize)]
pub struct IosDevice {
    pub udid: String,
    pub connection_type: ConnectionType,
    pub name: Option<String>,
    pub ios_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct MovementConfig {
    pub speed: f64,
    // 預留其他參數，如路徑點
}