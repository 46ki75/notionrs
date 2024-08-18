use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioBlockRequest {
    r#type: String,
    audio: crate::others::file::File,
}

/// Uploading files via the API is not supported; only specifying an external URL is supported.
impl AudioBlockRequest {
    pub fn build(self) -> super::BlockRequest {
        super::BlockRequest::Audio(self)
    }

    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        AudioBlockRequest {
            r#type: "audio".to_string(),
            audio: crate::others::file::File::new(url),
        }
    }

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.external.url = url.as_ref().to_string();
        }
        self
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.caption = Some(caption)
        }
        self
    }
}
