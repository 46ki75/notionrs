use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseSelectProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub select: DatabaseSelectOptionProperty,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseSelectOptionProperty {
    options: Vec<crate::others::select::Select>,
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

        let select = serde_json::from_str::<DatabaseSelectProperty>(json_data).unwrap();

        assert_eq!(select.id, "%40Q%5BM");
        assert_eq!(select.name, "Food group");

        let options = &select.select.options;
        assert_eq!(options.len(), 3);

        assert_eq!(options[0].id, "e28f74fc-83a7-4469-8435-27eb18f9f9de");
        assert_eq!(options[0].name, "🥦Vegetable");
        assert_eq!(options[0].color, crate::others::color::ColorFG::Purple);

        assert_eq!(options[1].id, "6132d771-b283-4cd9-ba44-b1ed30477c7f");
        assert_eq!(options[1].name, "🍎Fruit");
        assert_eq!(options[1].color, crate::others::color::ColorFG::Red);

        assert_eq!(options[2].id, "fc9ea861-820b-4f2b-bc32-44ed9eca873c");
        assert_eq!(options[2].name, "💪Protein");
        assert_eq!(options[2].color, crate::others::color::ColorFG::Yellow);
    }
}
