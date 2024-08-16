use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#checkbox>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"checkbox"`
/// - `$.['*'].checkbox`: Whether the checkbox is checked (`true`) or unchecked (`false`).
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example checkbox page property value
///
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
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    /// Whether the checkbox is checked (`true`) or unchecked (`false`).
    pub checkbox: bool,
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
    fn deserialize_page_checkbox_property() {
        let json_data = r#"
        {
            "Task completed": {
                "id": "ZI%40W",
                "type": "checkbox",
                "checkbox": true
            }
        }
        "#;

        let checkbox_map = serde_json::from_str::<
            std::collections::HashMap<String, PageCheckboxProperty>,
        >(json_data)
        .unwrap();

        let checkbox = checkbox_map.get("Task completed").unwrap();

        assert_eq!(checkbox.id, "ZI%40W".to_string());
        assert!(checkbox.checkbox);
    }
}
