use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("無法連接至 usbmuxd 服務: {0}")]
    DaemonUnavailable(String),

    #[error("設備 {udid} 未回應")]
    DeviceUnresponsive { udid: String },

    #[error("找不到 iOS {version} 的 DDI 映像檔")]
    DdiNotFound { version: String },

    #[error("DDI 掛載失敗: {0}")]
    DdiMountFailed(String),

    #[error("GPS 模擬服務未就緒，請先掛載 DDI")]
    LocationServiceNotReady,

    #[error("座標超出有效範圍: lat={lat}, lon={lon}")]
    InvalidCoordinate { lat: f64, lon: f64 },

    #[error("內部錯誤: {0}")]
    Internal(String),
}