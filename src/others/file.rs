use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/file-object>
///
/// File objects contain data about a file that is uploaded to Notion,
/// or data about an external file that is linked to in Notion.
///
/// When a link to an external file is set,
/// it becomes an object like the one shown below.
///
/// ```json
/// {
///     "name": "https://www.notion.so/images/favicon.ico",
///     "type": "external",
///     "external": {
///         "url": "https://www.notion.so/images/favicon.ico"
///     }
/// }
/// ```
///
/// When a file is uploaded to Notion, it becomes an object as shown below.
///
/// json```
/// {
///     "name": "Project Alpha blueprint",
///     "type": "file",
///     "file": {
///         "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///         "expiry_time": "2024-04-04T10:45:54.308Z"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum File {
    External(FileExternal),
    File(FileFile),
}

/// When a link to an external file is set,
/// it becomes an object like the one shown below.
///
/// ```json
/// {
///     "name": "https://www.notion.so/images/favicon.ico",
///     "type": "external",
///     "external": {
///         "url": "https://www.notion.so/images/favicon.ico"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileExternal {
    /// always "external"
    pub r#type: String,

    /// file
    pub external: FileExternalParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileExternalParameter {
    /// URL of the file
    pub url: String,
}

/// When a file is uploaded to Notion, it becomes an object as shown below.
///
/// json```
/// {
///     "name": "Favicon",
///     "type": "file",
///     "file": {
///         "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///         "expiry_time": "2024-04-04T10:45:54.308Z"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct FileFile {
    /// always "file"
    pub r#type: String,

    /// file
    pub file: FileFileParameter,

    /// File name
    pub name: String,
}

/// file
#[derive(Debug, Deserialize, Serialize)]
pub struct FileFileParameter {
    /// Signed URL for the file (Amazon S3)
    pub url: String,

    /// The expiration time of the signed URL for Amazon S3
    pub expiry_time: String,
}
