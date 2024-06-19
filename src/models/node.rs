use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::base::{
    default_api_port, default_min_node_version, default_port, default_true,
    default_usage_coefficient,
};

#[derive(Serialize, Deserialize, Validate)]
pub struct NodeCreate {
    name: String,
    address: String,
    #[serde(default = "default_port")]
    port: u32, // default: 62050
    #[serde(default = "default_api_port")]
    api_port: u32, // default: 62051
    #[serde(default = "default_usage_coefficient")]
    #[validate(range(min = 0.0))]
    usage_coefficient: f32, // exclusiveMinimum: 0, default: 1
    #[serde(default = "default_true")]
    add_as_new_host: bool, // default: true
}

#[derive(Serialize, Deserialize)]
pub struct NodeModify {
    name: Option<String>,
    address: Option<String>,
    port: Option<u32>,
    api_port: Option<u32>,
    usage_coefficient: Option<f32>,
    status: Option<NodeStatus>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NodeResponse {
    name: String,
    address: String,
    #[serde(default = "default_port")]
    port: u32,
    #[serde(default = "default_api_port")]
    api_port: u32,
    #[serde(default = "default_usage_coefficient")]
    #[validate(range(min = 0.0))]
    usage_coefficient: f32,
    id: u32,
    xray_version: String,
    status: NodeStatus,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeSettings {
    #[serde(default = "default_min_node_version")]
    min_node_version: String,
    certificate: String,
}

#[derive(Serialize, Deserialize)]
pub enum NodeStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Serialize, Deserialize)]
pub struct NodeUsageResponse {
    node_id: Option<u32>,
    node_name: String,
    uplink: u32,
    downlink: u32,
}

// Note 'Nodes' (S plural) in NodesUsageResponse.
#[derive(Serialize, Deserialize)]
pub struct NodesUsageResponse {
    usages: Vec<NodeUsageResponse>,
}
