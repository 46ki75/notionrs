use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#formula>
///
/// Formula property value objects represent the result of evaluating
/// a formula described in the database's properties.
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"formula"`
/// - `$.['*'].formula.type`: A string indicating the data type of the result of the formula.
///                         Possible type values are:
///                         - `boolean`
///                         - `date`
///                         - `number`
///                         - `string`
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example formula page property value
///
/// ```json
/// {
///   "Formula": {
///     "type": "formula",
///     "id": "W~%5BW",
///     "formula": { "type": "string", "string": "My Title" }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageFormulaProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: Option<String>,

    /// Formula property value objects represent the result of evaluating
    /// a formula described in the database's properties.
    pub formula: Formula,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Formula {
    Boolean(FormulaBoolean),
    Date(FormulaDate),
    Number(FormulaNumber),
    String(FormulaString),
}

/// ```json
/// {
///   "type": "boolean",
///   "boolean": false
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FormulaBoolean {
    /// Calculated value of the database property
    pub boolean: Option<bool>,
}

/// ```json
/// {
///   "type": "date",
///   "date": "2024-08-15T05:24:00.000Z"
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FormulaDate {
    /// Calculated value of the database property
    pub date: Option<chrono::DateTime<chrono::Utc>>,
}

/// ```json
/// {
///   "type": "number",
///   "number": 56
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FormulaNumber {
    /// Calculated value of the database property
    pub number: Option<f64>,
}

/// ```json
/// {
///   "type": "string",
///   "string": "My Title"
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormulaString {
    /// Calculated value of the database property
    pub string: Option<String>,
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
    pub fn deserialize_page_forlula_property() {
        let json_data = r#"
        {
            "Formula": {
                "type": "formula",
                "id": "W~%5BW",
                "formula": {
                    "type": "string",
                    "string": "My Title"
                }
            }
        }
        "#;

        let formula_map = serde_json::from_str::<
            std::collections::HashMap<String, PageFormulaProperty>,
        >(json_data)
        .unwrap();

        let formula = formula_map.get("Formula").unwrap();

        assert_eq!(formula.id, Some("W~%5BW".to_string()));

        match &formula.formula {
            Formula::String(s) => assert_eq!(s.string, Some("My Title".to_string())),
            Formula::Boolean(_) => panic!(),
            Formula::Date(_) => panic!(),
            Formula::Number(_) => panic!(),
        }
    }
}
