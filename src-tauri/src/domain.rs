use serde::{Deserialize, Serialize};

/// 連線類型定義
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    Usb,
    Network,
}

/// iOS 裝置領域模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosDevice {
    pub udid: String,
    pub connection_type: ConnectionType,
    pub name: Option<String>,      // 裝置自訂名稱，如 "iPhone 15 Pro"
    pub ios_version: Option<String>, // 系統版本，如 "17.4.1"
}

/// GPS 座標結構
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

/// 路徑移動配置
#[derive(Debug, Clone, Deserialize)]
pub struct MovementConfig {
    pub waypoints: Vec<GpsCoordinate>,
    pub speed_kmh: f64,   // 移動速度 (0.1 ~ 30.0)
    pub loop_path: bool,  // 是否循環路徑
}

/// 通用操作結果回傳
#[derive(Debug, Serialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}