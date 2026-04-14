mod domain;
mod error;
mod discovery;
mod ddi;
mod location; // 👈 引入新模組

use location::LocationState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    tauri::Builder::default()
        // 1. 初始化並託管 LocationState (存放取消令牌)
        .manage(LocationState {
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
        })
        // 2. 註冊所有指令，包含剛剛寫的 location 相關指令
        .invoke_handler(tauri::generate_handler![
            discovery::get_connected_devices,
            discovery::get_device_ios_version,
            ddi::check_ddi_exists,
            ddi::mount_ddi,
            location::set_location,
            location::start_movement,
            location::stop_movement
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 應用程式執行時發生致命錯誤");
}