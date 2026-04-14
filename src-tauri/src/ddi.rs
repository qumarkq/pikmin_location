use crate::domain::OperationResult;
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;
use rusty_libimobiledevice::services::mobile_image_mounter::MobileImageMounter;
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose};

#[tauri::command]
pub async fn mount_ddi(udid: String, ios_version: String) -> Result<OperationResult, AppError> {
    let udid_clone = udid.clone();
    let version_clone = ios_version.clone();

    tokio::task::spawn_blocking(move || {
        // 絕對動態：根據前端傳來的版本號去找資料夾
        let base_path = format!("/Users/quma/Documents/ddi/{}", version_clone);
        let dmg_path = Path::new(&base_path).join("DeveloperDiskImage.dmg");
        let sig_path = Path::new(&base_path).join("DeveloperDiskImage.dmg.signature");

        if !dmg_path.exists() || !sig_path.exists() {
            return Err(AppError::DdiNotFound { ios_version: version_clone });
        }

        let image_bytes = fs::read(&dmg_path)?;
        let signature_bytes = fs::read(&sig_path)?;
        
        // 核心：轉換為 Base64 字串傳輸
        let img_b64 = general_purpose::STANDARD.encode(image_bytes);
        let sig_b64 = general_purpose::STANDARD.encode(signature_bytes);

        let device = idevice::get_device(&udid_clone)
            .map_err(|_| AppError::DeviceUnresponsive { udid: udid_clone })?;
        
        let mut lockdownd = LockdowndClient::new(&device, "pikmin")
            .map_err(|e| AppError::Internal(format!("Lockdownd 失敗: {:?}", e)))?;

        let service = lockdownd.start_service("com.apple.mobile.developer_images_mounter", false)
            .map_err(|_| AppError::Internal("無法啟動掛載服務，請確認手機已信任此電腦".to_string()))?;

        let mounter = MobileImageMounter::new(&device, service)
            .map_err(|e| AppError::Internal(format!("Mounter 初始化失敗: {:?}", e)))?;

        mounter.mount_image(img_b64, sig_b64, "Developer")
            .map_err(|e| AppError::Internal(format!("掛載過程失敗: {:?}", e)))?;

        Ok(OperationResult {
            success: true,
            message: format!("iOS {} 映像檔掛載完成", version_clone),
        })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}