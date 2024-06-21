use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoreStats {
    pub version: String,
    pub started: bool,
    pub logs_websocket: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemStats {
    pub version: String,
    pub mem_total: u32,
    pub mem_used: u32,
    pub cpu_cores: u32,
    pub cpu_usage: f32,
    pub total_user: u32,
    pub users_active: u32,
    pub incoming_bandwidth: u32,
    pub outgoing_bandwidth: u32,
    pub incoming_bandwidth_speed: u32,
    pub outgoing_bandwidth_speed: u32,
}
