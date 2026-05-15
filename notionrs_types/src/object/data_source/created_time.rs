use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceCreatedTimeProperty {
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
    pub created_time: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_created_time_property() {
        let json_data = r#"
        {
            "id": "XcAf",
            "name": "Created time",
            "type": "created_time",
            "created_time": {}
        }
        "#;

        let created_time =
            serde_json::from_str::<DataSourceCreatedTimeProperty>(json_data).unwrap();

        assert_eq!(created_time.id, Some("XcAf".to_string()));
        assert_eq!(created_time.name, "Created time");
        assert_eq!(created_time.created_time, std::collections::HashMap::new());
    }

    #[test]
    fn exercise_setters() {
        let p = DataSourceCreatedTimeProperty::default()
            .id("ID")
            .name("Name")
            .description("Desc")
            .created_time(std::collections::HashMap::new());
        assert_eq!(p.id.as_deref(), Some("ID"));
        assert_eq!(p.name, "Name");
        assert_eq!(p.description.as_deref(), Some("Desc"));
        let _ = serde_json::to_string(&p).unwrap();
    }
}
