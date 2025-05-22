#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct FileUpload {
    pub object: String,

    pub id: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_time: time::OffsetDateTime,

    // pub created_by: crate::object::user::User,
    #[serde(with = "time::serde::rfc3339")]
    pub last_edited_time: time::OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub expiry_time: time::OffsetDateTime,

    pub upload_url: Option<String>,

    pub complete_url: Option<String>,

    pub archived: bool,

    pub status: FileUploadStatus,

    pub filename: String,

    pub content_type: String,

    pub content_length: u64,

    pub number_of_parts: Option<NumberOfParts>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileUploadStatus {
    Pending,
    Uploaded,
    Expired,
    Failed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct NumberOfParts {
    pub total: u32,
    pub sent: u32,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_file_upload() {
        let json_data = r#"
        {
            "object": "file_upload",
            "id": "1fb34608-d5c9-81c5-8327-00b20e785bdb",
            "created_time": "2025-05-22T05:53:00.000Z",
            "created_by": {
                "id": "75666148-f814-439c-a95b-6350b22345e6",
                "type": "bot"
            },
            "last_edited_time": "2025-05-22T05:53:00.000Z",
            "expiry_time": "2025-05-22T06:53:00.000Z",
            "archived": false,
            "status": "uploaded",
            "filename": "85323087.jpg",
            "content_type": "image/jpeg",
            "content_length": 35183,
            "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
            "request_id": "40a4f669-38e3-4f50-b4c5-4d4b705ee6d6"
        }
        "#;

        let file_upload = serde_json::from_str::<FileUpload>(json_data).unwrap();

        let FileUpload {
            object,
            id,
            created_time: _,
            // created_by: _,
            last_edited_time: _,
            expiry_time: _,
            upload_url: _,
            complete_url: _,
            archived: _,
            status: _,
            filename,
            content_type: _,
            content_length: _,
            number_of_parts: _,
        } = file_upload;

        assert_eq!(object, "file_upload".to_owned());
        assert_eq!(id, "1fb34608-d5c9-81c5-8327-00b20e785bdb".to_owned());
        assert_eq!(filename, "85323087.jpg".to_owned());
    }
}
