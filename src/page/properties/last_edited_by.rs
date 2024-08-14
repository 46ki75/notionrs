use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-by>
///
/// Example last_edited_by page property value
///
/// ```json
/// {
///   "CLast edited by": {
///     "id": "fR4s",
///     "type": "last_edited_by",
///     "created_by": {
///       "object": "user",
///       "id": "cb497a8c-1c30-4c22-87af-f8b0c1ee7389",
///       "name": "Sam",
///       "avatar_url": null,
///       "type": "person",
///       "person": {
///         "email": "info@example.com"
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedByProperty {
    pub id: String,
    pub last_edited_by: crate::user::User,
}
