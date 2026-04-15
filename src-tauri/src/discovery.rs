use crate::domain::IosDevice;
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;

#[tauri::command]
pub async fn get_connected_devices() -> Result<Vec<IosDevice>, AppError> {
    let devices = idevice::get_devices().map_err(|_| AppError::DaemonUnavailable)?;
    let mut result: Vec<IosDevice> = Vec::new();

    for dev in devices {
        let udid = dev.get_udid();
        
        let ios_version = match get_device_ios_version(udid.clone()).await {
            Ok(v) => v,
            Err(e) => format!("錯誤: {:?}", e),
        };
        
        result.push(IosDevice {
            udid: udid.clone(),
            connection_type: crate::domain::ConnectionType::Usb,
            name: None,
            ios_version: Some(ios_version),
        });
    }
    Ok(result)
}

#[tauri::command]
pub async fn get_device_ios_version(udid: String) -> Result<String, AppError> {
    let device = idevice::get_device(&udid)
        .map_err(|_| AppError::DeviceUnresponsive { udid: udid.clone() })?;
    
    let lockdown = LockdowndClient::new(&device, "pikmin")
        .map_err(|e| AppError::Internal(format!("Lockdown 連線失敗: {:?}", e)))?;

    // 🚨 核心破解：將 Domain 與 Key 皆設為 ""，強制獲取整包設備資訊 (Root Dictionary)
    let version_plist = lockdown.get_value("", "")
        .map_err(|e| AppError::Internal(format!("無法獲取設備資訊: {:?}", e)))?;

    // 將整包資訊轉為 XML 字串
    let xml_str = version_plist.to_string();

    // 🚨 字串精準切割：手動尋找 ProductVersion，徹底無視 Plist 物件轉換錯誤
    let version = if let Some(key_idx) = xml_str.find("<key>ProductVersion</key>") {
        let sub_str = &xml_str[key_idx..];
        // 尋找 key 下方的第一個 <string> 標籤
        if let Some(start) = sub_str.find("<string>") {
            let val_str = &sub_str[start + 8..]; // 8 是 "<string>" 的長度
            if let Some(end) = val_str.find("</string>") {
                val_str[..end].to_string() // 精準抽出 17.4.1
            } else {
                return Err(AppError::Internal("XML 解析失敗：找不到版本號結尾".to_string()));
            }
        } else {
            return Err(AppError::Internal("XML 解析失敗：找不到版本號開頭".to_string()));
        }
    } else {
        return Err(AppError::Internal("設備資訊中找不到 ProductVersion".to_string()));
    };

    Ok(version)
}