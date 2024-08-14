use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#last-edited-time>
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
///   "Last edited time": {
///     "id": "sv%3Fi",
///     "type": "last_edited_time",
///     "last_edited_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedTimeProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub last_edited_time: String,
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
