//! # Marzban API Client
//!
//! This module contains the API client for the Marzban API.

use std::{fmt::Debug, sync::Arc};

use reqwest::{Client, IntoUrl};
use tokio::sync::RwLock;

/// The Marzban API client.
///
/// This struct contains the base URL for the API, the reqwest client, and the token within the Inner struct.
///
/// You do **not** have to wrap the `Client` in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Debug, Clone)]
pub struct MarzbanAPIClient {
    pub(crate) inner: Arc<MarzbanAPIClientRef>,
}

/// The Marzban API client reference. Contains all the data needed to make requests.
/// This struct is used to allow for thread-safe access to the client and cloning, also making it cheap to clone the outer MarzbanAPIClient struct.
pub(crate) struct MarzbanAPIClientRef {
    pub(crate) base_url: String,
    pub(crate) client: Client,
    pub(crate) token: RwLock<Option<String>>,
}

impl Debug for MarzbanAPIClientRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarzbanAPIClient")
            .field("base_url", &self.base_url)
            .field("client", &self.client)
            .field("token", &"*****")
            .finish()
    }
}

impl MarzbanAPIClient {
    /// Create a new Marzban API client with the given base URL.
    pub fn new(base_url: &str) -> Self {
        MarzbanAPIClient {
            inner: MarzbanAPIClientRef {
                base_url: base_url.to_string(),
                client: Client::new(),
                token: RwLock::new(None),
            }
            .into(),
        }
    }

    /// Create a new Marzban API client with the given base URL and token.
    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        MarzbanAPIClient {
            inner: MarzbanAPIClientRef {
                base_url: base_url.to_string(),
                client: Client::new(),
                token: RwLock::new(Some(token.to_owned())),
            }
            .into(),
        }
    }

    /// Helper method to create a request with authorization header if token is present
    pub(crate) async fn prepare_authorized_request(
        &self,
        method: reqwest::Method,
        url: impl IntoUrl,
    ) -> reqwest::RequestBuilder {
        let mut request_builder = self.inner.client.request(method, url);
        if let Some(token) = self.inner.token.read().await.as_ref() {
            request_builder = request_builder.bearer_auth(token);
        }
        request_builder
    }
}
