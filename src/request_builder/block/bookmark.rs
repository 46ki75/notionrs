use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BookmarkBlockRequest {
    pub r#type: String,

    pub bookmark: crate::block::bookmark::BookmarkBlock,
}

impl BookmarkBlockRequest {
    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        BookmarkBlockRequest {
            r#type: "bookmark".to_string(),
            bookmark: crate::block::bookmark::BookmarkBlock {
                caption: vec![],
                url: url.as_ref().to_string(),
            },
        }
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.bookmark.caption = caption;
        self
    }
}
