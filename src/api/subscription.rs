use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user::{UserResponse, UserUsagesResponse},
    },
};

impl MarzbanAPIClient {
    // Returns the website. If you are looking for subscription format, please instead use user_subscription_with_client_type().
    pub async fn user_subscription(&self, user_token: &str) -> Result<String, ApiError> {
        let url = format!("{}/sub/{}", self.base_url, user_token);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
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

    // Returns information about the user
    pub async fn user_subscription_info(&self, user_token: &str) -> Result<UserResponse, ApiError> {
        let url = format!("{}/sub/{}/info", self.base_url, user_token);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
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

    // Returns user usage details
    pub async fn user_get_usage(
        &self,
        user_token: &str,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<UserUsagesResponse, ApiError> {
        let url = format!("{}/sub/{}/usage", self.base_url, user_token);
        let mut params = Vec::new();
        if let Some(value) = start {
            params.push(value)
        }
        if let Some(value) = end {
            params.push(value)
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserUsagesResponse>()
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

    // Returns the actual subscription format
    pub async fn user_subscription_with_client_type(
        &self,
        user_token: &str,
        client_type: &ClientTypes,
    ) -> Result<String, ApiError> {
        let url = format!("{}/sub/{}/{}", self.base_url, user_token, client_type);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
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
}

pub enum ClientTypes {
    SingBox,
    ClashMeta,
    Clash,
    Outline,
    V2Ray,
}

impl std::fmt::Display for ClientTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientTypes::SingBox => write!(f, "sing-box"),
            ClientTypes::ClashMeta => write!(f, "clash-meta"),
            ClientTypes::Clash => write!(f, "clash"),
            ClientTypes::Outline => write!(f, "outline"),
            ClientTypes::V2Ray => write!(f, "v2ray"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_client_types() {
        assert_eq!(ClientTypes::Clash.to_string(), "clash")
    }
}
