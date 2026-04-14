import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { GpsCoordinate, IosDevice } from "./commands";

/** --- 事件 Payload 定義 --- **/
export interface DdiMountedPayload {
  udid: string;
  ios_version: string;
}

export interface MovementStoppedPayload {
  udid: string;
  reason: string;
}

/**
 * 全域事件訂閱中心
 */
export const Events = {
  // 監聽設備連線狀態 (規格書 8)
  onDeviceConnected: (callback: (dev: IosDevice) => void) =>
    listen<IosDevice>("device://connected", (event) => callback(event.payload)),

  // 監聽 DDI 掛載成功
  onDdiMounted: (callback: (payload: DdiMountedPayload) => void) =>
    listen<DdiMountedPayload>("ddi://mounted", (event) => callback(event.payload)),

  // 監聽座標更新 (最核心：連動地圖軌跡)
  onLocationUpdated: (callback: (coord: GpsCoordinate) => void) =>
    listen<GpsCoordinate>("location://updated", (event) => callback(event.payload)),

  // 監聽移動停止
  onMovementStopped: (callback: (payload: MovementStoppedPayload) => void) =>
    listen<MovementStoppedPayload>("movement://stopped", (event) => callback(event.payload)),
};

/**
 * 輔助工具：一次性清理所有訂閱 (如果未來 UI 複雜化時使用)
 */
export class EventManager {
  private unlisteners: UnlistenFn[] = [];

  async add(promise: Promise<UnlistenFn>) {
    const unlisten = await promise;
    this.unlisteners.push(unlisten);
  }

  destroy() {
    this.unlisteners.forEach((fn) => fn());
    this.unlisteners = [];
  }
}