use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BookmarkBlockRequest {
    r#type: String,
    bookmark: BookmarkBlockRequestParams,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookmarkBlockRequestParams {
    url: String,
    caption: Vec<crate::others::rich_text::RichText>,
}

impl BookmarkBlockRequest {
    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        BookmarkBlockRequest {
            r#type: "bookmark".to_string(),
            bookmark: BookmarkBlockRequestParams {
                url: url.as_ref().to_string(),
                caption: vec![],
            },
        }
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.bookmark.caption = caption;
        self
    }
}
