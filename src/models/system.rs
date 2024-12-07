use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreStats {
    pub version: String,
    pub started: bool,
    pub logs_websocket: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStats {
    pub version: String,
    pub mem_total: u64,
    pub mem_used: u64,
    pub cpu_cores: u64,
    pub cpu_usage: f64,
    pub total_user: u64,
    pub users_active: u64,
    pub incoming_bandwidth: u64,
    pub outgoing_bandwidth: u64,
    pub incoming_bandwidth_speed: u64,
    pub outgoing_bandwidth_speed: u64,
}
