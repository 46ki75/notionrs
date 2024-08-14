use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#unique_id>
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
///   "ID": {
///     "id": "mBKy",
///     "type": "unique_id",
///     "unique_id": {
///       "prefix": "TES",
///       "number": 434
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdProperty {
    pub id: String,

    // TODO: documentation
    pub unique_id: PageUniqueIdPropertyParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdPropertyParameter {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub prefix: Option<String>,
    pub number: u64,
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
