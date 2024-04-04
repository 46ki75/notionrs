use serde::{Deserialize, Serialize};

use crate::others::rich_text::RichText;

/// https://developers.notion.com/reference/page-property-values#title
///
/// Example title page property value
///
/// ```json
/// {
///   "title": {
///     "id": "title",
///     "type": "title",
///     "title": [
///       {
///         "type": "text",
///         "text": {
///           "content": "My Title",
///           "link": null
///         },
///         "annotations": {
///           "bold": false,
///           "italic": false,
///           "strikethrough": false,
///           "underline": false,
///           "code": false,
///           "color": "default"
///         },
///         "plain_text": "My Title",
///         "href": null
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageTitleProperty {
    pub id: String,
    pub title: Vec<RichText>,
}
