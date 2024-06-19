use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoreStats {
    version: String,
    started: bool,
    logs_websocket: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemStats {
    version: String,
    mem_total: u32,
    mem_used: u32,
    cpu_cores: u32,
    cpu_usage: f32,
    total_user: u32,
    users_active: u32,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
    incoming_bandwidth_speed: u32,
    outgoing_bandwidth_speed: u32,
}
