use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseCreatedByProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
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

        let created_by = serde_json::from_str::<DatabaseCreatedByProperty>(json_data).unwrap();

        assert_eq!(created_by.id, Some("%7Cy~C".to_string()));
        assert_eq!(created_by.name, "CreatedBy");
        assert_eq!(created_by.created_by, std::collections::HashMap::new());
    }
}
