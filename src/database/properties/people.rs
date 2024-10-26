use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabasePeopleProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub people: std::collections::HashMap<(), ()>,
}

impl DatabasePeopleProperty {
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

        assert_eq!(people.id, Some("FlgQ".to_string()));
        assert_eq!(people.name, "Project owner");
        assert_eq!(people.people, std::collections::HashMap::new());
    }
}
