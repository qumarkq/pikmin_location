use serde::Serialize;
use std::path::Path;
use tauri::command;

// 定義回傳給前端的標準資料結構
#[derive(Serialize)]
pub struct MountResult {
    pub success: bool,
    pub message: String,
}

// 定義未來要使用的 iOS 設備結構 (與 discovery 模組共用)
#[derive(Serialize)]
pub struct IosDevice {
    pub udid: String,
    pub connection_type: String,
}

/// 檢查並掛載 DDI 映像檔的 Tauri Command
/// 此函數會接收前端傳來的 UDID 與 iOS 版本字串
#[command]
pub async fn check_and_mount_ddi(udid: String, ios_version: String) -> Result<MountResult, String> {
    println!("準備為設備 {} 掛載 iOS {} 映像檔...", udid, ios_version);

    // 1. 定義 DDI 檔案的預期路徑 (相對於 src-tauri 目錄)
    // 我們將映像檔放在 src-tauri/ddi/{ios_version}/ 目錄下
    let base_ddi_dir = format!("./ddi/{}", ios_version);
    let dmg_path = format!("{}/DeveloperDiskImage.dmg", base_ddi_dir);
    let sig_path = format!("{}/DeveloperDiskImage.dmg.signature", base_ddi_dir);

    // 2. 嚴格的路徑與檔案存在性檢查
    let dmg_exists = Path::new(&dmg_path).exists();
    let sig_exists = Path::new(&sig_path).exists();

    if !dmg_exists || !sig_exists {
        let error_msg = format!(
            "找不到 iOS {} 的開發者映像檔。\n請確保以下檔案存在：\n1. {}\n2. {}",
            ios_version, dmg_path, sig_path
        );
        return Err(error_msg);
    }

    // 3. 模擬底層 libimobiledevice 掛載邏輯 (未來實作區塊)
    // TODO: 在此處呼叫 C FFI 建立 IDEVICE 連線，並啟動 com.apple.mobile.image_mounter 服務
    // 目前使用延遲來模擬掛載過程 (讓前端 UI 有足夠的反應時間顯示 loading)
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("設備 {} 掛載 DDI 成功！", udid);

    // 4. 回傳成功狀態給前端
    Ok(MountResult {
        success: true,
        message: format!("iOS {} 映像檔掛載成功，simulatelocation 服務已解鎖。", ios_version),
    })
}