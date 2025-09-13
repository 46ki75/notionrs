use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceMultiSelectProperty {
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

    pub multi_select: DataSourceMultiSelectOptionProperty,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceMultiSelectOptionProperty {
    pub options: Vec<crate::object::select::Select>,
}

impl DataSourceMultiSelectProperty {
    pub fn options(mut self, options: Vec<crate::object::select::Select>) -> Self {
        self.multi_select.options = options;
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
    fn deserialize_database_multi_select_property() {
        let json_data = r#"
        {
            "id": "flsb",
            "name": "Store availability",
            "type": "multi_select",
            "multi_select": {
                "options": [
                {
                    "id": "5de29601-9c24-4b04-8629-0bca891c5120",
                    "name": "Duc Loi Market",
                    "color": "blue"
                },
                {
                    "id": "385890b8-fe15-421b-b214-b02959b0f8d9",
                    "name": "Rainbow Grocery",
                    "color": "gray"
                },
                {
                    "id": "72ac0a6c-9e00-4e8c-80c5-720e4373e0b9",
                    "name": "Nijiya Market",
                    "color": "purple"
                },
                {
                    "id": "9556a8f7-f4b0-4e11-b277-f0af1f8c9490",
                    "name": "Gus's Community Market",
                    "color": "yellow"
                }
                ]
            }
        }
        "#;

        let multi_select =
            serde_json::from_str::<DataSourceMultiSelectProperty>(json_data).unwrap();

        assert_eq!(multi_select.id, Some("flsb".to_string()));
        assert_eq!(multi_select.name, "Store availability");

        let options = &multi_select.multi_select.options;
        assert_eq!(options.len(), 4);

        assert_eq!(
            options[0].id,
            Some("5de29601-9c24-4b04-8629-0bca891c5120".to_string())
        );
        assert_eq!(options[0].name, "Duc Loi Market");
        assert_eq!(
            options[0].color,
            Some(crate::object::select::SelectColor::Blue)
        );

        assert_eq!(
            options[1].id,
            Some("385890b8-fe15-421b-b214-b02959b0f8d9".to_string())
        );
        assert_eq!(options[1].name, "Rainbow Grocery");
        assert_eq!(
            options[1].color,
            Some(crate::object::select::SelectColor::Gray)
        );

        assert_eq!(
            options[2].id,
            Some("72ac0a6c-9e00-4e8c-80c5-720e4373e0b9".to_string())
        );
        assert_eq!(options[2].name, "Nijiya Market");
        assert_eq!(
            options[2].color,
            Some(crate::object::select::SelectColor::Purple)
        );

        assert_eq!(
            options[3].id,
            Some("9556a8f7-f4b0-4e11-b277-f0af1f8c9490".to_string())
        );
        assert_eq!(options[3].name, "Gus's Community Market");
        assert_eq!(
            options[3].color,
            Some(crate::object::select::SelectColor::Yellow)
        );
    }
}
