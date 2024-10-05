use serde::{Deserialize, Serialize};

/// **Note**: The Notion API does not support file uploads.
/// Therefore, for creating or updating, you are only allowed to
/// specify an external URL using the `ExternalFile` variant of the enum.
///
/// <https://developers.notion.com/reference/file-object>
///
/// ## File
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
/// ```json
/// {
///     "name": "Project Alpha blueprint",
///     "type": "file",
///     "file": {
///         "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///         "expiry_time": "2024-04-04T10:45:54.308Z"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum File {
    External(ExternalFile),
    Uploaded(UploadedFile),
}

impl File {
    /// Note that while `File` is an enum, this function will return the `External` variant of `File`.
    /// This is because the Notion API does not support uploading new files.
    pub fn new() -> Self {
        File::External(ExternalFile {
            r#type: "external".to_string(),
            external: ExternalFileParameter::default(),
            name: None,
            caption: None,
        })
    }

    /// This utility returns the URL regardless of whether the File variant is External or Uploaded.
    /// (You can retrieve the URL without having to check the variant).
    pub fn get_url(&self) -> String {
        match self {
            File::External(f) => f.external.url.clone(),
            File::Uploaded(f) => f.file.url.clone(),
        }
    }

    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        if let File::External(ref mut external) = self {
            external.name = Some(name.as_ref().to_string())
        }
        self
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        if let File::External(ref mut external) = self {
            external.caption = Some(caption)
        }
        self
    }

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        if let File::External(ref mut external) = self {
            external.external.url = url.as_ref().to_string()
        }
        self
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

// # --------------------------------------------------------------------------------
//
// external
//
// # --------------------------------------------------------------------------------

/// ## ExternalFile
///
/// When a link to an external file is set,
/// it becomes an object like the one shown below.
///
/// ```json
/// {
///     "type": "external",
///     "external": {
///         "url": "https://www.notion.so/images/favicon.ico"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ExternalFile {
    /// always "external"
    // An error occurs if this field is present when calling the block creation API.
    #[serde(skip_serializing)]
    pub r#type: String,

    /// file
    pub external: ExternalFileParameter,

    /// File caption (can only be set in the file type block or database properties)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// File caption (setting is available only in the file type block)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<crate::others::rich_text::RichText>>,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq, Clone)]
pub struct ExternalFileParameter {
    /// URL of the file
    pub url: String,
}

impl ExternalFile {
    pub fn new() -> Self {
        Self {
            r#type: "external".to_string(),
            external: ExternalFileParameter::default(),
            name: None,
            caption: None,
        }
    }

    pub fn url<T>(&mut self, url: T)
    where
        T: AsRef<str>,
    {
        self.external.url = url.as_ref().to_string();
    }
}

impl Default for ExternalFile {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for ExternalFile
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        let mut instance = Self::new();
        instance.url(url);
        instance
    }
}

impl std::fmt::Display for ExternalFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.external.url)
    }
}

impl std::fmt::Display for ExternalFileParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

// # --------------------------------------------------------------------------------
//
// file (uploaded via web ui)
//
// # --------------------------------------------------------------------------------

/// ## UploadedFile
///
/// **This struct is read-only.**
///
/// **Note**: The Notion API does not support file uploads.
///
/// When a file is uploaded to Notion, it becomes an object as shown below.
///
/// ```json
/// {
///     "type": "file",
///     "file": {
///         "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///         "expiry_time": "2024-04-04T10:45:54.308Z"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct UploadedFile {
    /// always "file"
    pub r#type: String,

    /// file
    pub file: UploadedFileParameter,

    /// File caption (can only be set in the file type block or database properties)
    pub name: Option<String>,

    /// File caption (setting is available only in the file type block)
    pub caption: Option<Vec<crate::others::rich_text::RichText>>,
}

impl std::fmt::Display for UploadedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file.url)
    }
}

/// file
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct UploadedFileParameter {
    /// Signed URL for the file (Amazon S3)
    pub url: String,

    /// The expiration time of the signed URL for Amazon S3
    pub expiry_time: String,
}

impl std::fmt::Display for UploadedFileParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
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
    fn deserialize_file_external() {
        let json_data = r#"
        {
            "type": "external",
            "external": {
                "url": "https://www.notion.so/images/favicon.ico"
            }
        }
        "#;

        let file = serde_json::from_str::<ExternalFile>(json_data).unwrap();

        assert_eq!(file.name, None);
        assert_eq!(file.caption, None);
        assert_eq!(file.r#type, "external");
        assert_eq!(
            file.external.url,
            "https://www.notion.so/images/favicon.ico"
        );
    }

    #[test]
    fn deserialize_file_file() {
        let json_data = r#"
        {
            "type": "file",
            "file": {
                "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
                "expiry_time": "2024-04-04T10:45:54.308Z"
            }
        }
        "#;

        let file = serde_json::from_str::<UploadedFile>(json_data).unwrap();

        assert_eq!(file.name, None);
        assert_eq!(file.caption, None);
        assert_eq!(file.r#type, "file");
        assert_eq!(
            file.file.url,
            "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d"
        );
        assert_eq!(file.file.expiry_time, "2024-04-04T10:45:54.308Z");
    }
}
