use serde::{Deserialize, Serialize};

/// Example checkbox page property value
///
/// - `$.['*'].id`: Property identifier.
/// - `$.['*'].type`: Always `"button"`
/// - `$.['*'].button`: Always an empty object
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageButtonProperty {
    pub id: String,
    pub button: std::collections::HashMap<String, String>,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    fn unit_test_deserialize_page_button_property() {
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

        assert_eq!(button.id, "c%60qZ".to_string());
        assert!(button.button.is_empty());
    }
}
