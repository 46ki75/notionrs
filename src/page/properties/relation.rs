use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#relation>
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
///   "Related": {
///     "id": "b%7D%3Ek",
///     "type": "relation",
///     "relation": [
///       {
///         "id": "669ffc58-9c20-4264-956b-f7f917c58400"
///       }
///     ],
///     "has_more": false
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub relation: Vec<PageRelationPropertyParameter>,

    /// If a relation has more than 25 references,
    /// then the has_more value for the relation in the response object is true.
    /// If a relation doesn’t exceed the limit, then has_more is false.
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationPropertyParameter {
    pub id: String,
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
