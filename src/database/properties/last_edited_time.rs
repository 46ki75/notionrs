use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseLastEditedTimeProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub last_edited_time: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_last_edited_time_property() {
        let json_data = r#"
        {
            "id": "jGdo",
            "name": "Last edited time",
            "type": "last_edited_time",
            "last_edited_time": {}
        }
        "#;

        let last_edited_time =
            serde_json::from_str::<DatabaseLastEditedTimeProperty>(json_data).unwrap();

        assert_eq!(last_edited_time.id, Some("jGdo".to_string()));
        assert_eq!(last_edited_time.name, "Last edited time");
        assert_eq!(
            last_edited_time.last_edited_time,
            std::collections::HashMap::new()
        );
    }
}
