use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#title>
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
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    pub id: String,

    // TODO: documentation
    pub title: Vec<crate::others::rich_text::RichText>,
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
