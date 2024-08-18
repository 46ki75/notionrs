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
    pub fn build(self) -> super::BlockRequest {
        super::BlockRequest::Bookmark(self)
    }

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

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.bookmark.url = url.as_ref().to_string();
        self
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.bookmark.caption = caption;
        self
    }
}
