use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabasePeopleProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub people: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_people_property() {
        let json_data = r#"
        {
            "id": "FlgQ",
            "name": "Project owner",
            "type": "people",
            "people": {}
        }
        "#;

        let people = serde_json::from_str::<DatabasePeopleProperty>(json_data).unwrap();

        assert_eq!(people.id, "FlgQ");
        assert_eq!(people.name, "Project owner");
        assert_eq!(people.people, std::collections::HashMap::new());
    }
}
