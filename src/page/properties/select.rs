use serde::{Deserialize, Serialize};

use crate::others::select::Select;

/// https://developers.notion.com/reference/page-property-values#select
///
/// Example select page property value
///
/// ```json
/// {
///   "Select": {
///     "type": "select",
///     "id": "chOy",
///     "select": {
///       "id": "eede87ce-52db-4b16-9931-2bc40687d697",
///       "name": "TODO",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageSelectProperty {
    pub id: String,
    pub select: Option<Select>,
}
