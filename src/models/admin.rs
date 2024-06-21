use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Admin {
    pub username: String,
    pub is_sudo: bool,
    pub telegram_id: Option<u32>,
    pub discord_webhook: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdminCreate {
    pub username: String,
    pub is_sudo: bool,
    pub telegram_id: Option<u32>,
    pub discord_webhook: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminModify {
    pub password: Option<String>,
    pub is_sudo: bool,
    pub telegram_id: Option<u32>,
    pub discord_webhook: Option<String>,
}
