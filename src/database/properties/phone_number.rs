use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabasePhoneNumberProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub phone_number: std::collections::HashMap<(), ()>,
}

impl DatabasePhoneNumberProperty {
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
