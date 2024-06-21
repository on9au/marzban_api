use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::base::{
    default_api_port, default_min_node_version, default_port, default_true,
    default_usage_coefficient,
};

#[derive(Serialize, Deserialize, Validate)]
pub struct NodeCreate {
    pub name: String,
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u32, // default: 62050
    #[serde(default = "default_api_port")]
    pub api_port: u32, // default: 62051
    #[serde(default = "default_usage_coefficient")]
    #[validate(range(min = 0.0))]
    pub usage_coefficient: f32, // exclusiveMinimum: 0, default: 1
    #[serde(default = "default_true")]
    pub add_as_new_host: bool, // default: true
}

#[derive(Serialize, Deserialize)]
pub struct NodeModify {
    pub name: Option<String>,
    pub address: Option<String>,
    pub port: Option<u32>,
    pub api_port: Option<u32>,
    pub usage_coefficient: Option<f32>,
    pub status: Option<NodeStatus>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct NodeResponse {
    pub name: String,
    pub address: String,
    #[serde(default = "default_port")]
    pub port: u32,
    #[serde(default = "default_api_port")]
    pub api_port: u32,
    #[serde(default = "default_usage_coefficient")]
    #[validate(range(min = 0.0))]
    pub usage_coefficient: f32,
    pub id: u32,
    pub xray_version: String,
    pub status: NodeStatus,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeSettings {
    #[serde(default = "default_min_node_version")]
    pub min_node_version: String,
    pub certificate: String,
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
    pub node_id: Option<u32>,
    pub node_name: String,
    pub uplink: u32,
    pub downlink: u32,
}

// Note 'Nodes' (S plural) in NodesUsageResponse.
#[derive(Serialize, Deserialize)]
pub struct NodesUsageResponse {
    pub usages: Vec<NodeUsageResponse>,
}
