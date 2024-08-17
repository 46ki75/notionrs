use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AudioBlockRequest {
    r#type: String,
    audio: crate::others::file::File,
}

impl AudioBlockRequest {
    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        AudioBlockRequest {
            r#type: "audio".to_string(),
            audio: crate::others::file::File::new(url),
        }
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.caption = Some(caption)
        }
        self
    }
}
