use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#formula>
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
    pub id: String,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaBoolean {
    pub boolean: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaDate {
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaNumber {
    pub number: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaString {
    pub string: String,
}
