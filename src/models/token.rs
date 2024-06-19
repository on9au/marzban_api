use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    token_type: Option<String>, // default: bearer
}
