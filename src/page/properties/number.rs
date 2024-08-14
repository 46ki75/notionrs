use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#number>
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
/// { "Number": { "type": "number", "id": "%5Chme", "number": 20.0 } }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageNumberProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub number: Option<f64>,
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
