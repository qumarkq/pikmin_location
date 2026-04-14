import { invoke } from "@tauri-apps/api/core";

interface IosDevice {
  udid: string;
  connection_type: string;
}

const scanBtn = document.getElementById("scan-btn") as HTMLButtonElement;
const deviceListDiv = document.getElementById("device-list") as HTMLDivElement;

scanBtn.addEventListener("click", async () => {
  deviceListDiv.innerHTML = "掃描中，請稍候...";
  
  try {
    const devices: IosDevice[] = await invoke("get_connected_devices");
    
    if (devices.length === 0) {
      deviceListDiv.innerHTML = "未偵測到任何設備，請檢查連線。";
      return;
    }

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