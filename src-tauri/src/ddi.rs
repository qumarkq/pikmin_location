use crate::domain::OperationResult;
use crate::error::AppError;
use std::process::Command;

#[tauri::command]
pub async fn mount_ddi(udid: String, ios_version: String) -> Result<OperationResult, AppError> {
    tokio::task::spawn_blocking(move || {
        println!("LOG: 啟動 pymobiledevice3 自動掛載流程...");

        let output = Command::new("python3")
            .args([
                "-m", "pymobiledevice3", 
                "mounter", "auto-mount", 
                "--udid", &udid
            ])
            .output()
            .map_err(|e| AppError::CliExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("掛載失敗: {}", err_msg)));
        }

        Ok(OperationResult {
            success: true,
            message: format!("iOS {} 掛載完成 (via pymobiledevice3)", ios_version),
        })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}