use crate::domain::{GpsCoordinate, MovementConfig, OperationResult};
use crate::error::AppError;
use rusty_libimobiledevice::idevice;
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tauri::{AppHandle, Emitter, State};

// 儲存每個 UDID 對應的取消令牌
pub struct LocationState {
    pub active_tasks: Arc<Mutex<HashMap<String, CancellationToken>>>,
}

/// 內部共用：發送座標到 iOS 設備的底層實作
async fn send_location_to_device(udid: &str, lat: f64, lon: f64) -> Result<(), AppError> {
    if lat < -90.0 || lat > 90.0 || lon < -180.0 || lon > 180.0 {
        return Err(AppError::InvalidCoordinate { lat, lon });
    }

    let udid_clone = udid.to_string();
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let device = match idevice::get_device(&udid_clone) {
            Ok(d) => d,
            Err(_) => return Err(AppError::DeviceUnresponsive { udid: udid_clone.clone() }),
        };
        
        let mut lockdownd = match LockdowndClient::new(&device, "pikmin_location") {
            Ok(l) => l,
            Err(e) => return Err(AppError::Internal(format!("Lockdownd 連線失敗: {:?}", e))),
        };

        // 啟動定位模擬服務
        let _service = match lockdownd.start_service("com.apple.dt.simulatelocation", false) {
            Ok(s) => s,
            Err(_) => return Err(AppError::LocationServiceNotReady),
        };

        // TODO: 這裡需實作向 _service socket 寫入大端序 (Big-Endian) 座標字串的邏輯
        // 格式通常為：[32-bit length] + "1\0\0\0" + length + lat.to_string() + "," + lon.to_string()
        
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("執行緒崩潰: {:?}", e)))??;

    Ok(())
}

#[tauri::command]
pub async fn set_location(udid: String, coordinate: GpsCoordinate) -> Result<OperationResult, AppError> {
    send_location_to_device(&udid, coordinate.latitude, coordinate.longitude).await?;
    
    Ok(OperationResult {
        success: true,
        message: format!("已設定座標: {}, {}", coordinate.latitude, coordinate.longitude),
    })
}

#[tauri::command]
pub async fn start_movement(
    app: AppHandle,
    state: State<'_, LocationState>,
    udid: String,
    config: MovementConfig,
) -> Result<OperationResult, AppError> {
    if config.waypoints.is_empty() {
        return Err(AppError::Internal("路徑點不能為空".to_string()));
    }

    let token = CancellationToken::new();
    
    {
        let mut tasks = state.active_tasks.lock().await;
        if let Some(old_token) = tasks.insert(udid.clone(), token.clone()) {
            old_token.cancel();
        }
    }

    let udid_clone = udid.clone();
    
    tokio::spawn(async move {
        let mut current_idx = 0;
        let waypoints = config.waypoints;
        
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    let _ = app.emit("movement://stopped", format!("{} 移動被手動取消", udid_clone));
                    break;
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                    let target = &waypoints[current_idx];
                    
                    if let Ok(_) = send_location_to_device(&udid_clone, target.latitude, target.longitude).await {
                        let _ = app.emit("location://updated", target.clone());
                    }

                    current_idx += 1;
                    if current_idx >= waypoints.len() {
                        if config.loop_path {
                            current_idx = 0;
                        } else {
                            let _ = app.emit("movement://stopped", format!("{} 移動已到達終點", udid_clone));
                            break;
                        }
                    }
                }
            }
        }
    });

    Ok(OperationResult {
        success: true,
        message: "開始路徑移動".to_string(),
    })
}

#[tauri::command]
pub async fn stop_movement(state: State<'_, LocationState>, udid: String) -> Result<OperationResult, AppError> {
    let mut tasks = state.active_tasks.lock().await;
    if let Some(token) = tasks.remove(&udid) {
        token.cancel();
        Ok(OperationResult {
            success: true,
            message: "已停止移動".to_string(),
        })
    } else {
        Ok(OperationResult {
            success: false,
            message: "沒有正在執行的移動任務".to_string(),
        })
    }
}