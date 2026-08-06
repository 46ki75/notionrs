#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct SendFileUploadClient {
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) file_upload_id: Option<String>,

    pub(crate) file: Option<Vec<u8>>,

    pub(crate) part_number: Option<u32>,

    pub(crate) filename: Option<String>,

    /// MIME type of the multipart file part.
    pub(crate) content_type: Option<String>,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn reject_invalid_content_type_before_sending() {
        let error = SendFileUploadClient::default()
            .file_upload_id("upload-id")
            .file(vec![0])
            .content_type("not a MIME type")
            .send()
            .await
            .unwrap_err();

        assert!(matches!(error, crate::error::Error::RequestParameter(_)));
    }
}

impl SendFileUploadClient {
    pub async fn send(self) -> Result<notionrs_types::prelude::FileUpload, crate::error::Error> {
        let file_upload_id = self
            .file_upload_id
            .ok_or(crate::error::Error::RequestParameter(
                "`file_upload_id` is not set.".to_owned(),
            ))?;

        let file = self.file.ok_or(crate::error::Error::RequestParameter(
            "`file` is not set.".to_owned(),
        ))?;

        let mut file_part = reqwest::multipart::Part::bytes(file)
            .file_name(self.filename.unwrap_or("untitled".to_owned()));
        if let Some(content_type) = self.content_type {
            file_part = file_part.mime_str(&content_type).map_err(|error| {
                crate::error::Error::RequestParameter(format!(
                    "`content_type` is not a valid MIME type: {error}"
                ))
            })?;
        }

        let form = match self.part_number {
            Some(part_number) => reqwest::multipart::Form::new()
                .part("file", file_part)
                .text("part_number", part_number.to_string()),
            None => reqwest::multipart::Form::new().part("file", file_part),
        };

        let request = self
            .reqwest_client
            .post(format!(
                "https://api.notion.com/v1/file_uploads/{file_upload_id}/send"
            ))
            .multipart(form);

        let response =
            crate::util::send_and_convert::<notionrs_types::prelude::FileUpload>(request).await?;

        Ok(response)
    }
}
