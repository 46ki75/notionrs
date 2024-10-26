use serde::{Deserialize, Serialize};

use crate::{
    error::{api_error::ApiError, Error},
    page::page_response::PageResponse,
};

#[derive(Debug, Default)]
pub struct UpdatePageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,

    pub(crate) properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,

    pub(crate) icon: Option<crate::others::icon::Icon>,

    pub(crate) cover: Option<crate::File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePageRequestBody {
    pub(crate) properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<crate::others::icon::Icon>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cover: Option<crate::File>,
}

impl UpdatePageClient {
    pub async fn send(self) -> Result<PageResponse, Error> {
        let page_id = self.page_id.ok_or(crate::error::Error::RequestParameter(
            "You need to specify either the page_id.".to_string(),
        ))?;

        let request_body_struct = UpdatePageRequestBody {
            properties: self.properties,
            icon: self.icon,
            cover: self.cover,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = format!("https://api.notion.com/v1/pages/{}", page_id);

        println!("{}", request_body);

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(request_body);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.text().await?;

        let page: PageResponse = serde_json::from_str::<PageResponse>(&body)?;

        Ok(page)
    }

    pub fn page_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.page_id = Some(page_id.as_ref().to_string());
        self
    }

    pub fn properties(
        mut self,
        properties: std::collections::HashMap<String, crate::page::properties::PageProperty>,
    ) -> Self {
        self.properties = properties;
        self
    }

    pub fn icon(mut self, icon: crate::others::icon::Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn cover(mut self, cover: crate::File) -> Self {
        self.cover = Some(cover);
        self
    }
}
