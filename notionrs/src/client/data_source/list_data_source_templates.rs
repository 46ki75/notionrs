use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

/// @see <https://developers.notion.com/reference/list-data-source-templates>
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ListDataSourceTemplatesClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) data_source_id: Option<String>,

    pub(crate) name: Option<String>,

    pub(crate) start_cursor: Option<String>,

    pub(crate) page_size: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListDataSourceTemplatesRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u8>,
}

impl ListDataSourceTemplatesClient {
    pub async fn send(self) -> Result<DataSourceTemplateListResponse, crate::error::Error> {
        let data_source_id = match self.data_source_id {
            Some(id) => id,
            None => {
                return Err(crate::error::Error::RequestParameter(
                    "data_source_id is not set.".to_string(),
                ));
            }
        };

        let request_params = serde_urlencoded::to_string(&ListDataSourceTemplatesRequestParams {
            name: self.name,
            start_cursor: self.start_cursor,
            page_size: self.page_size,
        })?;

        let url = format!(
            "https://api.notion.com/v1/data_sources/{data_source_id}/templates?{request_params}"
        );

        let request = self
            .reqwest_client
            .get(url)
            .header("Content-Type", "application/json");

        let response = request
            .send()
            .await
            .map_err(|e| crate::error::Error::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(crate::error::Error::try_from_response_async(response).await);
        }

        let body = response
            .bytes()
            .await
            .map_err(|e| crate::error::Error::BodyParse(e.to_string()))?;

        let pages = serde_json::from_slice::<DataSourceTemplateListResponse>(&body)?;

        Ok(pages)
    }
}
