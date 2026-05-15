use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceUniqueIdProperty {
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

    pub unique_id: DataSourceUniqueIdPropertyItem,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceUniqueIdPropertyItem {
    pub prefix: Option<String>,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn data_source_unique_id_property() {
        let json = r#"{"id":"u1","name":"UID","unique_id":{"prefix":"TASK"}}"#;
        let prop: DataSourceUniqueIdProperty = serde_json::from_str(json).unwrap();
        assert_eq!(prop.name, "UID");
        assert_eq!(prop.unique_id.prefix.as_deref(), Some("TASK"));
        let _ = serde_json::to_string(&prop).unwrap();

        let item = DataSourceUniqueIdPropertyItem::default();
        assert!(item.prefix.is_none());
    }

    #[test]
    fn exercise_setters() {
        let p = DataSourceUniqueIdProperty::default()
            .id("ID")
            .name("Name")
            .description("Desc")
            .unique_id(DataSourceUniqueIdPropertyItem {
                prefix: Some("X".into()),
            });
        assert_eq!(p.id.as_deref(), Some("ID"));
        assert_eq!(p.name, "Name");
        assert_eq!(p.description.as_deref(), Some("Desc"));
        assert_eq!(p.unique_id.prefix.as_deref(), Some("X"));

        let item = DataSourceUniqueIdPropertyItem::default().prefix("Y");
        assert_eq!(item.prefix.as_deref(), Some("Y"));
    }
}
