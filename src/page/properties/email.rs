use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#email>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"email
/// - `$.['*'].email`: A string describing an email address.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
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
///
/// When the value is not set:
///
/// ```json
/// {
///   "Email": {
///     "id": "rXuf",
///     "type": "email",
///     "email": null
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageEmailProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// A string describing an email address.
    pub email: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn unit_test_deserialize_page_email_property() {
        let json_data = r#"
        {
            "Email": {
                "id": "rXuf",
                "type": "email",
                "email": "hi@example.com"
            }
        }
        "#;

        let email_map =
            serde_json::from_str::<std::collections::HashMap<String, PageEmailProperty>>(json_data)
                .unwrap();

        let email = email_map.get("Email").unwrap();

        assert_eq!(email.id, "rXuf");
        assert_eq!(email.email, Some("hi@example.com".to_string()));
    }
}
