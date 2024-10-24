use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseFilesProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub files: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_files_property() {
        let json_data = r#"
        {
            "id": "pb%3E%5B",
            "name": "Product image",
            "type": "files",
            "files": {}
        }
        "#;

        let files = serde_json::from_str::<DatabaseFilesProperty>(json_data).unwrap();

        assert_eq!(files.id, Some("pb%3E%5B".to_string()));
        assert_eq!(files.name, "Product image");
        assert_eq!(files.files, std::collections::HashMap::new());
    }
}
