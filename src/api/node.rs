use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        node::{NodeCreate, NodeModify, NodeResponse, NodeSettings, NodesUsageResponse},
    },
};

impl MarzbanAPIClient {
    // Get Node settings
    pub async fn get_node_settings(&self) -> Result<NodeSettings, ApiError> {
        let url = format!("{}/api/node/settings", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeSettings>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Add Node
    pub async fn add_node(&self, body: &NodeCreate) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .json(body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
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

    // Get Node
    pub async fn get_node(&self, node_id: &i32) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node/{}", self.base_url, node_id);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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

    // Modify Node
    pub async fn modify_node(
        &self,
        node_id: &i32,
        body: &NodeModify,
    ) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node/{}", self.base_url, node_id);
        let response = self
            .prepare_authorized_request(reqwest::Method::PUT, &url)
            .json(body)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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

    // Remove Node
    pub async fn remove_node(&self, node_id: &i32) -> Result<String, ApiError> {
        let url = format!("{}/api/node/{}", self.base_url, node_id);
        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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

    // Get Nodes
    pub async fn get_nodes(&self) -> Result<Vec<NodeResponse>, ApiError> {
        let url = format!("{}/api/nodes", self.base_url);
        let response = self
            .prepare_authorized_request(reqwest::Method::GET, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<NodeResponse>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    // Reconnect a Node
    pub async fn reconnect_node(&self, node_id: &i32) -> Result<String, ApiError> {
        let url = format!("{}/api/node/{}/reconnect", self.base_url, node_id);
        let response = self
            .prepare_authorized_request(reqwest::Method::POST, &url)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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

    // Get Node usage
    pub async fn get_nodes_usage(
        &self,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<NodesUsageResponse, ApiError> {
        let url = format!("{}/api/nodes/usage", self.base_url);
        let mut params = Vec::new();
        if let Some(value) = start {
            params.push(value)
        }
        if let Some(value) = end {
            params.push(value)
        }

        let response = self
            .prepare_authorized_request(reqwest::Method::DELETE, &url)
            .query(&params)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodesUsageResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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
