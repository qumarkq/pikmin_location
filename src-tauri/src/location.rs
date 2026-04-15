use crate::domain::{GpsCoordinate, MovementConfig, OperationResult};
use crate::error::AppError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use std::process::Command;

pub struct LocationState {
    pub active_tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

#[tauri::command]
pub async fn set_location(udid: String, coord: GpsCoordinate) -> Result<OperationResult, AppError> {
    if coord.latitude < -90.0 || coord.latitude > 90.0 {
        return Err(AppError::InvalidCoordinate { lat: coord.latitude, lon: coord.longitude });
    }

    tokio::task::spawn_blocking(move || {
        let lat_str = coord.latitude.to_string();
        let lon_str = coord.longitude.to_string();

        // 呼叫 pymobiledevice3 修改座標
        let output = Command::new("python3")
            .args([
                "-m", "pymobiledevice3", 
                "developer", "simulate-location", "set",
                "--udid", &udid,
                &lat_str, &lon_str
            ])
            .output()
            .map_err(|e| AppError::CliExecutionFailed(e.to_string()))?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::Internal(format!("修改座標失敗: {}", err_msg)));
        }

        Ok(OperationResult {
            success: true,
            message: format!("座標已成功同步至 {}, {}", lat_str, lon_str),
        })
    })
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?
}

#[tauri::command]
pub async fn start_movement(
    _state: tauri::State<'_, LocationState>, 
    udid: String, 
    _config: MovementConfig
) -> Result<OperationResult, AppError> {
    Ok(OperationResult {
        success: true,
        message: format!("設備 {} 準備開始移動模擬", udid),
    })
}

#[tauri::command]
pub async fn stop_movement(
    state: tauri::State<'_, LocationState>, 
    udid: String
) -> Result<OperationResult, AppError> {
    let mut tasks = state.active_tasks.lock().await;
    if let Some(handle) = tasks.remove(&udid) {
        handle.abort();
        
        // 呼叫 pymobiledevice3 停止模擬
        let _ = Command::new("python3")
            .args(["-m", "pymobiledevice3", "developer", "simulate-location", "clear", "--udid", &udid])
            .output();

        return Ok(OperationResult { success: true, message: "已停止移動並清除虛擬定位".to_string() });
    }
    Err(AppError::LocationServiceNotReady)
}