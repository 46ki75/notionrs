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
        /// Notion's request identifier, taken from the error body's `request_id`
        /// field, or from the `x-notion-request-id` response header when the body
        /// doesn't provide one. Include it when reporting an issue to Notion.
        request_id: Option<String>,
        /// Value of the `cf-ray` response header, when present.
        ///
        /// A response carrying a Ray ID but no [`request_id`](Error::Http) was
        /// answered at the network edge before it reached the Notion API — for
        /// example by a network security rule. Include it when reporting an issue
        /// to Notion.
        ray_id: Option<String>,
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
    /// Notion's request identifier, when this error carries one.
    ///
    /// Only [`Error::Http`] carries it; every other variant returns `None`.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Http { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// The Cloudflare Ray ID of the response, when this error carries one.
    ///
    /// Only [`Error::Http`] carries it; every other variant returns `None`.
    pub fn ray_id(&self) -> Option<&str> {
        match self {
            Error::Http { ray_id, .. } => ray_id.as_deref(),
            _ => None,
        }
    }

    pub(crate) async fn try_from_response_async(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let headers = response.headers().clone();

        let error_body = response.text().await.ok();

        Self::http_from_parts(status, &headers, error_body.as_deref())
    }

    /// Builds an [`Error::Http`] from the pieces of a failed response.
    ///
    /// `body` is `None` when the response body could not be read.
    fn http_from_parts(
        status: u16,
        headers: &reqwest::header::HeaderMap,
        body: Option<&str>,
    ) -> Self {
        let ray_id = header_value(headers, "cf-ray");
        let header_request_id = header_value(headers, "x-notion-request-id");

        let Some(body) = body else {
            return Error::Http {
                status,
                message: "An error occurred, but failed to retrieve the error details from the response body.".to_string(),
                request_id: header_request_id,
                ray_id,
            };
        };

        match serde_json::from_str::<crate::error::ErrorResponse>(body) {
            // A well-formed Notion API error takes precedence over any header-derived diagnostics.
            Ok(error_response) => Error::Http {
                status,
                message: error_response.message,
                request_id: error_response.request_id.or(header_request_id),
                ray_id,
            },
            Err(_) => {
                // An unrecognized response with a Ray ID but no Notion request ID was
                // answered before it reached the Notion API. Its body is usually HTML,
                // so surface the Ray ID instead of dumping it into the message.
                let message = match (&ray_id, &header_request_id) {
                    (Some(ray_id), None) => build_edge_response_message(
                        status,
                        header_value(headers, "content-type").as_deref(),
                        ray_id,
                    ),
                    _ => format!("{:?}", body),
                };

                Error::Http {
                    status,
                    message,
                    request_id: header_request_id,
                    ray_id,
                }
            }
        }
    }
}

