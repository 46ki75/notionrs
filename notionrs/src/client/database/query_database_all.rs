use serde::{Deserialize, Serialize};

use crate::object::response::ListResponse;

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct QueryDatabaseAllClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) filter: Option<crate::object::request::filter::Filter>,

    pub(crate) sorts: Vec<crate::object::request::sort::Sort>,

    pub(crate) start_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryDatabaseAllRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<crate::object::request::filter::Filter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<crate::object::request::sort::Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl QueryDatabaseAllClient {
    pub async fn send(self) -> Result<Vec<crate::object::page::PageResponse>, crate::error::Error> {
        match self.database_id {
            Some(id) => {
                let mut start_cursor = self.start_cursor.clone();
                let mut results: Vec<crate::object::page::PageResponse> = vec![];

                loop {
                    let url = format!("https://api.notion.com/v1/databases/{}/query", id);

                    let request_body = serde_json::to_string(&QueryDatabaseAllRequestBody {
                        filter: self.filter.clone(),
                        sorts: self.sorts.clone(),
                        start_cursor,
                        page_size: Some(100),
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

                    let pages = serde_json::from_slice::<
                        ListResponse<crate::object::page::PageResponse>,
                    >(&body)?;

                    results.extend(pages.results);

                    if pages.has_more.unwrap_or(false) {
                        start_cursor = pages.next_cursor;
                    } else {
                        return Ok(results);
                    }
                }
            }
            None => Err(crate::error::Error::RequestParameter(
                "`database_id` is not set.".to_string(),
            )),
        }
    }
}
