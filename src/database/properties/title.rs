use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseTitleProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub title: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_title_property() {
        let json_data = r#"
        {
            "id": "title",
            "name": "Project name",
            "type": "title",
            "title": {}
        }
        "#;

        let title = serde_json::from_str::<DatabaseTitleProperty>(json_data).unwrap();

        assert_eq!(title.id, Some("title".to_string()));
        assert_eq!(title.name, "Project name");
        assert_eq!(title.title, std::collections::HashMap::new());
    }
}
