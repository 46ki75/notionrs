use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FileBlock {
    pub file: crate::others::file::File,
}

impl FileBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::File(self)
    }

    pub fn new<T>(url: T) -> Self
    where
        T: AsRef<str>,
    {
        FileBlock {
            file: crate::others::file::File::new(url),
        }
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
