use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub token_type: Option<String>, // default: bearer
}
