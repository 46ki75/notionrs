use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseEmailProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub email: std::collections::HashMap<(), ()>,
}

impl DatabaseEmailProperty {
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
