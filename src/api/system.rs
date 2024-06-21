use std::collections::HashMap;

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        proxy::{ProxyHost, ProxyInbound},
        system::SystemStats,
    },
};

impl MarzbanAPIClient {
    // Returns system stats
    pub async fn get_system_stats(&self) -> Result<SystemStats, ApiError> {
        let url = format!("{}/api/system", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<SystemStats>()
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

    // Returns inbound configs
    pub async fn get_inbounds(&self) -> Result<HashMap<String, Vec<ProxyInbound>>, ApiError> {
        let url = format!("{}/api/inbounds", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<String, Vec<ProxyInbound>>>()
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

    // Returns host configs
    pub async fn get_hosts(&self) -> Result<HashMap<String, Vec<ProxyHost>>, ApiError> {
        let url = format!("{}/api/hosts", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<String, Vec<ProxyHost>>>()
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

    // Returns host configs
    pub async fn modify_hosts(
        &self,
        body: &HashMap<String, Vec<ProxyHost>>,
    ) -> Result<HashMap<String, Vec<ProxyHost>>, ApiError> {
        let url = format!("{}/api/hosts", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .form(&body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<String, Vec<ProxyHost>>>()
                .await
                .map_err(ApiError::NetworkError),
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
