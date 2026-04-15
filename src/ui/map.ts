import * as L from 'leaflet';
import 'leaflet/dist/leaflet.css';

export class LocationMap {
  private map: L.Map;
  private marker: L.Marker;
  public selectedCoord: { lat: number; lng: number } | null = null;
  private onLocationSelectCallback: ((lat: number, lng: number) => void) | null = null;

  constructor(containerId: string) {
    // 預設中心點 (新莊區)
    this.map = L.map(containerId).setView([25.035, 121.432], 14);

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© OpenStreetMap contributors'
    }).addTo(this.map);

    this.marker = L.marker([25.035, 121.432]).addTo(this.map);

    // 點擊地圖時更新座標
    this.map.on('click', (e: L.LeafletMouseEvent) => {
      this.selectedCoord = { lat: e.latlng.lat, lng: e.latlng.lng };
      this.marker.setLatLng(e.latlng);
      
      if (this.onLocationSelectCallback) {
        this.onLocationSelectCallback(e.latlng.lat, e.latlng.lng);
      }
    });
  }

  // 註冊選擇座標後的回呼函式，讓 main.ts 可以更新 UI
  public onLocationSelect(callback: (lat: number, lng: number) => void) {
    this.onLocationSelectCallback = callback;
  }
}