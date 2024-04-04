use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#number
///
/// Example number page property value
///
/// ```json
/// { "Number": { "type": "number", "id": "%5Chme", "number": 20.0 } }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageNumberProperty {
    pub id: String,
    pub number: Option<f64>,
}
