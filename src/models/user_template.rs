use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::base::default_with_0;

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
