use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseCreatedTimeProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub created_time: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_created_time_property() {
        let json_data = r#"
        {
            "id": "XcAf",
            "name": "Created time",
            "type": "created_time",
            "created_time": {}
        }
        "#;

        let created_time = serde_json::from_str::<DatabaseCreatedTimeProperty>(json_data).unwrap();

        assert_eq!(created_time.id, Some("XcAf".to_string()));
        assert_eq!(created_time.name, "Created time");
        assert_eq!(created_time.created_time, std::collections::HashMap::new());
    }
}
