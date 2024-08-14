pub mod api_error;

#[derive(thiserror::Error, Debug)]
pub enum NotionError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("notion api error: {0}")]
    NotionApiError(Box<api_error::NotionApiError>),

    /// Since we are using the Builder pattern, it is possible to send
    /// a request even if some parameters are missing. In such cases
    /// where the request parameters are insufficient, we will throw this error.
    #[error("notion request parameter error: {0}")]
    NotionRequestParameterError(String),

    #[error("deserialization error: {0}")]
    NotionDeserializationError(#[from] serde_json::Error),

    #[error("unknown error: {0}")]
    NotionUnknownError(String),
}
