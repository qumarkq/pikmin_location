use crate::domain::{ConnectionType, IosDevice};
use crate::error::DeviceError;
use rusty_libimobiledevice::idevice;
use tracing::{debug, error, info};

// 嚴格審計點：這裡必須是 pub struct
pub struct DeviceDiscoveryService;

impl DeviceDiscoveryService {
    // 嚴格審計點：這裡必須是 pub fn
    pub fn scan_devices() -> Result<Vec<IosDevice>, DeviceError> {
        debug!("開始掃描 iOS 設備...");

        // 呼叫底層 C API
        let raw_devices = match idevice::get_devices() {
            Ok(devices) => devices,
            Err(e) => {
                error!("底層 API 呼叫失敗: {:?}", e);
                return Err(DeviceError::DaemonConnectionFailed);
            }
        };

        if raw_devices.is_empty() {
            info!("目前無設備連接。");
            return Ok(vec![]);
        }

        // 將第三方結構映射為我們自己的 Domain 結構
        let mut parsed_devices = Vec::new();
        for device in raw_devices {
            let udid = device.get_udid().to_string();
            
            // 邏輯審計：依據底層 API 判斷連線類型
            let conn_type = if device.get_network() {
                ConnectionType::Network
            } else {
                ConnectionType::Usb
            };

            parsed_devices.push(IosDevice {
                udid,
                connection_type: conn_type,
            });
        }

        Ok(parsed_devices)
    }
}

#[tauri::command]
pub fn get_connected_devices() -> Result<Vec<IosDevice>, String> {
    // 呼叫我們已經寫好的底層掃描邏輯
    match DeviceDiscoveryService::scan_devices() {
        Ok(devices) => Ok(devices),
        // 如果底層拋出 DeviceError，我們將其轉成字串 (String) 傳給前端
        // 因為 Tauri 預設不支援自定義錯誤的跨語言傳遞，字串是最安全的做法
        Err(e) => Err(e.to_string()),
    }
}