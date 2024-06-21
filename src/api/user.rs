use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user::{
            UserCreate, UserModify, UserResponse, UserStatus, UserUsagesResponse, UsersResponse,
        },
    },
};

// Custom struct for query params in get users
#[derive(Serialize)]
pub struct GetUsersQueryParams {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub username: Option<Vec<String>>,
    pub status: Option<UserStatus>,
    pub sort: Option<String>,
}

impl MarzbanAPIClient {
    // Add a new user
    pub async fn add_user(&self, new_user: &UserCreate) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .json(new_user)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "User already exists".to_string(),
            )),
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

    // Get user info
    pub async fn get_user(&self, username: &str) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Modify user
    pub async fn modify_user(
        &self,
        username: &str,
        body: &UserModify,
    ) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .json(&body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Delete user
    pub async fn delete_user(&self, username: &str) -> Result<String, ApiError> {
        let url = format!("{}/api/user/{}", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Reset user data usage
    pub async fn reset_user_data_usage(&self, username: &str) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}/reset", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "User already exists".to_string(),
            )),
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

    // Revoke user subscription
    pub async fn revoke_user_subscription(&self, username: &str) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}/revoke_sub", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Get users
    pub async fn get_users(
        &self,
        query_params: &GetUsersQueryParams,
    ) -> Result<UsersResponse, ApiError> {
        let url = format!("{}/api/users", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .query(&query_params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UsersResponse>()
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

    // Reset all users data usage
    pub async fn reset_all_users_data_usage(&self) -> Result<String, ApiError> {
        let url = format!("{}/api/users/reset", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Get user usage
    pub async fn get_user_usage(
        &self,
        username: &str,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<UserUsagesResponse, ApiError> {
        let url = format!("{}/api/user/{}/usage", self.base_url, username);
        let mut params = Vec::new();
        if let Some(value) = start {
            params.push(("start", value.to_string()))
        }
        if let Some(value) = end {
            params.push(("end", value.to_string()))
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserUsagesResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Set owner for a user
    pub async fn set_owner_of_user(
        &self,
        username: &str,
        admin_username: &str,
    ) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}/set-owner", self.base_url, username);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .query(&[("admin_username", admin_username)])
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
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

    // Get expired users
    pub async fn get_expired_users(
        &self,
        expired_before: Option<DateTime<Utc>>,
        expired_after: Option<DateTime<Utc>>,
    ) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/users/expired", self.base_url);
        let mut params = Vec::new();
        if let Some(value) = expired_before {
            params.push(("expired_before", value))
        }
        if let Some(value) = expired_after {
            params.push(("expired_after", value))
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<String>>()
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

    // Delete expired users
    pub async fn delete_expired_users(
        &self,
        expired_before: Option<DateTime<Utc>>,
        expired_after: Option<DateTime<Utc>>,
    ) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/users/expired", self.base_url);
        let mut params = Vec::new();
        if let Some(value) = expired_before {
            params.push(("expired_before", value))
        }
        if let Some(value) = expired_after {
            params.push(("expired_after", value))
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<String>>()
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
}
