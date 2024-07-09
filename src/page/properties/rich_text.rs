use serde::{Deserialize, Serialize};

use crate::others::rich_text::RichText;

/// <https://developers.notion.com/reference/page-property-values#rich-text>
///
/// Example rich_text page property value
///
/// ```json
/// {
///   "Text": {
///     "id": "mM%3BV",
///     "type": "rich_text",
///     "rich_text": [
///       {
///         "type": "text",
///         "text": {
///           "content": "My Description",
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
///         "plain_text": "My Description",
///         "href": null
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRichTextProperty {
    pub id: String,
    pub rich_text: Vec<RichText>,
}
