use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/page-property-values#url
///
/// Example url page property value
///
/// ```json
/// {
///   "URL": {
///     "type": "url",
///     "id": "h_AH",
///     "url": "https://developers.notion.com/reference/page-property-values#url"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUrlProperty {
    pub id: String,
    pub url: Option<String>,
}
