use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceDateProperty {
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
    pub date: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_date_property() {
        let json_data = r#"
        {
            "id": "AJP%7D",
            "name": "Task due date",
            "type": "date",
            "date": {}
        }
        "#;

        let date = serde_json::from_str::<DataSourceDateProperty>(json_data).unwrap();

        assert_eq!(date.id, Some("AJP%7D".to_string()));
        assert_eq!(date.name, "Task due date");
        assert_eq!(date.date, std::collections::HashMap::new());
    }
}