/// Reads a header as a `String`, ignoring values that aren't valid visible ASCII.
///
/// Header names are matched case-insensitively by [`reqwest::header::HeaderMap`].
fn header_value(headers: &reqwest::header::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

/// Builds the message for an unrecognized response generated at the network edge.
fn build_edge_response_message(status: u16, content_type: Option<&str>, ray_id: &str) -> String {
    let content_type_note = match content_type {
        Some(content_type) => format!(" (content-type: {content_type})"),
        None => String::new(),
    };

    let blocked_request_note = if status == 403 {
        " This may mean the request was blocked by a network security rule."
    } else {
        ""
    };

    format!(
        "The response was returned by Notion's edge proxy before reaching the Notion \
         API{content_type_note}.{blocked_request_note} Cloudflare Ray ID: {ray_id}. \
         Include this ID when contacting Notion support."
    )
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn header_map(pairs: &[(&str, &str)]) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        for (name, value) in pairs {
            headers.insert(
                reqwest::header::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                reqwest::header::HeaderValue::from_str(value).unwrap(),
            );
        }

        headers
    }

    /// Unpacks an [`Error::Http`], panicking on any other variant.
    fn unwrap_http(error: Error) -> (u16, String, Option<String>, Option<String>) {
        match error {
            Error::Http {
                status,
                message,
                request_id,
                ray_id,
            } => (status, message, request_id, ray_id),
            other => panic!("expected Error::Http, got {other:?}"),
        }
    }

    const NOTION_ERROR_BODY: &str = r#"
    {
        "object": "error",
        "status": 404,
        "code": "object_not_found",
        "message": "Could not find page.",
        "request_id": "body-request-id"
    }
    "#;

    #[test]
    fn http_from_parts_prefers_well_formed_notion_error() {
        let headers = header_map(&[
            ("cf-ray", "abc123-NRT"),
            ("x-notion-request-id", "header-request-id"),
        ]);

        let (status, message, request_id, ray_id) = unwrap_http(Error::http_from_parts(
            404,
            &headers,
            Some(NOTION_ERROR_BODY),
        ));

        assert_eq!(status, 404);
        assert_eq!(message, "Could not find page.");
        // The body's own request_id wins over the header.
        assert_eq!(request_id.as_deref(), Some("body-request-id"));
        assert_eq!(ray_id.as_deref(), Some("abc123-NRT"));
    }

    #[test]
    fn http_from_parts_falls_back_to_request_id_header() {
        let body = r#"
        {
            "object": "error",
            "status": 429,
            "code": "rate_limited",
            "message": "Rate limited."
        }
        "#;

        let headers = header_map(&[("x-notion-request-id", "header-request-id")]);

        let (_, message, request_id, ray_id) =
            unwrap_http(Error::http_from_parts(429, &headers, Some(body)));

        assert_eq!(message, "Rate limited.");
        assert_eq!(request_id.as_deref(), Some("header-request-id"));
        assert_eq!(ray_id, None);
    }

    #[test]
    fn http_from_parts_matches_headers_case_insensitively() {
        let headers = header_map(&[("CF-Ray", "abc123-NRT"), ("X-Notion-Request-Id", "req-1")]);

        let (_, _, request_id, ray_id) = unwrap_http(Error::http_from_parts(
            404,
            &headers,
            Some(r#"{"not":"a notion error"}"#),
        ));

        assert_eq!(request_id.as_deref(), Some("req-1"));
        assert_eq!(ray_id.as_deref(), Some("abc123-NRT"));
    }

    #[test]
    fn http_from_parts_diagnoses_edge_generated_403() {
        let headers = header_map(&[("cf-ray", "abc123-NRT"), ("content-type", "text/html")]);

        let (status, message, request_id, ray_id) = unwrap_http(Error::http_from_parts(
            403,
            &headers,
            Some("<html><body>Blocked</body></html>"),
        ));

        assert_eq!(status, 403);
        assert_eq!(
            message,
            "The response was returned by Notion's edge proxy before reaching the Notion API \
             (content-type: text/html). This may mean the request was blocked by a network \
             security rule. Cloudflare Ray ID: abc123-NRT. Include this ID when contacting \
             Notion support."
        );
        // The HTML body is kept out of the message.
        assert!(!message.contains("<html>"));
        assert_eq!(request_id, None);
        assert_eq!(ray_id.as_deref(), Some("abc123-NRT"));
        assert_eq!(
            Error::http_from_parts(403, &headers, Some("<html></html>")).to_string(),
            format!("HTTP error 403: {message}")
        );
    }

    #[test]
    fn http_from_parts_diagnoses_edge_generated_non_403_without_content_type() {
        let headers = header_map(&[("cf-ray", "def456-NRT")]);

        let (_, message, _, _) =
            unwrap_http(Error::http_from_parts(502, &headers, Some("Bad Gateway")));

        assert_eq!(
            message,
            "The response was returned by Notion's edge proxy before reaching the Notion API. \
             Cloudflare Ray ID: def456-NRT. Include this ID when contacting Notion support."
        );
        assert!(!message.contains("content-type"));
        assert!(!message.contains("network security rule"));
    }

    #[test]
    fn http_from_parts_keeps_raw_body_when_notion_answered() {
        // A Ray ID together with a Notion request ID means the response did reach the
        // API, so the body is still the best available diagnostic.
        let headers = header_map(&[("cf-ray", "abc123-NRT"), ("x-notion-request-id", "req-1")]);

        let (_, message, request_id, ray_id) =
            unwrap_http(Error::http_from_parts(500, &headers, Some("not json")));

        assert_eq!(message, "\"not json\"");
        assert_eq!(request_id.as_deref(), Some("req-1"));
        assert_eq!(ray_id.as_deref(), Some("abc123-NRT"));
    }

    #[test]
    fn http_from_parts_keeps_raw_body_without_ray_id() {
        let (_, message, request_id, ray_id) = unwrap_http(Error::http_from_parts(
            500,
            &reqwest::header::HeaderMap::new(),
            Some("not json"),
        ));

        assert_eq!(message, "\"not json\"");
        assert_eq!(request_id, None);
        assert_eq!(ray_id, None);
    }

    #[test]
    fn http_from_parts_without_readable_body() {
        let headers = header_map(&[("cf-ray", "abc123-NRT")]);

        let (status, message, request_id, ray_id) =
            unwrap_http(Error::http_from_parts(500, &headers, None));

        assert_eq!(status, 500);
        assert_eq!(
            message,
            "An error occurred, but failed to retrieve the error details from the response body."
        );
        assert_eq!(request_id, None);
        assert_eq!(ray_id.as_deref(), Some("abc123-NRT"));
    }

    #[test]
    fn request_id_and_ray_id_accessors() {
        let error = Error::http_from_parts(
            403,
            &header_map(&[("cf-ray", "abc123-NRT"), ("x-notion-request-id", "req-1")]),
            Some("nope"),
        );
        assert_eq!(error.request_id(), Some("req-1"));
        assert_eq!(error.ray_id(), Some("abc123-NRT"));

        let error = Error::Network("connection reset".to_string());
        assert_eq!(error.request_id(), None);
        assert_eq!(error.ray_id(), None);
    }

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
