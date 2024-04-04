use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#relation
///
/// Example relation page property value
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
    pub id: String,
    pub relation: Vec<PageRelationPropertyParameter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationPropertyParameter {
    pub id: String,
}
