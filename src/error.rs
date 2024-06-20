use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("API error: {0}")]
    ApiResponseError(String),

    #[error("Unexpected API response")]
    UnexpectedResponse,
}
