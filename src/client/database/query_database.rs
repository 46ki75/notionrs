use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    error::{api_error::NotionApiError, NotionError},
    filter::Filter,
    list_response::ListResponse,
    page::page_response::PageResponse,
    prelude::ToJson,
};

#[derive(Debug)]
pub struct QueryDatabaseClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) database_id: Option<String>,

    pub(crate) body: QueryDatabaseRequestBody,

    pub(crate) recursive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDatabaseRequestBody {
    // TODO: implement filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<Filter>,

    // TODO: implement sort
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl QueryDatabaseClient {
    pub async fn send<T>(mut self) -> Result<ListResponse<PageResponse<T>>, NotionError>
    where
        T: DeserializeOwned,
    {
        match self.database_id {
            Some(id) => {
                if self.recursive {
                    let mut results: Vec<PageResponse<T>> = vec![];

                    self.body.page_size = Some(100);

                    loop {
                        let url = format!("https://api.notion.com/v1/databases/{}/query", id);

                        let request_body = self.body.to_json().to_string();

                        let request = self
                            .reqwest_client
                            .post(url)
                            .header("Content-Type", "application/json")
                            .body(request_body);

                        let response = request.send().await?;

                        if !response.status().is_success() {
                            let error_body = response.text().await?;

                            let error_json = serde_json::from_str::<NotionApiError>(&error_body)?;

                            return Err(NotionError::NotionApiError(Box::new(error_json)));
                        }

                        let mut body = response.json::<ListResponse<PageResponse<T>>>().await?;

                        results.extend(body.results);

                        if body.has_more.unwrap_or(false) {
                            self.body.start_cursor = body.next_cursor;
                        } else {
                            body.results = results;
                            return Ok(body);
                        }
                    }
                } else {
                    let url = format!("https://api.notion.com/v1/databases/{}/query", id);

                    let request_body = self.body.to_json().to_string();

                    let request = self
                        .reqwest_client
                        .post(url)
                        .header("Content-Type", "application/json")
                        .body(request_body);

                    let response = request.send().await?;

                    if !response.status().is_success() {
                        let error_body = response.text().await?;

                        let error_json = serde_json::from_str::<NotionApiError>(&error_body)?;

                        return Err(NotionError::NotionApiError(Box::new(error_json)));
                    }

                    let body = response.json::<ListResponse<PageResponse<T>>>().await?;

                    Ok(body)
                }
            }
            None => Err(NotionError::NotionRequestParameterError(
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
    /// When `recursive` is set to true, it will also be 100.
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

    /// Normally, you can only retrieve up to 100 records in one query,
    /// but by setting recursive to true, you can recursively retrieve all the data.
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn filter(mut self, filter: Filter) -> Self {
        self.body.filter = Some(filter);
        self
    }
}
