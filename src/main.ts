import { Commands, GpsCoordinate, AppError } from "./ipc/commands";
import { Events } from "./ipc/events";
import { MapModule } from "./ui/map";

// 1. 初始化地圖模組
const map = new MapModule();

// 2. 取得 UI 元素
const scanBtn = document.getElementById("scan-btn") as HTMLButtonElement;
const mountBtn = document.getElementById("mount-btn") as HTMLButtonElement;
const deviceListDiv = document.getElementById("device-list") as HTMLDivElement;
const mountStatusDiv = document.getElementById("mount-status") as HTMLDivElement;

const latInput = document.getElementById("input-lat") as HTMLInputElement;
const lonInput = document.getElementById("input-lon") as HTMLInputElement;
const setLocBtn = document.getElementById("set-loc-btn") as HTMLButtonElement;

const speedInput = document.getElementById("input-speed") as HTMLInputElement;
const waypointCountSpan = document.getElementById("waypoint-count") as HTMLSpanElement;
const startMoveBtn = document.getElementById("start-move-btn") as HTMLButtonElement;
const stopMoveBtn = document.getElementById("stop-move-btn") as HTMLButtonElement;

// 3. 應用程式局部狀態
let selectedUdid = "";
let selectedVersion = "";
let waypoints: GpsCoordinate[] = [];

/**
 * 輔助函式：統一解析錯誤訊息，避免出現 [object Object]
 */
function parseError(e: any): string {
  console.error("DEBUG - IPC Error Object:", e);
  // 如果是後端傳回的結構化 AppError，通常會有 message 欄位
  if (typeof e === 'object' && e !== null && 'message' in e) {
    return e.message;
  }
  // 如果是字串或其他型別
  return typeof e === 'string' ? e : JSON.stringify(e);
}

/** --- 設備管理邏輯 --- **/

scanBtn.addEventListener("click", async () => {
  scanBtn.disabled = true;
  deviceListDiv.innerHTML = "正在掃描 iOS 設備...";
  
  try {
    const devices = await Commands.getConnectedDevices();
    if (devices.length === 0) {
      deviceListDiv.innerHTML = "未偵測到任何設備。";
      mountBtn.disabled = true;
      return;
    }

    // 取得第一台設備資訊
    const dev = devices[0];
    selectedUdid = dev.udid;
    selectedVersion = dev.ios_version || "16.0";
    mountBtn.disabled = false;

    deviceListDiv.innerHTML = `
      <div class="device-item active">
        <strong>UDID:</strong> ${dev.udid}<br/>
        <strong>系統版本:</strong> iOS ${selectedVersion}<br/>
        <strong>連線方式:</strong> ${dev.connection_type}
      </div>
    `;
  } catch (e) {
    deviceListDiv.innerHTML = `<span style="color:red">掃描失敗: ${parseError(e)}</span>`;
  } finally {
    scanBtn.disabled = false;
  }
});

mountBtn.addEventListener("click", async () => {
  mountBtn.disabled = true;
  mountStatusDiv.innerText = "⏳ 正在掛載 DDI 映像檔...";
  mountStatusDiv.style.color = "blue";

  try {
    const result = await Commands.mountDdi(selectedUdid, selectedVersion);
    mountStatusDiv.innerText = `✅ ${result.message}`;
    mountStatusDiv.style.color = "green";
    
    // 解鎖座標控制功能
    toggleLocationControls(true);
  } catch (e) {
    mountStatusDiv.innerText = `❌ 掛載失敗: ${parseError(e)}`;
    mountStatusDiv.style.color = "red";
  } finally {
    mountBtn.disabled = false;
  }
});

/** --- 位置與移動邏輯 --- **/

function toggleLocationControls(enabled: boolean) {
  latInput.disabled = !enabled;
  lonInput.disabled = !enabled;
  setLocBtn.disabled = !enabled;
  speedInput.disabled = !enabled;
  startMoveBtn.disabled = !enabled;
}

// 監聽地圖點擊（更新輸入框與累積路徑點）
document.getElementById("map")?.addEventListener("click", () => {
  const lat = parseFloat(latInput.value);
  const lon = parseFloat(lonInput.value);
  if (!isNaN(lat) && !isNaN(lon)) {
    waypoints.push({ latitude: lat, longitude: lon });
    waypointCountSpan.innerText = waypoints.length.toString();
  }
});

setLocBtn.addEventListener("click", async () => {
  const coord: GpsCoordinate = {
    latitude: parseFloat(latInput.value),
    longitude: parseFloat(lonInput.value)
  };

  try {
    await Commands.setLocation(selectedUdid, coord);
    map.updateDeviceLocation(coord);
  } catch (e) {
    alert(`設定失敗: ${parseError(e)}`);
  }
});

startMoveBtn.addEventListener("click", async () => {
  if (waypoints.length < 2) {
    alert("請至少在地圖上點選兩個以上的點位。");
    return;
  }

  try {
    await Commands.startMovement(selectedUdid, {
      waypoints,
      speed_kmh: parseFloat(speedInput.value),
      loop_path: true
    });
    startMoveBtn.disabled = true;
    stopMoveBtn.disabled = false;
    map.clearTrajectory();
  } catch (e) {
    alert(`啟動失敗: ${parseError(e)}`);
  }
});

stopMoveBtn.addEventListener("click", async () => {
  try {
    await Commands.stopMovement(selectedUdid);
    startMoveBtn.disabled = false;
    stopMoveBtn.disabled = true;
  } catch (e) {
    alert(`停止失敗: ${parseError(e)}`);
  }
});

/** --- 事件訂閱 --- **/

Events.onLocationUpdated((coord) => {
  map.updateDeviceLocation(coord);
});

Events.onMovementStopped((payload) => {
  console.log(`移動停止原因: ${payload.reason}`);
  startMoveBtn.disabled = false;
  stopMoveBtn.disabled = true;
});