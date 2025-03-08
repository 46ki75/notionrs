#![deny(missing_docs)]

//! Errors that can happen when using notionrs

/// Errors that can happen when using notionrs
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error occurs when the request fails due to a network issue.
    #[error("Network error: {0}")]
    Network(String),

    /// This error occurs when parsing the HTTP body fails.
    #[error("HTTP body parse error: {0}")]
    BodyParse(String),

    /// This error occurs when the HTTP response has a non-200 status code.
    #[error("HTTP error {status}: {message}")]
    Http {
        /// HTTP status code (e.g. 404)
        status: u16,
        /// Error message
        message: String,
    },

    /// This library follows the Builder pattern, allowing requests to be sent even with missing parameters.
    /// If request parameters are insufficient, this error will be returned.
    ///
    /// If invalid parameters are passed, the Notion API will return a 400 Bad Request error -> `Error::Http`.
    #[error("Notion request parameter error: {0}")]
    RequestParameter(String),

    /// This error occurs when serialization or deserialization fails.
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Error response from the Notion API
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ErrorResponse {
    /// always "error"
    pub object: String,

    /// HTTP Status Code ( `4xx` or `5xx` )
    pub status: u16,

    /// Error code
    pub code: String,

    /// Error details
    pub message: String,

    /// Request identifier
    pub request_id: Option<String>,

    /// URL for the developer survey
    pub developer_survey: Option<String>,
}

impl Error {
    pub(crate) async fn try_from_response_async(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();

        let error_body = match response.text().await{
            Err(_) =>{
                return crate::error::Error::Http {
                    status,
                    message: "An error occurred, but failed to retrieve the error details from the response body.".to_string(),
                }},
            Ok(body) => body
        };

        let error_json = serde_json::from_str::<crate::error::ErrorResponse>(&error_body).ok();

        let error_message = match error_json {
            Some(e) => e.message,
            None => format!("{:?}", error_body),
        };

        crate::error::Error::Http {
            status,
            message: error_message,
        }
    }
}
