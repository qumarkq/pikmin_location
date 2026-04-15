mod domain;
mod error;
mod ddi;
mod discovery;
mod location;

use location::LocationState; // 現在 location.rs 裡有 pub LocationState 了
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
            ddi::mount_ddi,      // 👈 現在 ddi.rs 有標籤了
            location::set_location,
            location::start_movement, // 👈 現在 location.rs 有標籤了
            location::stop_movement   // 👈 現在 location.rs 有標籤了
        ])
        .run(tauri::generate_context!())
        .expect("致命錯誤");
}