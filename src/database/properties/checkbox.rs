use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseCheckboxProperty {
    pub id: String,
    pub name: String,
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

        assert_eq!(checkbox.id, "XjE%60");
        assert_eq!(checkbox.name, "Checkbox");
        assert_eq!(checkbox.checkbox, std::collections::HashMap::new());
    }
}
