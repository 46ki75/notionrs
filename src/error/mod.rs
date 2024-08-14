pub mod api_error;

#[derive(thiserror::Error, Debug)]
pub enum NotionError {
    #[error("network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("notion api error: {0}")]
    NotionApiError(Box<api_error::NotionApiError>),

    #[error("notion request parameter error: {0}")]
    NotionRequestParameterError(String),

    #[error("deserialization error: {0}")]
    NotionDeserializationError(#[from] serde_json::Error),
}

impl NotionError {
    pub async fn from_response(
        res: reqwest::Response,
    ) -> Result<Option<api_error::NotionApiError>, reqwest::Error> {
        if !res.status().is_success() {
            let api_error = res.json::<api_error::NotionApiError>().await?;
            Ok(Some(api_error))
        } else {
            Ok(None)
        }
    }
}
