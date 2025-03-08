use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#rich-text>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///                 `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"rich_text"`
/// - `$.['*'].rich_text`: An array of [rich text objects](https://developers.notion.com/reference/rich-text)
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example rich_text page property value
///
/// ```json
/// {
///     "Text": {
///         "id": "mM%3BV",
///         "type": "rich_text",
///         "rich_text": [
///             {
///                 "type": "text",
///                 "text": {
///                     "content": "My Description",
///                     "link": null
///                 },
///                 "annotations": {
///                     "bold": false,
///                     "italic": false,
///                     "strikethrough": false,
///                     "underline": false,
///                     "code": false,
///                     "color": "default"
///                 },
///                 "plain_text": "My Description",
///                 "href": null
///             }
///         ]
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageRichTextProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of [rich text objects](https://developers.notion.com/reference/rich-text)
    pub rich_text: Vec<crate::others::rich_text::RichText>,
}

impl<T> From<T> for PageRichTextProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self {
            id: None,
            rich_text: vec![crate::RichText::from(value)],
        }
    }
}

impl From<crate::RichText> for PageRichTextProperty {
    fn from(rich_text: crate::RichText) -> Self {
        Self {
            id: None,
            rich_text: vec![rich_text],
        }
    }
}

impl std::fmt::Display for PageRichTextProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let plain_text = self
            .rich_text
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
    fn deserialize_rich_text_property() {
        let json_data = r#"
        {
            "Text": {
                "id": "mM%3BV",
                "type": "rich_text",
                "rich_text": [
                    {
                        "type": "text",
                        "text": {
                            "content": "My Description",
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
                        "plain_text": "My Description",
                        "href": null
                    }
                ]
            }
        }
        "#;

        let rich_text_map = serde_json::from_str::<
            std::collections::HashMap<String, PageRichTextProperty>,
        >(json_data)
        .unwrap();

        let rich_text = rich_text_map.get("Text").unwrap();

        assert_eq!(rich_text.id, Some("mM%3BV".to_string()));

        // if let TextType::Text(text) = &rich_text.rich_text.first().unwrap().text {
        //     assert_eq!(text.content, "My Description");
        //     assert_eq!(text.link, None);
        // }

        // assert!(!rich_text.rich_text.first().unwrap().annotations.bold);
        // assert!(!rich_text.rich_text.first().unwrap().annotations.italic);
        // assert!(
        //     !rich_text
        //         .rich_text
        //         .first()
        //         .unwrap()
        //         .annotations
        //         .strikethrough
        // );
        // assert!(!rich_text.rich_text.first().unwrap().annotations.underline);
        // assert!(!rich_text.rich_text.first().unwrap().annotations.code);
        // assert_eq!(
        //     rich_text.rich_text.first().unwrap().annotations.color,
        //     crate::others::color::Color::Default
        // );

        // assert_eq!(
        //     rich_text.rich_text.first().unwrap().plain_text,
        //     "My Description"
        // );
        // assert_eq!(rich_text.rich_text.first().unwrap().href, None);
    }
}
