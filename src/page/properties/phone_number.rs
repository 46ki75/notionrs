use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#phone-number>
///
/// Example phone_number page property value
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
    pub id: String,
    pub phone_number: Option<String>,
}
