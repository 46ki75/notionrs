use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseEmailProperty {
    /// Property Identifier
    #[serde(skip_serializing)]
    pub id: Option<String>,

    /// Modify the value of this field when updating the column name of the property.
    #[serde(skip_serializing)]
    pub name: String,

    /// Although it is not explicitly stated in the official documentation,
    /// you can add a description to the property by specifying this.
    #[serde(skip_serializing)]
    pub description: Option<String>,

    /// An empty object (`{}`)
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
