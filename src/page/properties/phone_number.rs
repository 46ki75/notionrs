use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#phone-number>
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
///   "Checkbox": {
///     "type": "phone_number",
///     "id": "Se%3Dp",
///     "phone_number": "080"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PagePhoneNumberProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub phone_number: Option<String>,
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
