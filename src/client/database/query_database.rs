use serde::{Deserialize, Serialize};

use crate::{
    filter::Filter, list_response::ListResponse, page::page_response::PageResponse, prelude::ToJson,
};

#[derive(Debug, Default)]
pub struct QueryDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) body: QueryDatabaseRequestBody,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryDatabaseRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<Filter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<crate::database::Sort>,

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

                let request_body = self.body.to_json().to_string();

                let request = self
                    .reqwest_client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .body(request_body);

                let response = request.send().await?;

                if !response.status().is_success() {
                    return Err(crate::error::Error::try_from_response_async(response).await);
                }

                let body = response.bytes().await?;

                let pages = serde_json::from_slice::<ListResponse<PageResponse>>(&body)?;

                Ok(pages)
            }
            None => Err(crate::error::Error::RequestParameter(
                "database_id is empty".to_string(),
            )),
        }
    }

    /// Specify the ID of the database to query.
    pub fn database_id<T: AsRef<str>>(mut self, database_id: T) -> Self {
        self.database_id = Some(database_id.as_ref().to_string());
        self
    }

    /// The amount of data retrieved in one query.
    /// If not specified, the default is 100.
    /// When `fetch_all` is set to true, it will also be 100.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.body.page_size = Some(page_size);
        self
    }

    /// Specify the cursor position at the beginning. In the query result,
    /// there is a field called `next_cursor` through
    /// which information is passed at the end.
    pub fn start_cursor<T: AsRef<str>>(mut self, start_cursor: T) -> Self {
        self.body.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.body.filter = Some(filter);
        self
    }

    pub fn sorts(mut self, sorts: Vec<crate::database::Sort>) -> Self {
        self.body.sorts = sorts;
        self
    }
}
