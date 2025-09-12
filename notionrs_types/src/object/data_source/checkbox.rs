use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceCheckboxProperty {
    /// Property Identifier
    #[serde(skip_serializing)]
    pub id: Option<String>,

    /// Modify the value of this field when updating the column name of the property.
    #[serde(skip_serializing)]
    pub name: String,

    /// Although it is not explicitly stated in the official documentation,
    /// you can add a description to the property by specifying this.
    #[serde(skip_serializing)]
    pub description: Option<String>,

    /// An empty object (`{}`)
    pub checkbox: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_checkbox_property() {
        let json_data = r#"
        {
            "id": "XjE%60",
            "name": "Checkbox",
            "type": "checkbox",
            "checkbox": {}
        }
        "#;

        let checkbox = serde_json::from_str::<DataSourceCheckboxProperty>(json_data).unwrap();

        assert_eq!(checkbox.id, Some("XjE%60".to_string()));
        assert_eq!(checkbox.name, "Checkbox");
        assert_eq!(checkbox.checkbox, std::collections::HashMap::new());
    }
}
