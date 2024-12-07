use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::base::default_with_0;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserTemplateCreate {
    pub name: Option<String>,
    #[serde(default = "default_with_0")]
    #[validate(range(min = 0))]
    pub data_limit: u64, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    pub expire_duration: u64, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    pub username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    pub username_suffix: String, // maxLength: 20, minLength: 1
    pub inbounds: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserTemplateModify {
    pub name: Option<String>,
    #[serde(default = "default_with_0")]
    #[validate(range(min = 0))]
    pub data_limit: u64, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    pub expire_duration: u64, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    pub username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    pub username_suffix: String, // maxLength: 20, minLength: 1
    pub inbounds: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct UserTemplateResponse {
    pub name: Option<String>,
    #[serde(default = "default_with_0")]
    #[validate(range(min = 0))]
    pub data_limit: u64, // default: 0, can be 0 or greater
    #[serde(default = "default_with_0")]
    pub expire_duration: u64, // default: 0, can be 0 or greater
    #[validate(length(min = 1, max = 20))]
    pub username_prefix: String, // maxLength: 20, minLength: 1
    #[validate(length(min = 1, max = 20))]
    pub username_suffix: String, // maxLength: 20, minLength: 1
    pub inbounds: HashMap<String, Vec<String>>,
    pub id: u64,
}
