use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#title>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"title"`
/// - `$.['*'].title`: An array of [rich text](https://developers.notion.com/reference/rich-text) objects.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example title page property value
///
/// ```json
/// {
///   "Title": {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of [rich text](https://developers.notion.com/reference/rich-text) objects.
    pub title: Vec<crate::others::rich_text::RichText>,
}

impl From<&str> for PageTitleProperty {
    fn from(value: &str) -> Self {
        Self {
            id: None,
            title: vec![crate::RichText::from(value)],
        }
    }
}

impl From<String> for PageTitleProperty {
    fn from(value: String) -> Self {
        Self {
            id: None,
            title: vec![crate::RichText::from(value)],
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_title_property() {
        let json_data = r#"
        {
            "Title": {
                "id": "frg3",
                "type": "title",
                "title": [
                    {
                        "type": "text",
                        "text": {
                            "content": "My Title",
                            "link": null
                        },
                        "annotations": {
                            "bold": false,
                            "italic": false,
                            "strikethrough": false,
                            "underline": false,
                            "code": false,
                            "color": "default"
                        },
                        "plain_text": "My Title",
                        "href": null
                    }
                ]
            }
        }
        "#;

        let title_map =
            serde_json::from_str::<std::collections::HashMap<String, PageTitleProperty>>(json_data)
                .unwrap();

        let title = title_map.get("Title").unwrap();

        assert_eq!(title.id, Some("frg3".to_string()));

        assert_eq!(title.title.first().unwrap().text.content, "My Title");
        assert_eq!(title.title.first().unwrap().text.link, None);

        assert!(!title.title.first().unwrap().annotations.bold);
        assert!(!title.title.first().unwrap().annotations.italic);
        assert!(!title.title.first().unwrap().annotations.strikethrough);
        assert!(!title.title.first().unwrap().annotations.underline);
        assert!(!title.title.first().unwrap().annotations.code);
        assert_eq!(
            title.title.first().unwrap().annotations.color,
            crate::others::color::Color::Default
        );

        assert_eq!(title.title.first().unwrap().plain_text, "My Title");
        assert_eq!(title.title.first().unwrap().href, None);
    }
}
