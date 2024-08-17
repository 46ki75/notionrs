use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AudioBlockRequest {
    pub r#type: String,

    pub audio: crate::others::file::File,
}

impl AudioBlockRequest {
    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        AudioBlockRequest {
            r#type: "audio".to_string(),
            audio: crate::others::file::File::External(crate::others::file::FileExternal {
                caption: Some(vec![]),
                r#type: "external".to_string(),
                name: None,
                external: crate::others::file::FileExternalParameter {
                    url: url.as_ref().to_string(),
                },
            }),
        }
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(audio) = &mut self.audio {
            audio.caption.get_or_insert(vec![]).extend(caption);
        }
        self
    }
}
