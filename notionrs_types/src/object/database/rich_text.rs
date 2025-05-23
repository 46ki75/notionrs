use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DatabaseRichTextProperty {
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
    pub rich_text: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_rich_text_property() {
        let json_data = r#"
        {
            "id": "NZZ%3B",
            "name": "Project description",
            "type": "rich_text",
            "rich_text": {}
        }
        "#;

        let rich_text = serde_json::from_str::<DatabaseRichTextProperty>(json_data).unwrap();

        assert_eq!(rich_text.id, Some("NZZ%3B".to_string()));
        assert_eq!(rich_text.name, "Project description");
        assert_eq!(rich_text.rich_text, std::collections::HashMap::new());
    }
}
