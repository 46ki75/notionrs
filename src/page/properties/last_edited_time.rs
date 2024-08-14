use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-time>
///
/// Example last_edited_by page property value
///
/// ```json
/// {
///   "Last edited time": {
///     "id": "sv%3Fi",
///     "type": "last_edited_time",
///     "last_edited_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedTimeProperty {
    pub id: String,
    pub last_edited_time: String,
}
