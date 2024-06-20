use std::sync::{Arc, Mutex};

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{auth::BodyAdminTokenApiAdminTokenPost, errors::HTTPValidationError, token::Token},
};

impl MarzbanAPIClient {
    pub async fn admin_token(
        &self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<Token, ApiError> {
        let url = format!("{}/api/admin/token", self.base_url);
        let response = self.client.post(url).form(&auth).send().await?;

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

    pub async fn authenticate(
        &mut self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<(), ApiError> {
        let token = self.admin_token(auth).await?;
        self.token = Arc::new(Mutex::new(Some(token.access_token)));
        Ok(())
    }
}
