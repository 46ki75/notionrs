use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceSelectProperty {
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

    pub select: DataSourceSelectOptionProperty,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceSelectOptionProperty {
    options: Vec<crate::object::select::Select>,
}

impl DataSourceSelectProperty {
    pub fn options(mut self, options: Vec<crate::object::select::Select>) -> Self {
        self.select.options = options;
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
    fn deserialize_database_select_property() {
        let json_data = r#"
        {
            "id": "%40Q%5BM",
            "name": "Food group",
            "type": "select",
            "select": {
                "options": [
                {
                    "id": "e28f74fc-83a7-4469-8435-27eb18f9f9de",
                    "name": "🥦Vegetable",
                    "color": "purple"
                },
                {
                    "id": "6132d771-b283-4cd9-ba44-b1ed30477c7f",
                    "name": "🍎Fruit",
                    "color": "red"
                },
                {
                    "id": "fc9ea861-820b-4f2b-bc32-44ed9eca873c",
                    "name": "💪Protein",
                    "color": "yellow"
                }
                ]
            }
        }
        "#;

        let select = serde_json::from_str::<DataSourceSelectProperty>(json_data).unwrap();

        assert_eq!(select.id, Some("%40Q%5BM".to_string()));
        assert_eq!(select.name, "Food group");

        let options = &select.select.options;
        assert_eq!(options.len(), 3);

        assert_eq!(
            options[0].id,
            Some("e28f74fc-83a7-4469-8435-27eb18f9f9de".to_string())
        );
        assert_eq!(options[0].name, "🥦Vegetable");
        assert_eq!(
            options[0].color,
            Some(crate::object::select::SelectColor::Purple)
        );

        assert_eq!(
            options[1].id,
            Some("6132d771-b283-4cd9-ba44-b1ed30477c7f".to_string())
        );
        assert_eq!(options[1].name, "🍎Fruit");
        assert_eq!(
            options[1].color,
            Some(crate::object::select::SelectColor::Red)
        );

        assert_eq!(
            options[2].id,
            Some("fc9ea861-820b-4f2b-bc32-44ed9eca873c".to_string())
        );
        assert_eq!(options[2].name, "💪Protein");
        assert_eq!(
            options[2].color,
            Some(crate::object::select::SelectColor::Yellow)
        );
    }
}
