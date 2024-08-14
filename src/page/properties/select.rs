use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#select>
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
///   "Select": {
///     "type": "select",
///     "id": "chOy",
///     "select": {
///       "id": "eede87ce-52db-4b16-9931-2bc40687d697",
///       "name": "TODO",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageSelectProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub select: Option<crate::others::select::Select>,
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
