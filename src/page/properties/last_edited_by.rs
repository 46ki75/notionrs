use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-by>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"__________"` // TODO: documentation replace placeholder
/// - `$.['*'].__________`: // TODO: documentation
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example __________ page property value // TODO: documentation replace placeholder
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
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub last_edited_by: crate::user::User,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // TODO: test
}
