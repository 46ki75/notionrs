use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseLastEditedByProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub last_edited_by: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_last_edited_by_property() {
        let json_data = r#"
        {
            "id": "%7Cy~C",
            "name": "LastEditedBy",
            "type": "last_edited_by",
            "last_edited_by": {}
        }
        "#;

        let last_edited_by =
            serde_json::from_str::<DatabaseLastEditedByProperty>(json_data).unwrap();

        assert_eq!(last_edited_by.id, Some("%7Cy~C".to_string()));
        assert_eq!(last_edited_by.name, "LastEditedBy");
        assert_eq!(
            last_edited_by.last_edited_by,
            std::collections::HashMap::new()
        );
    }
}
