use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#bookmark>
///
/// Bookmark block objects contain the following
/// information within the bookmark property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct BookmarkBlock {
    /// The caption for the bookmark.
    pub caption: Vec<crate::object::rich_text::RichText>,

    /// The link for the bookmark.
    pub url: String,
}

crate::impl_from_as_ref!(BookmarkBlock, url);
crate::impl_display_from_string_field!(BookmarkBlock, url);

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::BookmarkBlock;

    #[test]
    fn deserialize_block_bookmark() {
        let json_data = r#"
        {
            "caption": [
                {
                    "type": "text",
                    "text": {
                        "content": "example domain",
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
                    "plain_text": "example domain",
                    "href": null
                }
            ],
            "url": "https://example.com"
        }
        "#;

        let bookmark = serde_json::from_str::<BookmarkBlock>(json_data).unwrap();

        assert_eq!(bookmark.url, "https://example.com");

        let rich_text = bookmark.caption.first().unwrap();

        match rich_text {
            crate::object::rich_text::RichText::Text {
                annotations,
                plain_text,
                href,
                ..
            } => {
                assert_eq!(plain_text, "example domain");
                assert_eq!(*href, None);

                assert!(!annotations.bold);
                assert!(!annotations.code);
                assert!(!annotations.strikethrough);
                assert!(!annotations.underline);
                assert!(!annotations.italic);
                assert_eq!(annotations.color, crate::object::color::Color::Default)
            }
            _ => panic!(),
        }
    }
}
