import "leaflet/dist/leaflet.css";
import { Commands, GpsCoordinate, AppError } from "./ipc/commands";
import { Events } from "./ipc/events";
import { MapModule } from "./ui/map";

const map = new MapModule();
const scanBtn = document.getElementById("scan-btn") as HTMLButtonElement;
const mountBtn = document.getElementById("mount-btn") as HTMLButtonElement;
const deviceListDiv = document.getElementById("device-list") as HTMLDivElement;
const mountStatusDiv = document.getElementById("mount-status") as HTMLDivElement;

let selectedUdid = "";
let selectedVersion = "";

/**
 * 嚴格審計版錯誤解析：絕對不准噴出 [object Object]
 */
function parseError(e: any): string {
  console.error("IPC Error Details:", e);
  if (typeof e === 'object' && e !== null && 'message' in e) {
    return String(e.message);
  }
  return typeof e === 'string' ? e : JSON.stringify(e);
}

scanBtn.addEventListener("click", async () => {
  scanBtn.disabled = true;
  deviceListDiv.innerText = "掃描中...";
  try {
    const devices = await Commands.getConnectedDevices();
    if (devices.length === 0) {
      deviceListDiv.innerText = "未發現設備";
      return;
    }
    const dev = devices[0];
    selectedUdid = dev.udid;
    selectedVersion = dev.ios_version || "16.0";
    mountBtn.disabled = false;
    deviceListDiv.innerHTML = `<strong>${dev.udid}</strong><br/>iOS ${selectedVersion}`;
  } catch (e) {
    deviceListDiv.innerText = `掃描出錯: ${parseError(e)}`;
  } finally {
    scanBtn.disabled = false;
  }
});

mountBtn.addEventListener("click", async () => {
  mountBtn.disabled = true;
  mountStatusDiv.innerText = "⏳ 掛載中...";
  try {
    // 這裡直接傳入掃描到的版本，不准硬編碼
    const res = await Commands.mountDdi(selectedUdid, selectedVersion);
    mountStatusDiv.innerText = `✅ ${res.message}`;
    mountStatusDiv.style.color = "green";
  } catch (e) {
    mountStatusDiv.innerText = `❌ ${parseError(e)}`;
    mountStatusDiv.style.color = "red";
  } finally {
    mountBtn.disabled = false;
  }
});