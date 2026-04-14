use crate::domain::IosDevice;
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;

#[tauri::command]
pub async fn get_connected_devices() -> Result<Vec<IosDevice>, AppError> {
    let devices = idevice::get_devices()
        .map_err(|_| AppError::DaemonUnavailable)?;

    let mut result: Vec<IosDevice> = Vec::new();

    for dev in devices {
        let udid = dev.get_udid();
        let ios_version = get_device_ios_version(udid.clone()).await.ok();
        
        result.push(IosDevice {
            udid: udid.clone(),
            connection_type: crate::domain::ConnectionType::Usb,
            name: None,
            ios_version,
        });
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_device_ios_version(udid: String) -> Result<String, AppError> {
    let device = idevice::get_device(&udid)
        .map_err(|_| AppError::DeviceUnresponsive { udid: udid.clone() })?;
    
    // 修正：移除 mut，因為 get_value 只需要不可變引用
    let lockdown = LockdowndClient::new(&device, "pikmin")
        .map_err(|e| AppError::Internal(format!("Lockdownd 失敗: {:?}", e)))?;

    let version_plist = lockdown.get_value("", "ProductVersion")
        .map_err(|e| AppError::Internal(format!("無法獲取版本: {:?}", e)))?;

    // 使用 to_string() 取得內容字串
    let version_str = version_plist.to_string();

    if version_str.is_empty() {
        return Err(AppError::Internal("抓到的版本號為空".to_string()));
    }

    Ok(version_str)
}