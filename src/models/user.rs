use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::base::{
    default_data_limit_reset_strategy, default_empty_string, parse_datetime, parse_some_datetime,
};

use super::admin::Admin;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserCreate {
    pub proxies: Proxies,
    pub expire: Option<u64>,
    #[validate(range(min = 0))]
    pub data_limit: u64, // min: 0, can be 0 or greater
    #[serde(default = "default_data_limit_reset_strategy")]
    pub data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    pub inbounds: Inbounds,
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_last_user_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_expire_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_in_days: Option<u64>,
    pub username: String,
    pub status: UserStatusCreate,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserModify {
    pub proxies: Proxies,
    pub expire: Option<u64>,
    #[validate(range(min = 0))]
    pub data_limit: u64, // min: 0, can be 0 or greater
    pub data_limit_reset_strategy: UserDataLimitResetStrategy,
    pub inbounds: Inbounds,
    pub note: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_updated_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_last_user_agent: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_expire_duration: Option<u64>,
    #[serde(deserialize_with = "parse_some_datetime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_timeout: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_delete_in_days: Option<u64>,
    pub status: UserStatusModify,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Proxies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trojan: Option<Trojan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vless: Option<Vless>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmess: Option<Vmess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowsocks: Option<Shadowsocks>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trojan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vless {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vmess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Shadowsocks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inbounds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trojan: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vless: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vmess: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadowsocks: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserResponse {
    pub proxies: Proxies,
    pub expire: Option<u64>,
    #[validate(range(min = 0))]
    pub data_limit: Option<u64>, // min: 0, can be 0 or greater
    pub data_limit_reset_strategy: UserDataLimitResetStrategy, // default: no_reset
    pub inbounds: Inbounds,
    pub note: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub sub_updated_at: Option<DateTime<Utc>>,
    pub sub_last_user_agent: Option<String>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub online_at: Option<DateTime<Utc>>,
    pub on_hold_expire_duration: Option<u64>,
    #[serde(deserialize_with = "parse_some_datetime")]
    pub on_hold_timeout: Option<DateTime<Utc>>,
    pub auto_delete_in_days: Option<u64>,
    pub username: String,
    pub status: UserStatus,
    pub used_traffic: u64,
    pub lifetime_used_traffic: u64, // default: 0
    #[serde(deserialize_with = "parse_datetime")]
    pub created_at: DateTime<Utc>,
    pub links: Vec<String>,
    #[serde(default = "default_empty_string")]
    pub subscription_url: String,
    pub excluded_inbounds: Inbounds,
    pub admin: Admin,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatusCreate {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "on_hold")]
    OnHold,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatusModify {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "on_hold")]
    OnHold,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUsageResponse {
    pub node_id: Option<u64>,
    pub node_name: String,
    pub used_traffic: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUsagesResponse {
    pub username: String,
    pub usages: Vec<UserUsageResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersUsagesResponse {
    pub users: Vec<UserUsagesResponse>,
}
