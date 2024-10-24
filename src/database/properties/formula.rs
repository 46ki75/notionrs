use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseFormulaProperty {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub formula: DatabaseFormulaExpressionProperty,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseFormulaExpressionProperty {
    expression: String,
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
    fn deserialize_database_formula_property() {
        let json_data = r#"
        {
            "id": "YU%7C%40",
            "name": "Updated price",
            "type": "formula",
            "formula": {
                "expression": "{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"
            }
        }
        "#;

        let formula = serde_json::from_str::<DatabaseFormulaProperty>(json_data).unwrap();

        assert_eq!(formula.id, Some("YU%7C%40".to_string()));
        assert_eq!(formula.name, "Updated price");
        assert_eq!(formula.formula.expression, "{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2");
    }
}
