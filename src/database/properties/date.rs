use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseDateProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub date: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_date_property() {
        let json_data = r#"
        {
            "id": "AJP%7D",
            "name": "Task due date",
            "type": "date",
            "date": {}
        }
        "#;

        let date = serde_json::from_str::<DatabaseDateProperty>(json_data).unwrap();

        assert_eq!(date.id, "AJP%7D");
        assert_eq!(date.name, "Task due date");
        assert_eq!(date.date, std::collections::HashMap::new());
    }
}
