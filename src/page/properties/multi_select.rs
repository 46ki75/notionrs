use serde::{Deserialize, Serialize};

use crate::others::select::Select;

/// https://developers.notion.com/reference/page-property-values#multi-select
///
/// Example multi_select page property value
///
/// ```json
/// {
///   "Multi-select": {
///     "id": "_bnY",
///     "type": "multi_select",
///     "multi_select": [
///       {
///         "id": "959ba3e3-5a64-4ee6-864b-9e94ddc024d5",
///         "name": "HTML",
///         "color": "orange"
///       },
///       {
///         "id": "f22b05c9-0225-4dee-b25b-db7e63a47e0b",
///         "name": "CSS",
///         "color": "blue"
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageMultiSelectProperty {
    pub id: String,
    pub multi_select: Vec<Select>,
}
