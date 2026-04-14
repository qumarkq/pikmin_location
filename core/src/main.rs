mod discovery;
mod domain;
mod error;

use crate::discovery::DeviceDiscoveryService;
use tracing::{Level, error, info};

fn main() {
    // 1. 初始化結構化日誌系統
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Core 啟動中...");

    // 2. 執行掃描服務
    match DeviceDiscoveryService::scan_devices() {
        Ok(devices) => {
            info!("掃描完成，共找到 {} 台設備。", devices.len());
            for dev in devices {
                info!(
                    "發現設備 -> UDID: {}, 類型: {}",
                    dev.udid, dev.connection_type
                );
            }
        }
        Err(e) => {
            // 發生錯誤時，能清楚知道是 Domain Error 還是底層 Error
            error!("設備掃描失敗: {}", e);
        }
    }
}
