use serde::{Deserialize, Serialize};

use crate::{object::page::page_response::PageResponse, object::response::ListResponse};

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct QueryDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) filter: Option<crate::object::request::filter::Filter>,

    pub(crate) sorts: Vec<crate::object::database::Sort>,

    pub(crate) start_cursor: Option<String>,

    pub(crate) page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryDatabaseRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<crate::object::request::filter::Filter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<crate::object::database::Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl QueryDatabaseClient {
    pub async fn send(self) -> Result<ListResponse<PageResponse>, crate::error::Error> {
        match self.database_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/databases/{}/query", id);

                let request_body = serde_json::to_string(&QueryDatabaseRequestBody {
                    filter: self.filter.clone(),
                    sorts: self.sorts.clone(),
                    start_cursor: self.start_cursor.clone(),
                    page_size: self.page_size,
                })?;

                let request = self
                    .reqwest_client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .body(request_body);

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

                let pages = serde_json::from_slice::<ListResponse<PageResponse>>(&body)?;

                Ok(pages)
            }
            None => Err(crate::error::Error::RequestParameter(
                "`database_id` is not set".to_string(),
            )),
        }
    }
}
