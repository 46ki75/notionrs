use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq, notionrs_macro::Setter)]
pub struct DataSourceFormulaProperty {
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

    pub formula: DataSourceFormulaExpressionProperty,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DataSourceFormulaExpressionProperty {
    expression: String,
}

impl<T> From<T> for DataSourceFormulaProperty
where
    T: AsRef<str>,
{
    fn from(expression: T) -> Self {
        Self {
            formula: DataSourceFormulaExpressionProperty {
                expression: expression.as_ref().to_string(),
            },
            ..Default::default()
        }
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

        let formula = serde_json::from_str::<DataSourceFormulaProperty>(json_data).unwrap();

        assert_eq!(formula.id, Some("YU%7C%40".to_string()));
        assert_eq!(formula.name, "Updated price");
        assert_eq!(
            formula.formula.expression,
            "{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"
        );
    }
}
