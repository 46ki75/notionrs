#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct RetrieveFileUploadClient {
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) file_upload_id: Option<String>,
}

impl RetrieveFileUploadClient {
    pub async fn send(self) -> Result<notionrs_types::prelude::FileUpload, crate::error::Error> {
        let file_upload_id = self
            .file_upload_id
            .ok_or(crate::error::Error::RequestParameter(
                "`file_upload_id` is not set.".to_owned(),
            ))?;

        let request = self
            .reqwest_client
            .get(format!(
                "https://api.notion.com/v1/file_uploads/{file_upload_id}"
            ))
            .header("Content-Type", "application/json");

        let response =
            crate::util::send_and_convert::<notionrs_types::prelude::FileUpload>(request).await?;

        Ok(response)
    }
}
