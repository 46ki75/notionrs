use std::collections::HashMap;

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
pub struct PageButton {
    pub id: String,
    pub button: HashMap<String, String>,
}
