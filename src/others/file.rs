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
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum File {
    External(FileExternal),
    File(FileFile),
}

impl File {
    pub fn new() -> Self {
        File::External(FileExternal {
            r#type: "external".to_string(),
            external: FileExternalParameter::default(),
            name: None,
            caption: None,
        })
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
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

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
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileExternal {
    /// always "external"
    // An error occurs if this field is present when calling the block creation API.
    #[serde(skip_serializing)]
    pub r#type: String,

    /// file
    pub external: FileExternalParameter,

    /// File caption (can only be set in the file type block or database properties)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// File caption (setting is available only in the file type block)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<crate::others::rich_text::RichText>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub struct FileExternalParameter {
    /// URL of the file
    pub url: String,
}

/// When a file is uploaded to Notion, it becomes an object as shown below.
///
/// json```
/// {
///     "type": "file",
///     "file": {
///         "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///         "expiry_time": "2024-04-04T10:45:54.308Z"
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileFile {
    /// always "file"
    pub r#type: String,

    /// file
    pub file: FileFileParameter,

    /// File caption (can only be set in the file type block or database properties)
    pub name: Option<String>,

    /// File caption (setting is available only in the file type block)
    pub caption: Option<Vec<crate::others::rich_text::RichText>>,
}

/// file
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct FileFileParameter {
    /// Signed URL for the file (Amazon S3)
    pub url: String,

    /// The expiration time of the signed URL for Amazon S3
    pub expiry_time: String,
}

impl FileExternal {
    pub fn new() -> Self {
        Self {
            r#type: "external".to_string(),
            external: FileExternalParameter { url: String::new() },
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

impl Default for FileExternal {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<T> for FileExternal
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        let mut instance = Self::new();
        instance.url(url);
        instance
    }
}

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

        let file = serde_json::from_str::<FileExternal>(json_data).unwrap();

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

        let file = serde_json::from_str::<FileFile>(json_data).unwrap();

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
