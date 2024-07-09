use serde::{Deserialize, Serialize};

use crate::others::file::File;

/// <https://developers.notion.com/reference/page-property-values#files>
///
/// Example filea page property value
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
#[derive(Debug, Deserialize, Serialize)]
pub struct PageFilesProperty {
    pub id: String,
    pub files: Vec<File>,
}
