use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseCheckboxProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

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

        let checkbox = serde_json::from_str::<DatabaseCheckboxProperty>(json_data).unwrap();

        assert_eq!(checkbox.id, Some("XjE%60".to_string()));
        assert_eq!(checkbox.name, "Checkbox");
        assert_eq!(checkbox.checkbox, std::collections::HashMap::new());
    }
}
