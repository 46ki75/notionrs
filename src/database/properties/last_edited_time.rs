use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseLastEditedTimeProperty {
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
    pub last_edited_time: std::collections::HashMap<(), ()>,
}

impl DatabaseLastEditedTimeProperty {
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
