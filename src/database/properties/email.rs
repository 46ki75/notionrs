use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseEmailProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub email: std::collections::HashMap<(), ()>,
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
    fn deserialize_database_email_property() {
        let json_data = r#"
        {
            "id": "oZbC",
            "name": "Contact email",
            "type": "email",
            "email": {}
        }
        "#;

        let email = serde_json::from_str::<DatabaseEmailProperty>(json_data).unwrap();

        assert_eq!(email.id, Some("oZbC".to_string()));
        assert_eq!(email.name, "Contact email");
        assert_eq!(email.email, std::collections::HashMap::new());
    }
}
