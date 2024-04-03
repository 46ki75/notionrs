use serde::{Deserialize, Serialize};

use crate::user::User;

/// https://developers.notion.com/reference/page-property-values#created-by
///
/// Example created_by page property value
///
/// ```json
/// {
///   "Created by": {
///     "id": "fR4s",
///     "type": "created_by",
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
pub struct PageCreatedBy {
    pub id: String,
    pub created_by: User,
}
