use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DatabaseLastEditedByProperty {
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
