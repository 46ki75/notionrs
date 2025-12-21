use serde::{Deserialize, Serialize};

/// [Notion API Reference](https://developers.notion.com/reference/property-object#place)
///
/// This property was added to Notion API on 2025-11-10.
/// [@notionhq/notion-sdk-js@5.4.0 Release Note](https://github.com/makenotion/notion-sdk-js/releases/tag/v5.4.0)
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourcePlaceProperty {
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

    pub place: std::collections::HashMap<(), ()>,
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_database_place_property() {
        let json_data = r#"
        {
            "id": "%60j%3Bh",
            "name": "Place",
            "description": null,
            "type": "place",
            "place": {}
        }
        "#;

        let place = serde_json::from_str::<DataSourcePlaceProperty>(json_data).unwrap();

        assert_eq!(place.id, Some("%60j%3Bh".to_string()));
        assert_eq!(place.name, "Place");
        assert_eq!(place.place, std::collections::HashMap::new());
    }
}
