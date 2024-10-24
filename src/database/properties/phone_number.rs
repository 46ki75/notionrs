use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabasePhoneNumberProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub phone_number: std::collections::HashMap<(), ()>,
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
            "id": "ULHa",
            "name": "Contact phone number",
            "type": "phone_number",
            "phone_number": {}
        }
        "#;

        let phone_number = serde_json::from_str::<DatabasePhoneNumberProperty>(json_data).unwrap();

        assert_eq!(phone_number.id, Some("ULHa".to_string()));
        assert_eq!(phone_number.name, "Contact phone number");
        assert_eq!(phone_number.phone_number, std::collections::HashMap::new());
    }
}
