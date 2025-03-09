use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DatabaseFilesProperty {
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
