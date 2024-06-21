use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user_template::{UserTemplateCreate, UserTemplateModify, UserTemplateResponse},
    },
};

impl MarzbanAPIClient {
    // Get user templates
    pub async fn get_user_templates(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<UserTemplateResponse>, ApiError> {
        let url = format!("{}/api/user_template", self.base_url);
        let mut params = Vec::new();
        if let Some(value) = offset {
            params.push(value)
        }
        if let Some(value) = limit {
            params.push(value)
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<UserTemplateResponse>>()
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

    // Add a user template
    pub async fn add_user_template(
        &self,
        body: &UserTemplateCreate,
    ) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .json(body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Template by this name already exists".to_string(),
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

    // Get user template with ID
    pub async fn get_user_template(&self, id: &i32) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template/{}", self.base_url, id);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
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

    // Modify user template
    pub async fn modify_user_template(
        &self,
        id: &i32,
        body: &UserTemplateModify,
    ) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template/{}", self.base_url, id);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .json(body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError(
                "User Template not found".to_string(),
            )),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Template by this name already exists".to_string(),
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

    // Remove user template
    pub async fn remove_user_template(&self, id: &i32) -> Result<String, ApiError> {
        let url = format!("{}/api/user_template/{}", self.base_url, id);
        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError(
                "User Template not found".to_string(),
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
}
