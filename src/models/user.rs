use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::base::{
    default_data_limit_reset_strategy, default_empty_string, parse_datetime, parse_some_datetime,
};

use super::proxy::ProxySettings;

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    pub proxies: HashMap<String, ProxySettings>,
    pub expire: Option<u32>,
    pub data_limit: u32, // min: 0, can be 0 or greater
    #[serde(default = "default_data_limit_reset_strategy")]
    pub data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    pub inbounds: HashMap<String, Vec<String>>,
    pub note: Option<String>,
    pub sub_updated_at: Option<DateTime<Utc>>,
    pub sub_last_user_agent: Option<String>,
    pub online_at: Option<DateTime<Utc>>,
    pub on_hold_expire_duration: Option<u32>,
    pub on_hold_timeout: Option<DateTime<Utc>>,
    pub username: String,
    pub status: UserStatusCreate,
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
    pub proxies: HashMap<String, ProxySettings>,
    pub expire: Option<u32>,
    pub data_limit: u32, // min: 0, can be 0 or greater
    pub data_limit_reset_strategy: UserDataLimitResetStrategy,
    pub inbounds: HashMap<String, Vec<String>>,
    pub note: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub sub_updated_at: Option<DateTime<Utc>>,
    pub sub_last_user_agent: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub online_at: Option<DateTime<Utc>>,
    pub on_hold_expire_duration: Option<u32>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub on_hold_timeout: Option<DateTime<Utc>>,
    pub status: UserStatusModify,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub proxies: HashMap<String, ProxySettings>,
    pub expire: Option<u32>,
    pub data_limit: Option<u32>, // min: 0, can be 0 or greater
    pub data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    pub inbounds: HashMap<String, Vec<String>>,
    pub note: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub sub_updated_at: Option<DateTime<Utc>>,
    pub sub_last_user_agent: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub online_at: Option<DateTime<Utc>>,
    pub on_hold_expire_duration: Option<u32>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub on_hold_timeout: Option<DateTime<Utc>>,
    pub username: String,
    pub status: UserStatus,
    pub used_traffic: u32,
    pub lifetime_used_traffic: u32, // default: 0
    #[serde(deserialize_with = "parse_datetime")]
    pub created_at: DateTime<Utc>,
    pub links: Vec<String>,
    #[serde(default = "default_empty_string")]
    pub subscription_url: String,
    pub excluded_inbounds: HashMap<String, Vec<String>>,
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
    pub node_id: Option<u32>,
    pub node_name: String,
    pub used_traffic: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UserUsagesResponse {
    pub username: String,
    pub usages: Vec<UserUsageResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
    pub total: u32,
}
