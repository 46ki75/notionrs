use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseFilesProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub files: std::collections::HashMap<(), ()>,
}

impl DatabaseFilesProperty {
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
