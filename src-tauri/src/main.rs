// 宣告使用到的模組
mod ddi;

// 如果你的 Milestone 2 有實作 discovery 模組，請取消註解下面這行
mod discovery; 

// 為了讓這份 main.rs 能獨立編譯，我們在此處放置一個假的 get_connected_devices
// (如果你已經有獨立的 discovery.rs，請刪除這個假函數，並取消上面的 mod discovery 註解)
#[tauri::command]
fn get_connected_devices() -> Result<Vec<ddi::IosDevice>, String> {
    Ok(vec![
        ddi::IosDevice {
            udid: "00008110-00123456789ABCDE".to_string(), // 假 UDID
            connection_type: "USB".to_string(),
        }
    ])
}

fn main() {
    tauri::Builder::default()
        // 將我們所有的 Command 註冊給前端使用
        .invoke_handler(tauri::generate_handler![
            get_connected_devices,
            ddi::check_and_mount_ddi
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 應用程式執行時發生致命錯誤");
}