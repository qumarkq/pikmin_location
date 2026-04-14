use crate::domain::{ConnectionType, IosDevice};
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;

/// 取得目前所有連接的 iOS 設備
#[tauri::command]
pub async fn get_connected_devices() -> Result<Vec<IosDevice>, AppError> {
    let devices = tokio::task::spawn_blocking(|| -> Result<Vec<IosDevice>, AppError> {
        let mut result_list = Vec::new();

        let device_list = match idevice::get_devices() {
            Ok(list) => list,
            Err(e) => return Err(AppError::DaemonUnavailable(format!("usbmuxd 掃描失敗: {}", e))),
        };

        for device in device_list {
            let udid = device.get_udid();
            let connection_type = ConnectionType::Usb;

            // 修正 1：依照編譯器建議，改用 LockdowndClient::new
            let (name, ios_version) = match LockdowndClient::new(&device, "pikmin_location") {
                Ok(client) => {
                    // 修正 2：捨棄 .ok() 簡寫，使用完整的 match 強制確立型別為 Option<String>
                    let dev_name: Option<String> = match client.get_device_name() {
                        Ok(n) => Some(n),
                        Err(_) => None,
                    };
                    
                    // 修正 3：同上，捨棄任何 map 或 ok() 閉包
                    let version: Option<String> = match client.get_value("ProductVersion", "") {
                        Ok(node) => {
                            // 這裡透過明確宣告變數 s 幫助編譯器確立 to_string() 的目標型別
                            let s: String = node.to_string();
                            Some(s)
                        },
                        Err(_) => None,
                    };

                    (dev_name, version)
                },
                Err(_) => (None, None),
            };

            result_list.push(IosDevice {
                udid,
                connection_type,
                name,
                ios_version,
            });
        }

        Ok(result_list)
    })
    .await
    .map_err(|e| AppError::Internal(format!("執行緒崩潰: {}", e)))??;

    Ok(devices)
}

/// 查詢指定裝置的 iOS 版本
#[tauri::command]
pub async fn get_device_ios_version(udid: String) -> Result<String, AppError> {
    let devices = get_connected_devices().await?;
    
    let device = devices.into_iter().find(|d| d.udid == udid)
        .ok_or_else(|| AppError::DeviceUnresponsive { udid: udid.clone() })?;

    device.ios_version.ok_or_else(|| {
        AppError::Internal("無法讀取設備版本，請確認手機已解鎖並信任此電腦".to_string())
    })
}