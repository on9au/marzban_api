//! # Default API Category

use reqwest::StatusCode;

use crate::{client::MarzbanAPIClient, error::ApiError};

impl MarzbanAPIClient {
    /// `GET /`
    ///
    /// Base URL of the Marzban panel.
    pub async fn base_url(&self) -> Result<String, ApiError> {
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &self.inner.base_url)
            .await
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            _ => Err(ApiError::UnexpectedResponse),
        }
    }
}
