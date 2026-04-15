use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("設備無回應 (UDID: {udid})")]
    DeviceUnresponsive { udid: String },
    
    #[error("無法連接至 usbmuxd，請確認設備已插上並信任")]
    DaemonUnavailable,
    
    #[error("找不到 DDI 檔案。版本：{ios_version}")]
    DdiNotFound { ios_version: String },
    
    #[error("Python CLI 執行失敗: {0}。請確認已執行 pip3 install pymobiledevice3")]
    CliExecutionFailed(String),
    
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