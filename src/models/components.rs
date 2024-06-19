/*
    # components.rs
    Models for the components of the API.
*/

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use validator::Validate;

// Define default values for fields that need them
fn default_usage_coefficient() -> f32 {
    1.0
}

fn default_port() -> u32 {
    62050
}

fn default_api_port() -> u32 {
    62051
}

fn default_true() -> bool {
    true
}

fn default_min_node_version() -> String {
    "v0.2.0".to_string()
}

fn default_proxy_host_security() -> ProxyHostSecurity {
    ProxyHostSecurity::InboundDefault
}

fn default_with_0() -> u32 {
    0
}

fn default_data_limit_reset_strategy() -> UserDataLimitResetStrategy {
    UserDataLimitResetStrategy::NoReset
}

fn default_empty_string() -> String {
    "".to_string()
}

#[derive(Serialize, Deserialize)]
pub struct Admin {
    username: String,
    is_sudo: bool,
    telegram_id: Option<u32>,
    discord_webhook: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminCreate {
    username: String,
    is_sudo: bool,
    telegram_id: Option<u32>,
    discord_webhook: Option<String>,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminModify {
    password: Option<String>,
    is_sudo: bool,
    telegram_id: Option<u32>,
    discord_webhook: Option<String>,
}

// Orignally named: 'Body_admin_token_api_admin_token_post' in the openapi.json
#[derive(Serialize, Deserialize)]
pub struct BodyAdminTokenApiAdminTokenPost {
    grant_type: Option<String>,
    username: String,
    password: String,
    #[serde(default = "default_empty_string")]
    scope: String,
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CoreStats {
    version: String,
    started: bool,
    logs_websocket: String,
}

#[derive(Serialize, Deserialize)]
pub struct HTTPValidationError {
    detail: Option<ValidationError>,
}

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

#[derive(Serialize, Deserialize)]
pub struct ProxyHost {
    remark: String,
    address: String,
    port: Option<u32>,
    sni: Option<String>,
    host: Option<String>,
    path: Option<String>,
    #[serde(default = "default_proxy_host_security")]
    security: ProxyHostSecurity,
    alpn: Option<ProxyHostALPN>,
    fingerprint: Option<ProxyHostFingerprint>,
    allow_insecure: Option<bool>,
    is_disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostALPN {
    // TODO!
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostFingerprint {
    // TODO!
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostSecurity {
    #[serde(rename = "inbound_default")]
    InboundDefault,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "tls")]
    Tls,
}

#[derive(Serialize, Deserialize)]
pub struct ProxyInbound {
    tag: String,
    protocol: ProxyTypes,
    network: String,
    tls: String,
    port: ProxyInboundPort,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProxyInboundPort {
    String(String),
    Integer(u32),
}

#[derive(Serialize, Deserialize)]
pub struct ProxySettings {}

#[derive(Serialize, Deserialize)]
pub enum ProxyTypes {
    #[serde(rename = "vmess")]
    Vmess,
    #[serde(rename = "vless")]
    Vless,
    #[serde(rename = "trojan")]
    Trojan,
    #[serde(rename = "shadowsocks")]
    ShadowSocks,
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

#[derive(Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    token_type: Option<String>, // default: bearer
}

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    proxies: HashMap<String, ProxySettings>,
    expire: Option<u32>,
    data_limit: u32, // min: 0, can be 0 or greater
    #[serde(default = "default_data_limit_reset_strategy")]
    data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    inbounds: HashMap<String, Vec<String>>,
    note: Option<String>,
    sub_updated_at: Option<DateTime<Utc>>,
    sub_last_user_agent: Option<String>,
    online_at: Option<DateTime<Utc>>,
    on_hold_expire_duration: Option<u32>,
    on_hold_timeout: Option<DateTime<Utc>>,
    username: String,
    status: UserStatusCreate,
}

#[derive(Serialize, Deserialize)]
pub enum UserDataLimitResetStrategy {
    #[serde(rename = "no_reset")]
    NoReset,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}

#[derive(Serialize, Deserialize)]
pub struct UserModify {
    proxies: HashMap<String, ProxySettings>,
    expire: Option<u32>,
    data_limit: u32, // min: 0, can be 0 or greater
    data_limit_reset_strategy: UserDataLimitResetStrategy,
    inbounds: HashMap<String, Vec<String>>,
    note: Option<String>,
    sub_updated_at: Option<DateTime<Utc>>,
    sub_last_user_agent: Option<String>,
    online_at: Option<DateTime<Utc>>,
    on_hold_expire_duration: Option<u32>,
    on_hold_timeout: Option<DateTime<Utc>>,
    status: UserStatusModify,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    proxies: HashMap<String, ProxySettings>,
    expire: Option<u32>,
    data_limit: u32, // min: 0, can be 0 or greater
    data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    inbounds: HashMap<String, Vec<String>>,
    note: Option<String>,
    sub_updated_at: Option<DateTime<Utc>>,
    sub_last_user_agent: Option<String>,
    online_at: Option<DateTime<Utc>>,
    on_hold_expire_duration: Option<u32>,
    on_hold_timeout: Option<DateTime<Utc>>,
    username: String,
    status: UserStatus,
    used_traffic: u32,
    lifetime_used_traffic: u32, // default: 0
    created_at: DateTime<Utc>,
    links: Vec<String>,
    #[serde(default = "default_empty_string")]
    subscription_url: String,
    excluded_inbounds: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "on_hold")]
    OnHold,
}

#[derive(Serialize, Deserialize)]
pub enum UserStatusCreate {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "on_hold")]
    OnHold,
}

#[derive(Serialize, Deserialize)]
pub enum UserStatusModify {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "on_hold")]
    OnHold,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserTemplateCreate {
    name: Option<String>,
    #[serde(default = "default_with_0")]
    data_limit: u32, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    expire_duration: u32, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    username_suffix: String, // maxLength: 20, minLength: 1
    inbounds: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserTemplateModify {
    name: Option<String>,
    #[serde(default = "default_with_0")]
    data_limit: u32, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    expire_duration: u32, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    username_suffix: String, // maxLength: 20, minLength: 1
    inbounds: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserTemplateResponse {
    name: Option<String>,
    #[serde(default = "default_with_0")]
    data_limit: u32, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    expire_duration: u32, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    username_suffix: String, // maxLength: 20, minLength: 1
    inbounds: HashMap<String, Vec<String>>,
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UserUsageResponse {
    node_id: Option<u32>,
    node_name: String,
    used_traffic: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UserUsagesResponse {
    username: String,
    usages: Vec<UserUsageResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    users: Vec<UserResponse>,
    total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ValidationError {
    loc: ValidationErrorLocation, // Location
    msg: String,                  // Message
    r#type: String,               // Error Type
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValidationErrorLocation {
    String(String),
    Integer(u32),
}
