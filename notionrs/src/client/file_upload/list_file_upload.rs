use notionrs_types::prelude::*;

#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ListFileUploadClient {
    pub(crate) reqwest_client: reqwest::Client,

    pub status: Option<FileUploadStatus>,

    pub start_cursor: Option<String>,

    pub page_size: Option<u8>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ListFileUploadQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FileUploadStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u8>,
}

crate::impl_paginate!(ListFileUploadClient, FileUpload);

impl ListFileUploadClient {
    pub async fn send(self) -> Result<ListResponse<FileUpload>, crate::error::Error> {
        let params = ListFileUploadQueryParams {
            status: self.status,
            start_cursor: self.start_cursor,
            page_size: self.page_size,
        };

        let request = self
            .reqwest_client
            .get("https://api.notion.com/v1/file_uploads")
            .query(&params)
            .header("Content-Type", "application/json");

        let response = crate::util::send_and_convert::<ListResponse<FileUpload>>(request).await?;

        Ok(response)
    }
}
