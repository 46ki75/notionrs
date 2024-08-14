use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#status>
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
///   "Status": {
///     "type": "status",
///     "id": "xx%7Cd",
///     "status": {
///       "id": "4a1accbf-6716-4cf2-9034-5877581fc5f6",
///       "name": "Not started",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageStatusProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub status: crate::others::select::Select,
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
