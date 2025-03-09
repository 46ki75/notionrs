use serde::{Deserialize, Serialize};

/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"button"`
/// - `$.['*'].button`: Always an empty object
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example button page property value
///
/// ```json
/// {
///     "Button": {
///         "id": "c%60qZ",
///         "type": "button",
///         "button": {}
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageButtonProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Always `"button"`
    pub button: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for PageButtonProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
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
    use serde_json;

    #[test]
    fn deserialize_page_button_property() {
        let json_data = r#"
        {
            "Button": {
                "id": "c%60qZ",
                "type": "button",
                "button": {}
            }
        }
        "#;

        let button_map = serde_json::from_str::<
            std::collections::HashMap<String, PageButtonProperty>,
        >(json_data)
        .unwrap();

        let button = button_map.get("Button").unwrap();

        assert_eq!(button.id, Some("c%60qZ".to_string()));
        assert!(button.button.is_empty());
    }
}
