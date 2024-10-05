use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#bookmark>
///
/// Bookmark block objects contain the following
/// information within the bookmark property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BookmarkBlock {
    /// The caption for the bookmark.
    pub caption: Vec<crate::others::rich_text::RichText>,

    /// The link for the bookmark.
    pub url: String,
}

impl BookmarkBlock {
    pub fn build(self) -> super::Block {
        super::Block::Bookmark { bookmark: self }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.url = url.as_ref().to_string();
        self
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.caption = caption;
        self
    }
}

impl<T> From<T> for BookmarkBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self::default().url(url)
    }
}

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

        assert_eq!(rich_text.plain_text, "example domain");
        assert_eq!(rich_text.href, None);

        assert!(!rich_text.annotations.bold);
        assert!(!rich_text.annotations.italic);
        assert!(!rich_text.annotations.strikethrough);
        assert!(!rich_text.annotations.underline);
        assert!(!rich_text.annotations.code);
        assert_eq!(
            rich_text.annotations.color,
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default)
        );
    }
}
