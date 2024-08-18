use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct VideoBlock {
    /// When creating an AudioBlock via the API, only files of the External type are accepted.
    /// (File uploads are not supported.)
    /// [Documentation](https://developers.notion.com/reference/file-object)
    pub video: crate::others::file::File,
}

impl VideoBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Video(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Set the external URL for the file.
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.video {
            external.external.url = url.as_ref().to_string();
        }
        self
    }

    /// Add a caption to the file.
    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let crate::others::file::File::External(ref mut external) = self.video {
            external.caption = Some(caption);
        }
        self
    }

    /// Assign a file name
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        if let crate::others::file::File::External(ref mut external) = self.video {
            external.name = Some(name.as_ref().to_string());
        }
        self
    }
}

impl<T> From<T> for VideoBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self {
            video: crate::others::file::File::External(crate::others::file::FileExternal::from(
                url,
            )),
        }
    }
}
