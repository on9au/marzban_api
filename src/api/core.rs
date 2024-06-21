use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{errors::HTTPValidationError, system::CoreStats},
};

impl MarzbanAPIClient {
    // Returns (xtls) core stats
    pub async fn get_core_stats(&self) -> Result<CoreStats, ApiError> {
        let url = format!("{}/api/core", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<CoreStats>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Restarts (xtls) core
    pub async fn restart_core(&self) -> Result<String, ApiError> {
        let url = format!("{}/api/core", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Returns core config as string
    pub async fn get_core_config(&self) -> Result<String, ApiError> {
        let url = format!("{}/api/core/config", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Returns core config as string
    pub async fn modify_core_config(&self, config_as_json: &str) -> Result<String, ApiError> {
        let url = format!("{}/api/core/config", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .form(config_as_json)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }
}
