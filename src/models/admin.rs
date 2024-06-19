use serde::{Deserialize, Serialize};

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
