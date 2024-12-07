use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Admin {
    pub username: String,
    pub is_sudo: bool,
    pub telegram_id: Option<u64>,
    pub discord_webhook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminCreate {
    pub username: String,
    pub is_sudo: bool,
    pub telegram_id: Option<u64>,
    pub discord_webhook: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminModify {
    pub password: Option<String>,
    pub is_sudo: bool,
    pub telegram_id: Option<u64>,
    pub discord_webhook: Option<String>,
}
