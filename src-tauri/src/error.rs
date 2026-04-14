use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("設備無回應: {udid}")]
    DeviceUnresponsive { udid: String },
    
    #[error("無法連接至 usbmuxd 守護進程，請確認已安裝 iTunes 或 Apple 裝置服務")]
    DaemonUnavailable, // 👈 補上這一行，解決 discovery.rs 的問題
    
    #[error("找不到 iOS {ios_version} 的 DDI 檔案，路徑應為 /Users/quma/Documents/ddi/{ios_version}")]
    DdiNotFound { ios_version: String },
    
    #[error("定位服務未就緒")]
    LocationServiceNotReady,
    
    #[error("無效的座標: {lat}, {lon}")]
    InvalidCoordinate { lat: f64, lon: f64 },
    
    #[error("內部錯誤: {0}")]
    Internal(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}