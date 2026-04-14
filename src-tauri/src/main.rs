mod domain;
mod error;
mod discovery;
mod ddi;

fn main() {
    tauri::Builder::default()
        // 2. 註冊所有可供前端呼叫的 Tauri 指令
        // 注意路徑：指名道姓說清楚是哪個模組底下的指令
        .invoke_handler(tauri::generate_handler![
            discovery::get_connected_devices,
            ddi::check_and_mount_ddi
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 應用程式執行時發生致命錯誤");
}