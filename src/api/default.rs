use reqwest::StatusCode;

use crate::{client::MarzbanAPIClient, error::ApiError};

impl MarzbanAPIClient {
    // Base URL, would return HTML
    pub async fn base_url(&self) -> Result<String, ApiError> {
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &self.base_url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            _ => Err(ApiError::UnexpectedResponse),
        }
    }
}
