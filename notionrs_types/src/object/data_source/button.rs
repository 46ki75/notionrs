use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceButtonProperty {
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
    pub button: std::collections::HashMap<(), ()>,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn data_source_button_property() {
        let json = r#"{"id":"abc","name":"Btn","button":{}}"#;
        let prop: DataSourceButtonProperty = serde_json::from_str(json).unwrap();
        assert_eq!(prop.id.as_deref(), Some("abc"));
        assert_eq!(prop.name, "Btn");
        let _ = serde_json::to_string(&prop).unwrap();
    }

    #[test]
    fn exercise_setters() {
        let p = DataSourceButtonProperty::default()
            .id("ID")
            .name("Name")
            .description("Desc")
            .button(std::collections::HashMap::new());
        assert_eq!(p.id.as_deref(), Some("ID"));
        assert_eq!(p.name, "Name");
        assert_eq!(p.description.as_deref(), Some("Desc"));
        let _ = serde_json::to_string(&p).unwrap();
    }
}
