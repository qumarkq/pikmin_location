import { invoke } from "@tauri-apps/api/core";

interface IosDevice {
  udid: string;
  connection_type: string;
}

interface MountResult {
  success: boolean;
  message: string;
}

const scanBtn = document.getElementById("scan-btn") as HTMLButtonElement;
const deviceListDiv = document.getElementById("device-list") as HTMLDivElement;
const mountBtn = document.getElementById("mount-btn") as HTMLButtonElement;
const mountStatusDiv = document.getElementById("mount-status") as HTMLDivElement;

// 宣告一個變數來記錄目前選中的設備 UDID
let currentTargetUdid = "";

// 掃描按鈕邏輯
scanBtn.addEventListener("click", async () => {
  deviceListDiv.innerHTML = "掃描中，請稍候...";
  mountBtn.disabled = true; // 掃描時先鎖住掛載按鈕
  mountStatusDiv.innerHTML = "";
  
  try {
    const devices: IosDevice[] = await invoke("get_connected_devices");
    
    if (devices.length === 0) {
      deviceListDiv.innerHTML = "未偵測到任何設備，請檢查 USB 連線。";
      return;
    }

    // 紀錄第一台掃描到的設備，供後續掛載使用
    currentTargetUdid = devices[0].udid;
    // 掃描成功，解鎖掛載按鈕
    mountBtn.disabled = false; 

    deviceListDiv.innerHTML = devices.map(dev => `
      <div class="device-item">
        <strong>UDID:</strong> ${dev.udid} <br/>
        <strong>狀態:</strong> ${dev.connection_type}
      </div>
    `).join("");

  } catch (error) {
    deviceListDiv.innerHTML = `<span style="color: red;">掃描失敗: ${error}</span>`;
  }
});

// 掛載按鈕邏輯
mountBtn.addEventListener("click", async () => {
  if (!currentTargetUdid) return;

  mountStatusDiv.innerHTML = "正在檢查並掛載 DDI 映像檔，請稍候...";
  mountStatusDiv.style.color = "black";
  mountBtn.disabled = true; // 避免重複點擊

  try {
    // 呼叫 Rust 的 check_and_mount_ddi
    const result: MountResult = await invoke("check_and_mount_ddi", {
      udid: currentTargetUdid,
      iosVersion: "16.5" // 這是我們剛剛建立假資料夾的版本
    });

    if (result.success) {
      mountStatusDiv.innerHTML = `✅ ${result.message}`;
      mountStatusDiv.style.color = "green";
    } else {
      mountStatusDiv.innerHTML = `❌ ${result.message}`;
      mountStatusDiv.style.color = "red";
    }
  } catch (error) {
    // 捕捉 Rust 拋出的 Err(String)
    mountStatusDiv.innerHTML = `❌ 掛載失敗:\n${error}`;
    mountStatusDiv.style.color = "red";
  } finally {
    mountBtn.disabled = false; // 恢復按鈕狀態
  }
});