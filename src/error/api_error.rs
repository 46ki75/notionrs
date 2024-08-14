use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The Notion API error is returned in the following format. We'll need to deserialize this.
///
/// ```json
/// {
///     "object": "error",
///     "status": 403,
///     "code": "restricted_resource",
///     "message": "Insufficient permissions for this endpoint.",
///     "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
///     "request_id": "2cccb738-bf60-4e9b-bb6b-24a87fb17ef3"
/// }
/// ```
#[derive(Error, Debug, Deserialize, Serialize)]
#[error("Notion API error: status {status}, code: {code}, message: {message}")]
pub struct NotionApiError {
    /// always "error"
    object: String,

    /// HTTP Status Code ( `4xx` or `5xx` )
    status: u16,

    /// Error code
    code: String,

    /// Error details
    message: String,

    /// Request identifier
    request_id: String,

    /// URL for the developer survey
    developer_survey: Option<String>,
}
