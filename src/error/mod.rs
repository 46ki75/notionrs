pub mod api_error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("notion api error: {0}")]
    Api(Box<api_error::ApiError>),

    /// Since we are using the Builder pattern, it is possible to send
    /// a request even if some parameters are missing. In such cases
    /// where the request parameters are insufficient, we will throw this error.
    #[error("notion request parameter error: {0}")]
    RequestParameter(String),

    #[error("deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("unknown error: {0}")]
    Unknown(String),
}
