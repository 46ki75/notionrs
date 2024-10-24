use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseRichTextProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

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
