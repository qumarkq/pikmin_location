import { invoke } from '@tauri-apps/api/core';

// 對齊 backend domain.rs 的結構
export interface IosDevice {
  udid: string;
  connection_type: string;
  name: string | null;
  ios_version: string | null;
}

export interface OperationResult {
  success: boolean;
  message: string;
}

export async function getConnectedDevices(): Promise<IosDevice[]> {
  return await invoke<IosDevice[]>('get_connected_devices');
}

export async function mountDdi(udid: string, iosVersion: string): Promise<OperationResult> {
  return await invoke<OperationResult>('mount_ddi', { udid, iosVersion });
}

export async function setLocation(udid: string, latitude: number, longitude: number): Promise<OperationResult> {
  return await invoke<OperationResult>('set_location', { 
    udid, 
    coord: { latitude, longitude } 
  });
}