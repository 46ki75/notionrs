use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#email
///
/// Example email page property value
///
/// ```json
/// {
///   "Email": {
///     "id": "rXuf",
///     "type": "email",
///     "email": "hi@example.com"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageEmailProperty {
    pub id: String,
    pub email: Option<String>,
}
