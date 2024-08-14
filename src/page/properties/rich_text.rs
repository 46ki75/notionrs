use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#rich-text>
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
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub rich_text: Vec<crate::others::rich_text::RichText>,
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
