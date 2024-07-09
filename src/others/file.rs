use serde::{Deserialize, Serialize};

/// https://developers.notion.com/reference/file-object
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum File {
    External(FileExternal),
    File(FileFile),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileExternal {
    /// always "external"
    pub r#type: String,
    pub external: FileExternalParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileExternalParameter {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileFile {
    /// always "file"
    pub r#type: String,
    pub file: FileFileParameter,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileFileParameter {
    pub url: String,
    pub expiry_time: String,
}
