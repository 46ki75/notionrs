use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#email>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageEmailProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A string describing an email address.
    pub email: Option<String>,
}

impl<T> From<T> for PageEmailProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::default().email(value)
    }
}

impl std::fmt::Display for PageEmailProperty {
    /// display the email address
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.email.as_deref().unwrap_or(""))
    }
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
    pub fn deserialize_page_email_property() {
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

        assert_eq!(email.id, Some("rXuf".to_string()));
        assert_eq!(email.email, Some("hi@example.com".to_string()));
    }
}
