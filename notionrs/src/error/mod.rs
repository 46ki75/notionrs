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

    /// This error occurs when serialization or deserialization fails (JSON).
    #[error("Serialization/Deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// This error occurs when serialization or deserialization fails (URL-encoded).
    #[error("Serialization/Deserialization error: {0}")]
    SerdeUrlEncodedSerialize(#[from] serde_urlencoded::ser::Error),

    /// This error occurs when a synchronous response was expected (e.g. via
    /// `into_page()`/`into_markdown()`), but the request was instead accepted
    /// for asynchronous processing (see `allow_async`). Use the async task ID
    /// with `Client::get_async_task` to poll for the result.
    #[error(
        "Expected a synchronous response, but the request was accepted as async task `{task_id}`"
    )]
    UnexpectedAsyncTask {
        /// The ID of the async task that was returned instead.
        task_id: String,
    },
}

/// Error code returned by the Notion API.
///
/// See <https://developers.notion.com/reference/errors> for details.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    /// The request body could not be decoded.
    InvalidJson,
    /// The request URL is not valid.
    InvalidRequestUrl,
    /// This request is not supported.
    InvalidRequest,
    /// The request is missing the `Notion-Version` header.
    MissingVersion,
    /// The bearer token is not valid.
    Unauthorized,
    /// Given the bearer token used, the client doesn't have permission to perform this operation.
    RestrictedResource,
    /// The body of the request is not valid.
    ValidationError,
    /// The resource does not exist.
    ObjectNotFound,
    /// The transaction could not be completed, potentially due to a data collision.
    ConflictError,
    /// The request exceeds the rate limit.
    RateLimited,
    /// The request would return too many rows.
    RowLimitExceeded,
    /// An unexpected error occurred on the Notion side.
    InternalServerError,
    /// Notion is unavailable.
    ServiceUnavailable,
    /// Notion is temporarily overloaded and could not process the request.
    ServiceOverload,
    /// The request timed out at the gateway.
    GatewayTimeout,
    /// An unknown error code.
    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiErrorCode::InvalidJson => write!(f, "invalid_json"),
            ApiErrorCode::InvalidRequestUrl => write!(f, "invalid_request_url"),
            ApiErrorCode::InvalidRequest => write!(f, "invalid_request"),
            ApiErrorCode::MissingVersion => write!(f, "missing_version"),
            ApiErrorCode::Unauthorized => write!(f, "unauthorized"),
            ApiErrorCode::RestrictedResource => write!(f, "restricted_resource"),
            ApiErrorCode::ValidationError => write!(f, "validation_error"),
            ApiErrorCode::ObjectNotFound => write!(f, "object_not_found"),
            ApiErrorCode::ConflictError => write!(f, "conflict_error"),
            ApiErrorCode::RateLimited => write!(f, "rate_limited"),
            ApiErrorCode::RowLimitExceeded => write!(f, "row_limit_exceeded"),
            ApiErrorCode::InternalServerError => write!(f, "internal_server_error"),
            ApiErrorCode::ServiceUnavailable => write!(f, "service_unavailable"),
            ApiErrorCode::ServiceOverload => write!(f, "service_overload"),
            ApiErrorCode::GatewayTimeout => write!(f, "gateway_timeout"),
            ApiErrorCode::Unknown(code) => write!(f, "{}", code),
        }
    }
}

/// Error response from the Notion API
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ErrorResponse {
    /// always "error"
    pub object: String,

    /// HTTP Status Code ( `4xx` or `5xx` )
    pub status: u16,

    /// Error code
    pub code: ApiErrorCode,

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

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn unexpected_async_task_display() {
        let error = Error::UnexpectedAsyncTask {
            task_id: "task-id-123".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Expected a synchronous response, but the request was accepted as async task `task-id-123`"
        );
    }

    #[test]
    fn deserialize_api_error_code_gateway_timeout() {
        let json = r#""gateway_timeout""#;
        let code: ApiErrorCode = serde_json::from_str(json).unwrap();
        assert_eq!(code, ApiErrorCode::GatewayTimeout);
    }

    #[test]
    fn deserialize_api_error_code_known_codes() {
        let test_cases = vec![
            (r#""invalid_json""#, ApiErrorCode::InvalidJson),
            (r#""invalid_request_url""#, ApiErrorCode::InvalidRequestUrl),
            (r#""invalid_request""#, ApiErrorCode::InvalidRequest),
            (r#""missing_version""#, ApiErrorCode::MissingVersion),
            (r#""unauthorized""#, ApiErrorCode::Unauthorized),
            (r#""restricted_resource""#, ApiErrorCode::RestrictedResource),
            (r#""validation_error""#, ApiErrorCode::ValidationError),
            (r#""object_not_found""#, ApiErrorCode::ObjectNotFound),
            (r#""conflict_error""#, ApiErrorCode::ConflictError),
            (r#""rate_limited""#, ApiErrorCode::RateLimited),
            (r#""row_limit_exceeded""#, ApiErrorCode::RowLimitExceeded),
            (
                r#""internal_server_error""#,
                ApiErrorCode::InternalServerError,
            ),
            (r#""service_unavailable""#, ApiErrorCode::ServiceUnavailable),
            (r#""service_overload""#, ApiErrorCode::ServiceOverload),
            (r#""gateway_timeout""#, ApiErrorCode::GatewayTimeout),
        ];

        for (json, expected) in test_cases {
            let code: ApiErrorCode = serde_json::from_str(json).unwrap();
            assert_eq!(code, expected);
        }
    }

    #[test]
    fn deserialize_api_error_code_unknown() {
        let json = r#""some_future_error_code""#;
        let code: ApiErrorCode = serde_json::from_str(json).unwrap();
        assert_eq!(
            code,
            ApiErrorCode::Unknown("some_future_error_code".to_string())
        );
    }

    #[test]
    fn serialize_api_error_code() {
        let json = serde_json::to_string(&ApiErrorCode::GatewayTimeout).unwrap();
        assert_eq!(json, r#""gateway_timeout""#);
    }

    #[test]
    fn deserialize_error_response_with_gateway_timeout() {
        let json = r#"
        {
            "object": "error",
            "status": 504,
            "code": "gateway_timeout",
            "message": "The request timed out.",
            "request_id": "abc123"
        }
        "#;

        let error: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(error.status, 504);
        assert_eq!(error.code, ApiErrorCode::GatewayTimeout);
        assert_eq!(error.message, "The request timed out.");
    }

    #[test]
    fn api_error_code_display() {
        assert_eq!(ApiErrorCode::GatewayTimeout.to_string(), "gateway_timeout");
        assert_eq!(
            ApiErrorCode::InternalServerError.to_string(),
            "internal_server_error"
        );
        assert_eq!(
            ApiErrorCode::ServiceOverload.to_string(),
            "service_overload"
        );
        assert_eq!(
            ApiErrorCode::RowLimitExceeded.to_string(),
            "row_limit_exceeded"
        );
        assert_eq!(ApiErrorCode::MissingVersion.to_string(), "missing_version");
        assert_eq!(
            ApiErrorCode::Unknown("custom".to_string()).to_string(),
            "custom"
        );
    }
}
