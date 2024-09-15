use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct FileBlock {
    /// When creating an AudioBlock via the API, only files of the External type are accepted.
    /// (File uploads are not supported.)
    /// [Documentation](https://developers.notion.com/reference/file-object)
    pub file: crate::others::file::File,
}

impl FileBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::File(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Set the external URL for the file.
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.file {
            external.external.url = url.as_ref().to_string();
        }
        self
    }

    /// Add a caption to the file.
    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.file {
            external.caption = Some(caption);
        }
        self
    }

    /// Assign a file name
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.file {
            external.name = Some(name.as_ref().to_string());
        }
        self
    }
}

impl<T> From<T> for FileBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self {
            file: crate::others::file::File::External(crate::others::file::ExternalFile::from(url)),
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

    use core::panic;

    use super::*;

    #[test]
    fn deserialize_block_file_() {
        let json_data = r#"
        {
            "file": {
                "caption": [],
                "type": "file",
                "file": {
                    "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                    "expiry_time": "2024-08-20T10:54:16.569Z"
                },
                "name": "sample.mp3"
            }
        }
        "#;

        let file = serde_json::from_str::<FileBlock>(json_data).unwrap();

        match file.file {
            crate::others::file::File::File(f) => {
                assert_eq!(f.caption, Some(vec![]));
                assert_eq!(f.r#type, "file");
                assert_eq!(f.name, Some("sample.mp3".to_string()));
                assert_eq!(
                    f.file.url,
                    "https://prod-files-secure.s3.us-west-2.amazonaws.com/"
                );
                assert_eq!(f.file.expiry_time, "2024-08-20T10:54:16.569Z");
            }
            crate::others::file::File::External(_) => panic!(),
        }
    }
}
