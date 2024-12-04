use std::sync::Arc;

use reqwest::Client;
use tokio::sync::RwLock;

pub struct MarzbanAPIClient {
    pub base_url: String,
    pub client: Client,
    pub token: Arc<RwLock<Option<String>>>,
}

impl MarzbanAPIClient {
    pub fn new(base_url: &str) -> Self {
        MarzbanAPIClient {
            base_url: base_url.to_owned(),
            client: Client::new(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        MarzbanAPIClient {
            base_url: base_url.to_owned(),
            client: Client::new(),
            token: Arc::new(RwLock::new(Some(token.to_owned()))),
        }
    }

    // Helper method to create a request with authorization header if token is present
    pub(crate) async fn prepare_authorized_request(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> reqwest::RequestBuilder {
        let mut request_builder = self.client.request(method, url);
        if let Some(token) = self.token.read().await.as_ref() {
            request_builder = request_builder.bearer_auth(token);
        }
        request_builder
    }
}
