use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseRichTextProperty {
    pub id: Option<String>,
    pub name: String,
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
