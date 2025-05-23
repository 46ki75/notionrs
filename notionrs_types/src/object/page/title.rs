use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#title>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
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
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, notionrs_macro::Setter)]
pub struct PageTitleProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of [rich text](https://developers.notion.com/reference/rich-text) objects.
    pub title: Vec<crate::object::rich_text::RichText>,
}

impl<T> From<T> for PageTitleProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self {
            id: None,
            title: vec![crate::object::rich_text::RichText::from(value)],
        }
    }
}

impl From<crate::object::rich_text::RichText> for PageTitleProperty {
    fn from(rich_text: crate::object::rich_text::RichText) -> Self {
        Self {
            id: None,
            title: vec![rich_text],
        }
    }
}

impl std::fmt::Display for PageTitleProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plain_text = self
            .title
            .iter()
            .map(|rt| rt.to_string())
            .collect::<String>();
        write!(f, "{}", plain_text)
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

        // if let crate::TextType::Text(text) = &title.title.first().unwrap().text {
        //     assert_eq!(text.content, "My Title");
        //     assert_eq!(text.link, None);
        // }

        // assert!(!title.title.first().unwrap().annotations.bold);
        // assert!(!title.title.first().unwrap().annotations.italic);
        // assert!(!title.title.first().unwrap().annotations.strikethrough);
        // assert!(!title.title.first().unwrap().annotations.underline);
        // assert!(!title.title.first().unwrap().annotations.code);
        // assert_eq!(
        //     title.title.first().unwrap().annotations.color,
        //     crate::object::color::Color::Default
        // );

        // assert_eq!(title.title.first().unwrap().plain_text, "My Title");
        // assert_eq!(title.title.first().unwrap().href, None);
    }
}
