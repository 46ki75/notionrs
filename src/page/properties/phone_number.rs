use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#phone-number>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"phone_number"`
/// - `$.['*'].phone_number`: A string representing a phone number. No phone number format is enforced.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example phone_number page property value
///
/// ```json
/// {
///   "Phone Number": {
///     "type": "phone_number",
///     "id": "Se%3Dp",
///     "phone_number": "415-202-4776"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PagePhoneNumberProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string representing a phone number. No phone number format is enforced.
    pub phone_number: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_phone_number_property() {
        let json_data = r#"
        {
            "Phone Number": {
                "type": "phone_number",
                "id": "Se%3Dp",
                "phone_number": "415-202-4776"
            }
        }
        "#;

        let phone_number_map = serde_json::from_str::<
            std::collections::HashMap<String, PagePhoneNumberProperty>,
        >(json_data)
        .unwrap();

        let phone_number = phone_number_map.get("Phone Number").unwrap();

        assert_eq!(phone_number.id, Some("Se%3Dp".to_string()));
        assert_eq!(phone_number.phone_number, Some("415-202-4776".to_string()));
    }
}
