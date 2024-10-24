use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseUrlProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    
    pub url: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_url_property() {
        let json_data = r#"
        {
            "id": "BZKU",
            "name": "Project URL",
            "type": "url",
            "url": {}
        }
        "#;

        let url = serde_json::from_str::<DatabaseUrlProperty>(json_data).unwrap();

        assert_eq!(url.id, Some("BZKU".to_string()));
        assert_eq!(url.name, "Project URL");
        assert_eq!(url.url, std::collections::HashMap::new());
    }
}
