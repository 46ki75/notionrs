use serde::{Deserialize, Serialize};

/// Example checkbox page property value
/// ```json
/// {
///     "Button": {
///         "id": "c%60qZ",
///         "type": "button",
///         "button": {}
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageButtonProperty {
    pub id: String,
    pub button: std::collections::HashMap<String, String>,
}
