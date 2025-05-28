#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct SendFileUploadClient {
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) file_upload_id: Option<String>,

    pub(crate) file: Option<Vec<u8>>,

    pub(crate) part_number: Option<u32>,

    pub(crate) filename: Option<String>,
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

        let form = match self.part_number {
            Some(part_number) => reqwest::multipart::Form::new()
                .part(
                    "file",
                    reqwest::multipart::Part::bytes(file)
                        .file_name(self.filename.unwrap_or("untitled".to_owned())),
                )
                .text("part_number", part_number.to_string()),
            None => reqwest::multipart::Form::new().part(
                "file",
                reqwest::multipart::Part::bytes(file)
                    .file_name(self.filename.unwrap_or("untitled".to_owned())),
            ),
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
