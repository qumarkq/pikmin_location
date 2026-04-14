use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("無法連接到底層服務 (usbmuxd)。請確認是否已安裝並啟動。")]
    DaemonConnectionFailed,

    #[error("讀取設備列表失敗: {0}")]
    ScanFailed(String),

    #[error("未知錯誤: {0}")]
    Unknown(String),
}
