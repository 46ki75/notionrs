use serde::{Deserialize, Serialize};

use crate::others::select::Select;

/// https://developers.notion.com/reference/page-property-values#status
///
/// Example status page property value
///
/// ```json
/// {
///   "Status": {
///     "type": "status",
///     "id": "xx%7Cd",
///     "status": {
///       "id": "4a1accbf-6716-4cf2-9034-5877581fc5f6",
///       "name": "Not started",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageStatusProperty {
    pub id: String,
    pub status: Select,
}
