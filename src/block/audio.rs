use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AudioBlock {
    /// When creating an AudioBlock via the API, only files of the External type are accepted.
    /// (File uploads are not supported.)
    /// [Documentation](https://developers.notion.com/reference/file-object)
    pub audio: crate::others::file::File,
}

impl AudioBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Audio(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Set the external URL for the file.
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.external.url = url.as_ref().to_string();
        }
        self
    }

    /// Add a caption to the file.
    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.caption = Some(caption);
        }
        self
    }

    /// Assign a file name
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.audio {
            external.name = Some(name.as_ref().to_string());
        }
        self
    }
}

impl<T> From<T> for AudioBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self {
            audio: crate::others::file::File::External(crate::others::file::FileExternal::from(
                url,
            )),
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_block_audio() {
        let json_data = r#"
        {
            "audio": {
                "caption": [],
                "type": "file",
                "file": {
                    "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                    "expiry_time": "2024-08-18T22:12:45.122Z"
                }
            }
        }
        "#;

        let audio_block = serde_json::from_str::<AudioBlock>(json_data).unwrap();

        match audio_block.audio {
            crate::others::file::File::File(file) => {
                assert_eq!(
                    file.file.url,
                    "https://prod-files-secure.s3.us-west-2.amazonaws.com/"
                );
            }
            crate::others::file::File::External(_) => panic!(),
        }
    }

    #[test]
    fn serialize_block_audio() {
        let audio_block = AudioBlock::new().url("https://example.com/audio.wav");

        let audio_block_json = serde_json::to_string(&audio_block).unwrap();

        let audio_block = serde_json::from_str::<AudioBlock>(&audio_block_json).unwrap();

        match audio_block.audio {
            crate::others::file::File::External(file) => {
                assert_eq!(file.external.url, "https://example.com/audio.wav");
            }
            crate::others::file::File::File(_) => panic!(),
        }
    }
}
