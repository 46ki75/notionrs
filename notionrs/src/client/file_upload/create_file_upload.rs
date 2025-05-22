#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct CreateFileUploadClient {
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) mode: FileUplpadMode,

    pub(crate) filename: Option<String>,

    pub(crate) content_type: Option<String>,

    pub(crate) number_of_parts: Option<i32>,

    pub(crate) external_url: Option<String>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateFileUploadRequestBody {
    pub(crate) mode: FileUplpadMode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) number_of_parts: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) external_url: Option<String>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FileUplpadMode {
    #[default]
    SinglePart,
    MultiPart,
    ExternalUrl,
}

impl CreateFileUploadClient {
    fn validate_request(&self) -> Result<(), crate::error::Error> {
        if matches!(
            self.mode,
            FileUplpadMode::MultiPart | FileUplpadMode::ExternalUrl
        ) && self.filename.is_none()
        {
            return Err(crate::error::Error::RequestParameter(
                "`filename` is required when `mode` is `multi_part` or `external_url`.".to_owned(),
            ));
        };

        if matches!(self.mode, FileUplpadMode::MultiPart) && self.number_of_parts.is_none() {
            return Err(crate::error::Error::RequestParameter(
                "`number_of_parts` is required when `mode` is `multi_part`.".to_owned(),
            ));
        };

        if matches!(self.mode, FileUplpadMode::ExternalUrl) && self.filename.is_none() {
            return Err(crate::error::Error::RequestParameter(
                "`external_url` is required when `mode` is `external_url`.".to_owned(),
            ));
        };

        Ok(())
    }

    pub async fn send(self) -> Result<notionrs_types::prelude::FileUpload, crate::error::Error> {
        self.validate_request()?;

        let request_body = serde_json::to_string(&CreateFileUploadRequestBody {
            mode: self.mode,
            filename: self.filename,
            content_type: self.content_type,
            number_of_parts: self.number_of_parts,
            external_url: self.external_url,
        })?;

        let request = self
            .reqwest_client
            .post("https://api.notion.com/v1/file_uploads")
            .header("Content-Type", "application/json")
            .body(request_body);

        let response =
            crate::util::send_and_convert::<notionrs_types::prelude::FileUpload>(request).await?;

        Ok(response)
    }
}
