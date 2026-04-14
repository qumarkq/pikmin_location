import { invoke } from "@tauri-apps/api/core";

/** --- 從規格書 domain.rs 映射的強型別定義 --- **/

export interface IosDevice {
  udid: string;
  connection_type: "usb" | "network";
  name: string | null;
  ios_version: string | null;
}

export interface GpsCoordinate {
  latitude: number; // 👈 修正：TS 使用 number
  longitude: number; // 👈 修正：TS 使用 number
}

export interface MovementConfig {
  waypoints: GpsCoordinate[];
  speed_kmh: number;
  loop_path: boolean;
}

export interface OperationResult {
  success: boolean;
  message: string;
}

/** --- 規格書 6 節定義的結構化錯誤 --- **/
export interface AppError {
  kind: string;
  message: string;
}

/**
 * 內部輔助函式：處理 Tauri Invoke 並統一錯誤型別
 */
async function call<T>(cmd: string, args: any = {}): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    const appErr = error as AppError;
    console.error(`[IPC Error] ${cmd}:`, appErr);
    throw appErr;
  }
}

/** --- 規格書 7 節定義的 Commands 實作 --- **/

export const Commands = {
  getConnectedDevices: () => 
    call<IosDevice[]>("get_connected_devices"),

  getDeviceIosVersion: (udid: string) => 
    call<string>("get_device_ios_version", { udid }),

  checkDdiExists: (iosVersion: string) => 
    call<boolean>("check_ddi_exists", { iosVersion }),

  mountDdi: (udid: string, iosVersion: string) => 
    call<OperationResult>("mount_ddi", { udid, iosVersion }),

  setLocation: (udid: string, coordinate: GpsCoordinate) => 
    call<OperationResult>("set_location", { udid, coordinate }),

  startMovement: (udid: string, config: MovementConfig) => 
    call<OperationResult>("start_movement", { udid, config }),

  stopMovement: (udid: string) => 
    call<OperationResult>("stop_movement", { udid }),
};