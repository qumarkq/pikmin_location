import { getConnectedDevices, mountDdi, setLocation } from './ipc/commands';
import { LocationMap } from './ui/map';
import './style.css'; // 假設你有一個全域樣式檔

// DOM 元素
const deviceSelect = document.getElementById('device-select') as HTMLSelectElement;
const mountBtn = document.getElementById('mount-btn') as HTMLButtonElement;
const teleportBtn = document.getElementById('teleport-btn') as HTMLButtonElement;
const currentCoordSpan = document.getElementById('current-coord') as HTMLSpanElement;
const logConsole = document.getElementById('log-console') as HTMLDivElement;

// 日誌系統
function log(message: string, type: 'info' | 'error' | 'success' = 'info') {
  const entry = document.createElement('div');
  entry.className = `log-entry ${type}`;
  entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
  logConsole.appendChild(entry);
  logConsole.scrollTop = logConsole.scrollHeight;
}

// 初始化地圖
const appMap = new LocationMap('map');
appMap.onLocationSelect((lat, lng) => {
  currentCoordSpan.textContent = `${lat.toFixed(5)}, ${lng.toFixed(5)}`;
  log(`已鎖定目標座標: ${lat.toFixed(5)}, ${lng.toFixed(5)}`, 'info');
});

// 載入設備
async function loadDevices() {
  log("掃描 USB 設備中...");
  try {
    const devices = await getConnectedDevices();
    deviceSelect.innerHTML = '';
    
    if (devices.length === 0) {
      deviceSelect.innerHTML = '<option value="">未偵測到設備</option>';
      return;
    }

    devices.forEach(dev => {
      const opt = document.createElement('option');
      opt.value = dev.udid;
      opt.dataset.version = dev.ios_version || '';
      opt.textContent = `iOS ${dev.ios_version} (${dev.udid.substring(0, 6)}...)`;
      deviceSelect.appendChild(opt);
    });
    log("設備掃描完成！", 'success');
  } catch (err: any) {
    log(`掃描失敗: ${err.message || err}`, 'error');
  }
}

// 綁定掛載按鈕
mountBtn.addEventListener('click', async () => {
  const udid = deviceSelect.value;
  const version = deviceSelect.options[deviceSelect.selectedIndex]?.dataset.version;
  
  if (!udid || !version) return log("請先選擇設備！", 'error');

  log(`開始掛載 iOS ${version} DDI...`, 'info');
  mountBtn.disabled = true;

  try {
    const res = await mountDdi(udid, version);
    log(`✅ ${res.message}`, 'success');
  } catch (err: any) {
    log(`❌ 掛載失敗: ${err.message || err}`, 'error');
  } finally {
    mountBtn.disabled = false;
  }
});

// 綁定傳送按鈕
teleportBtn.addEventListener('click', async () => {
  const udid = deviceSelect.value;
  const coord = appMap.selectedCoord;

  if (!udid) return log("請先選擇設備！", 'error');
  if (!coord) return log("請先在地圖上點選位置！", 'error');

  log(`🚀 傳送至 ${coord.lat.toFixed(5)}, ${coord.lng.toFixed(5)}...`, 'info');
  teleportBtn.disabled = true;

  try {
    const res = await setLocation(udid, coord.lat, coord.lng);
    log(`🌍 ${res.message}`, 'success');
  } catch (err: any) {
    log(`❌ 傳送失敗: ${err.message || err}`, 'error');
  } finally {
    teleportBtn.disabled = false;
  }
});

// 啟動時執行
window.addEventListener('DOMContentLoaded', loadDevices);