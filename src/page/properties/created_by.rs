use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#created-by>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"created_by"`
/// - `$.['*'].created_by`: A [user object](https://developers.notion.com/reference/user)
///                         containing information about the user who created the page.
///                         `created_by` can’t be updated.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example checkbox page property value
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
pub struct PageCreatedByProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// A [user object](https://developers.notion.com/reference/user)
    /// containing information about the user who created the page.
    /// `created_by` can’t be updated.
    pub created_by: crate::user::User,
}
