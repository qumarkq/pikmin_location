#![allow(dead_code)]

use crate::domain::OperationResult;
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::mobile_image_mounter::MobileImageMounter;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
struct DdiMountedPayload {
    udid: String,
    ios_version: String,
}

#[tauri::command]
pub async fn check_ddi_exists(ios_version: String) -> Result<bool, AppError> {
    let ddi_path = format!("./ddi/{}/DeveloperDiskImage.dmg", ios_version);
    let sig_path = format!("{}.signature", ddi_path);

    let ddi_exists = tokio::fs::try_exists(&ddi_path).await.unwrap_or(false);
    let sig_exists = tokio::fs::try_exists(&sig_path).await.unwrap_or(false);

    Ok(ddi_exists && sig_exists)
}

#[tauri::command]
pub async fn mount_ddi(
    app: AppHandle,
    udid: String,
    ios_version: String,
) -> Result<OperationResult, AppError> {
    let exists = check_ddi_exists(ios_version.clone()).await?;
    if !exists {
        return Err(AppError::DdiNotFound { version: ios_version });
    }

    let ddi_path = format!("./ddi/{}/DeveloperDiskImage.dmg", ios_version);
    let sig_path = format!("{}.signature", ddi_path);
    let udid_clone = udid.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let device = match idevice::get_device(&udid_clone) {
            Ok(d) => d,
            Err(_) => return Err(AppError::DeviceUnresponsive { udid: udid_clone.clone() }),
        };
        
        // 修正：加上 mut，因為 start_service 會修改 lockdownd 內部的連線狀態
        let mut lockdownd = match LockdowndClient::new(&device, "pikmin_location") {
            Ok(l) => l,
            Err(e) => return Err(AppError::Internal(format!("Lockdownd 連線失敗: {:?}", e))),
        };

        let service = match lockdownd.start_service("com.apple.mobile.mobile_image_mounter", true) {
            Ok(s) => s,
            Err(e) => return Err(AppError::Internal(format!("無法開啟掛載通訊協定: {:?}", e))),
        };

        let mounter = match MobileImageMounter::new(&device, service) {
            Ok(m) => m,
            Err(e) => return Err(AppError::Internal(format!("無法啟動 ImageMounter: {:?}", e))),
        };
        
        match mounter.mount_image(&ddi_path, &sig_path, "Developer") {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::DdiMountFailed(format!("{:?}", e))),
        }
    })
    .await
    .map_err(|e| AppError::Internal(format!("執行緒崩潰: {:?}", e)))??;

    let payload = DdiMountedPayload {
        udid: udid.clone(),
        ios_version: ios_version.clone(),
    };
    
    app.emit("ddi://mounted", payload)
        .map_err(|e| AppError::Internal(format!("Tauri 事件發送失敗: {}", e)))?;

    Ok(OperationResult {
        success: true,
        message: format!("iOS {} 開發者映像檔掛載成功", ios_version),
    })
}