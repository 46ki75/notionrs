#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// This error occurs when the HTTP status code is not 200.
    #[error("HTTP error: {status}: {message}")]
    Http { status: u16, message: String },

    /// Since this library follows the Builder pattern, requests can be sent even if some parameters are missing.
    /// In such cases where the request parameters are insufficient, we will throw this error.
    ///
    /// If invalid parameters are passed, the Notion API will return a 400 Bad Request error -> `HttpBadRequest`.
    #[error("notion request parameter error: {0}")]
    RequestParameter(String),

    #[error("deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("color conversion error: {0}")]
    Color(String),

    #[error("unknown error: {0}")]
    Unknown(String),

    /// If you want to handle multiple errors collectively in the `Error` enum of the `notionrs` crate,
    /// use this variant to create your own custom error.
    #[error("custom error: {0}")]
    Custom(String),
}

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
