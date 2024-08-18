use serde::{Deserialize, Serialize};

pub mod audio;
pub mod bookmark;
pub mod bulleted_list_item;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockRequest {
    Audio(audio::AudioBlockRequest),
    Bookmark(bookmark::BookmarkBlockRequest),
    BulletedListItem(bulleted_list_item::BulletedListItemBlockRequest),
}

impl BlockRequest {
    pub fn audio<T>(url: T) -> audio::AudioBlockRequest
    where
        T: AsRef<str>,
    {
        audio::AudioBlockRequest::new(url)
    }

    pub fn bookmark<T>(url: T) -> bookmark::BookmarkBlockRequest
    where
        T: AsRef<str>,
    {
        bookmark::BookmarkBlockRequest::new(url)
    }

    pub fn bulleted_list_item() -> bulleted_list_item::BulletedListItemBlockRequest {
        bulleted_list_item::BulletedListItemBlockRequest::new()
    }
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn serialize_bookmark() {
        let bookmark = BlockRequest::bookmark("url");
        let request_body = serde_json::to_string(&bookmark).unwrap();
        println!("{}", request_body);
    }
}
