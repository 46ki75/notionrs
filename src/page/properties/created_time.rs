use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#created-time
///
/// Example created_time page property value
///
/// ```json
/// {
///   "Created time": {
///     "id": "sv%3Fi",
///     "type": "created_time",
///     "created_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCreatedTimeProperty {
    pub id: String,
    pub created_time: String,
}
