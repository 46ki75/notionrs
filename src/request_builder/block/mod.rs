use serde::{Deserialize, Serialize};

pub mod audio;
pub mod bookmark;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BlockRequest {
    Audio(audio::AudioBlockRequest),
    Bookmark(bookmark::BookmarkBlockRequest),
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
