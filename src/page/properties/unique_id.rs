use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#unique_id
///
/// Example unique_id page property value
///
/// ```json
/// {
///   "ID": {
///     "id": "mBKy",
///     "type": "unique_id",
///     "unique_id": {
///       "prefix": "TES",
///       "number": 434
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdProperty {
    pub id: String,
    pub unique_id: PageUniqueIdPropertyParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdPropertyParameter {
    pub prefix: Option<String>,
    pub number: u64,
}
