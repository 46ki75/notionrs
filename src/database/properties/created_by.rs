use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseCreatedByProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub created_by: std::collections::HashMap<(), ()>,
}

impl DatabaseCreatedByProperty {
    /// Modify the value of this field when updating the column name of the property.
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        self.name = name.as_ref().to_string();
        self
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
    fn deserialize_database_created_by_property() {
        let json_data = r#"
        {
            "id": "%7Cy~C",
            "name": "CreatedBy",
            "type": "created_by",
            "created_by": {}
        }
        "#;

        let created_by = serde_json::from_str::<DatabaseCreatedByProperty>(json_data).unwrap();

        assert_eq!(created_by.id, Some("%7Cy~C".to_string()));
        assert_eq!(created_by.name, "CreatedBy");
        assert_eq!(created_by.created_by, std::collections::HashMap::new());
    }
}
