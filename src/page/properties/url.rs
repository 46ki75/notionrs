use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#url>
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
///   "URL": {
///     "type": "url",
///     "id": "h_AH",
///     "url": "https://developers.notion.com/reference/page-property-values#url"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUrlProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub url: Option<String>,
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