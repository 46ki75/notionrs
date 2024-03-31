use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Notion API error: status {status}, code: {code}, message: {message}")]
pub struct NotionApiError {
    object: String,
    status: u16,
    code: String,
    message: String,
    request_id: String,
    developer_survey: Option<String>,
}

#[derive(Error, Debug)]
pub enum NotionError {
    #[error("network error: {0}")]
    NetworkError(#[from] ReqwestError),

    #[error("notion api error: {0}")]
    NotionApiError(Box<NotionApiError>),

    #[error("notion request parameter error: {0}")]
    NotionRequestParameterError(String),
}

impl NotionError {
    pub async fn from_response(
        res: reqwest::Response,
    ) -> Result<Option<NotionApiError>, reqwest::Error> {
        if !res.status().is_success() {
            let api_error = res.json::<NotionApiError>().await?;
            Ok(Some(api_error))
        } else {
            Ok(None)
        }
    }
}
