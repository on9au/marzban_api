use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        admin::{Admin, AdminCreate, AdminModify},
        auth::BodyAdminTokenApiAdminTokenPost,
        errors::HTTPValidationError,
        token::Token,
    },
};

use super::user;

impl MarzbanAPIClient {
    pub async fn admin_token(
        &self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<Token, ApiError> {
        let url = format!("{}/api/admin/token", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .form(&auth)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Token>()
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

    // This method takes in a BodyAdminTokenApiAdminTokenPost, and if auth is successful, makes the returned token into the MarzbanAPIClient struct.
    pub async fn authenticate(
        &self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<(), ApiError> {
        let token = self.admin_token(auth).await?;
        let mut token_lock = self.token.lock().unwrap();
        *token_lock = Some(token.access_token);
        Ok(())
    }

    pub async fn get_current_admin(&self) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    pub async fn create_admin(&self, body: AdminCreate) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .form(&body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Admin already exists".to_string(),
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

    pub async fn modify_admin(
        &self,
        admin_username: &str,
        body: AdminModify,
    ) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin/{}", self.base_url, admin_username);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .form(&body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Admin not found".to_string())),
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

    pub async fn delete_admin(&self, admin_username: &str) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin/{}", self.base_url, admin_username);
        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Admin not found".to_string())),
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

    pub async fn get_admins(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        username: Option<&str>,
    ) -> Result<Vec<Admin>, ApiError> {
        let url = format!("{}/api/admin/", self.base_url);
        let mut params = Vec::new();
        if let Some(value) = offset {
            params.push(value.to_string())
        }
        if let Some(value) = limit {
            params.push(value.to_string())
        }
        if let Some(value) = username {
            params.push(value.to_string())
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<Admin>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {:?}",
                    error_response
                )))
            }
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
