use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#formula>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"__________"` // TODO: documentation replace placeholder
/// - `$.['*'].__________`: // TODO: documentation
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example __________ page property value // TODO: documentation replace placeholder
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
    pub id: String,

    // TODO: documentation
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FormulaBoolean {
    pub boolean: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaDate {
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FormulaNumber {
    pub number: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaString {
    pub string: String,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // TODO: test
}
