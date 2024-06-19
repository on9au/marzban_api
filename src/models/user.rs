use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::base::{default_data_limit_reset_strategy, default_empty_string};

use super::proxy::ProxySettings;

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
