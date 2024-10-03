use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ImageBlock {
    /// When creating an AudioBlock via the API, only files of the External type are accepted.
    /// (File uploads are not supported.)
    /// [Documentation](https://developers.notion.com/reference/file-object)
    pub image: crate::others::file::File,
}

impl ImageBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Image(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Set the external URL for the file.
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.image {
            external.external.url = url.as_ref().to_string();
        }
        self
    }

    /// Add a caption to the file.
    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.image {
            external.caption = Some(caption);
        }
        self
    }

    /// Assign a file name
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.image {
            external.name = Some(name.as_ref().to_string());
        }
        self
    }
}

impl<T> From<T> for ImageBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self {
            image: crate::others::file::File::External(crate::others::file::ExternalFile::from(
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

    use core::panic;

    use super::*;

    #[test]
    fn deserialize_block_image() {
        let json_data = r#"
        {
            "image": {
                "caption": [],
                "type": "file",
                "file": {
                    "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                    "expiry_time": "2024-08-20T11:07:14.256Z"
                }
            }
        }
        "#;

        let image_block = serde_json::from_str::<ImageBlock>(json_data).unwrap();

        match image_block.image {
            crate::others::file::File::Uploaded(f) => {
                assert_eq!(f.caption, Some(vec![]));
                assert_eq!(f.r#type, "file");
                assert_eq!(
                    f.file.url,
                    "https://prod-files-secure.s3.us-west-2.amazonaws.com/"
                );
                assert_eq!(f.file.expiry_time, "2024-08-20T11:07:14.256Z");
            }
            crate::others::file::File::External(_) => panic!(),
        }
    }
}
