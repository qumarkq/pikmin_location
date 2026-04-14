mod domain;
mod error;
mod discovery;
mod ddi;
mod location;

use location::LocationState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .manage(LocationState {
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![
            discovery::get_connected_devices,
            discovery::get_device_ios_version,
            // ddi::check_ddi_exists, // 👈 刪除這一行，因為它不存在
            ddi::mount_ddi,
            location::set_location,
            location::start_movement,
            location::stop_movement
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 應用程式執行時發生致命錯誤");
}