use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#checkbox>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"button"`
/// - `$.['*'].checkbox`: Whether the checkbox is checked (`true`) or unchecked (`false`).
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example checkbox page property value
///
/// ```json
/// {
///     "Task completed": {
///       "id": "ZI%40W",
///       "type": "checkbox",
///       "checkbox": true
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCheckboxProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// Whether the checkbox is checked (`true`) or unchecked (`false`).
    pub checkbox: bool,
}
