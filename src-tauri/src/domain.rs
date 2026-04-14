#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionType {
    Usb,
    Network,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosDevice {
    pub udid: String,
    pub connection_type: ConnectionType,
    pub name: Option<String>,
    pub ios_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementConfig {
    pub waypoints: Vec<GpsCoordinate>,
    pub speed_kmh: f64,
    pub loop_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool, // 👈 修正：由 boolean 改為 bool
    pub message: String,
}