mod domain;
mod error;
mod discovery;

use tracing::Level;

fn main() {
    // 1. 初始化日誌系統，方便我們在終端機監控底層運作
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    tracing::info!("Pikmin Location 桌面端啟動中...");

    // 2. 接管主執行緒，啟動 Tauri 應用程式生命週期
    tauri::Builder::default()
        // 將剛剛寫好的 Command 註冊進 Tauri 的呼叫處理器中
        .invoke_handler(tauri::generate_handler![
            discovery::get_connected_devices
        ])
        .run(tauri::generate_context!())
        .expect("啟動 Tauri 應用程式時發生錯誤");
}