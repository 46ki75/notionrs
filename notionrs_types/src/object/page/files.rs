use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/page-property-values#files>
///
/// - `$.['*'].id`: An underlying identifier for the property.
///   `id` remains constant when the property name changes.
/// - `$.['*'].type`: Always `"files"`
/// - `$.['*'].files`: An array of objects containing information about
///   the [files](https://developers.notion.com/reference/file-object).
///   If the file does not exist, an empty array will be returned.
///
/// **Note**: The `['*']` part represents the column name you set when creating the database.
///
/// Example files page property value
///
/// ```json
/// {
///   "Files & media": {
///     "id": "Q%7Dn%3E",
///     "type": "files",
///     "files": [
///       {
///         "name": "Project Alpha blueprint",
///         "type": "file",
///         "file": {
///           "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///           "expiry_time": "2024-04-04T10:45:54.308Z"
///         }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct PageFilesProperty {
    /// An underlying identifier for the property.
    /// `id` remains constant when the property name changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// An array of objects containing information
    /// about the [files](https://developers.notion.com/reference/file-object).
    ///
    /// When creating, both the external path of the file and `name` are required.
    pub files: Vec<crate::object::file::File>,
}

impl<T> From<T> for PageFilesProperty
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let file = crate::object::file::File::default()
            .url(value.as_ref())
            .name(value.as_ref());
        Self::default().files(vec![file])
    }
}

impl From<crate::object::file::File> for PageFilesProperty {
    fn from(value: crate::object::file::File) -> Self {
        Self::default().files(vec![value])
    }
}

impl std::fmt::Display for PageFilesProperty {
    /// display the files' names in a comma-separated list
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.files
                .iter()
                .map(|file| file.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
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
    fn deserialize_page_files_property() {
        let json_data = r#"
        {
            "File": {
                "id": "%3AlnV",
                "type": "files",
                "files": [
                    {
                        "name": "0208a.jpg",
                        "type": "file",
                        "file": {
                            "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com",
                            "expiry_time": "2024-08-15T05:56:14.346Z"
                        }
                    },
                    {
                        "name": "Favicon.ico",
                        "type": "external",
                        "external": {
                            "url": "https://www.notion.so/images/favicon.ico"
                        }
                    }
                ]
            }
        }
        "#;

        let file_map =
            serde_json::from_str::<std::collections::HashMap<String, PageFilesProperty>>(json_data)
                .unwrap();

        let file = file_map.get("File").unwrap();

        assert_eq!(file.id, Some("%3AlnV".to_string()));

        for file in &file.files {
            match &file {
                crate::object::file::File::NotionHosted(f) => {
                    assert_eq!(f.name, Some("0208a.jpg".to_string()));
                    assert_eq!(
                        f.file.url,
                        "https://prod-files-secure.s3.us-west-2.amazonaws.com"
                    );
                    assert_eq!(f.file.expiry_time, "2024-08-15T05:56:14.346Z");
                }
                crate::object::file::File::External(f) => {
                    assert_eq!(f.name, Some("Favicon.ico".to_string()));
                    assert_eq!(f.external.url, "https://www.notion.so/images/favicon.ico");
                }
                _ => panic!(),
            }
        }
    }
}
