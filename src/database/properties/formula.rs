use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseFormulaProperty {
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

    pub formula: DatabaseFormulaExpressionProperty,
}

impl DatabaseFormulaProperty {
    /// Modify the value of this field when updating the column name of the property.
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        self.name = name.as_ref().to_string();
        self
    }

    /// Although it is not explicitly stated in the official documentation,
    /// you can add a description to the property by specifying this.
    pub fn description<T>(mut self, description: T) -> Self
    where
        T: AsRef<str>,
    {
        self.description = Some(description.as_ref().to_string());
        self
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseFormulaExpressionProperty {
    expression: String,
}

impl<T> From<T> for DatabaseFormulaProperty
where
    T: AsRef<str>,
{
    fn from(expression: T) -> Self {
        Self {
            formula: DatabaseFormulaExpressionProperty {
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

        let formula = serde_json::from_str::<DatabaseFormulaProperty>(json_data).unwrap();

        assert_eq!(formula.id, Some("YU%7C%40".to_string()));
        assert_eq!(formula.name, "Updated price");
        assert_eq!(formula.formula.expression, "{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2");
    }
}
