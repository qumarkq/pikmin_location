import L from 'leaflet'; // 👈 補上這一行，讓 TS 認識 L
import { GpsCoordinate } from "../ipc/commands";

// 取得 UI 上的輸入框元素
const latInput = document.getElementById("input-lat") as HTMLInputElement;
const lonInput = document.getElementById("input-lon") as HTMLInputElement;

export class MapModule {
  private map: L.Map;
  private deviceMarker: L.Marker | null = null;
  private trajectory: L.Polyline | null = null;
  private pathPoints: [number, number][] = [];

  constructor() {
    // 1. 依據規格書 9.3：初始化地圖，預設中心台北
    this.map = L.map("map").setView([25.0330, 121.5654], 13);

    // 2. 引入 OpenStreetMap 圖層
    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution: '&copy; OpenStreetMap contributors'
    }).addTo(this.map);

    // 3. 處理點擊地圖選取座標邏輯
    this.map.on("click", (e: L.LeafletMouseEvent) => {
      const { lat, lng } = e.latlng;
      
      // 更新輸入框內容
      latInput.value = lat.toFixed(6);
      lonInput.value = lng.toFixed(6);
      
      console.log(`[Map] 選取座標: ${lat}, ${lng}`);
    });
  }

  /**
   * 更新設備在地圖上的位置與軌跡
   */
  public updateDeviceLocation(coord: GpsCoordinate) {
    const pos: L.LatLngExpression = [coord.latitude, coord.longitude];
    this.pathPoints.push([coord.latitude, coord.longitude]);

    // 更新或建立 Marker
    if (!this.deviceMarker) {
      this.deviceMarker = L.marker(pos).addTo(this.map)
        .bindPopup("iOS 裝置目前位置")
        .openPopup();
    } else {
      this.deviceMarker.setLatLng(pos);
    }

    // 更新或建立歷史軌跡 (Polyline)
    if (!this.trajectory) {
      this.trajectory = L.polyline(this.pathPoints, { color: 'blue', weight: 3 }).addTo(this.map);
    } else {
      this.trajectory.setLatLngs(this.pathPoints as L.LatLngExpression[]);
    }

    // 平滑移動地圖中心至新座標
    this.map.panTo(pos);
  }

  /**
   * 清除目前的軌跡
   */
  public clearTrajectory() {
    this.pathPoints = [];
    if (this.trajectory) {
      this.trajectory.setLatLngs([]);
    }
  }
}