use std::sync::{Arc, Mutex};

use reqwest::Client;

pub struct MarzbanAPIClient {
    pub base_url: String,
    pub client: Client,
    pub token: Arc<Mutex<Option<String>>>,
}

impl MarzbanAPIClient {
    pub fn new(base_url: &str) -> Self {
        MarzbanAPIClient {
            base_url: base_url.to_owned(),
            client: Client::new(),
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        MarzbanAPIClient {
            base_url: base_url.to_owned(),
            client: Client::new(),
            token: Arc::new(Mutex::new(Some(token.to_owned()))),
        }
    }
}
