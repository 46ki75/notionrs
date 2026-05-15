use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceCreatedByProperty {
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
    pub created_by: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_created_by_property() {
        let json_data = r#"
        {
            "id": "%7Cy~C",
            "name": "CreatedBy",
            "type": "created_by",
            "created_by": {}
        }
        "#;

        let created_by = serde_json::from_str::<DataSourceCreatedByProperty>(json_data).unwrap();

        assert_eq!(created_by.id, Some("%7Cy~C".to_string()));
        assert_eq!(created_by.name, "CreatedBy");
        assert_eq!(created_by.created_by, std::collections::HashMap::new());
    }

    #[test]
    fn exercise_setters() {
        let p = DataSourceCreatedByProperty::default()
            .id("ID")
            .name("Name")
            .description("Desc")
            .created_by(std::collections::HashMap::new());
        assert_eq!(p.id.as_deref(), Some("ID"));
        assert_eq!(p.name, "Name");
        assert_eq!(p.description.as_deref(), Some("Desc"));
        let _ = serde_json::to_string(&p).unwrap();
    }
}
