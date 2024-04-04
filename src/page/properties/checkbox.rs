use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#checkbox
///
/// Example checkbox page property value
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
    pub id: String,
    pub checkbox: bool,
}
