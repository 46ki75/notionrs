use serde::{Deserialize, Serialize};

use crate::{
    error::{api_error::ApiError, Error},
    filter::Filter,
    list_response::ListResponse,
    page::page_response::PageResponse,
    prelude::ToJson,
};

#[derive(Debug, Default)]
pub struct SearchClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) body: SearchRequestBody,

    pub(crate) fetch_all: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<Filter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<crate::database::Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl SearchClient {
    pub async fn send(mut self) -> Result<ListResponse<PageResponse>, Error> {
        if self.fetch_all {
            let mut results: Vec<PageResponse> = vec![];

            self.body.page_size = Some(100);

            loop {
                let url = String::from("https://api.notion.com/v1/search");

                let request_body = self.body.to_json().to_string();

                let request = self
                    .reqwest_client
                    .post(url)
                    .header("Content-Type", "application/json")
                    .body(request_body);

                let response = request.send().await?;

                if !response.status().is_success() {
                    let error_body = response.text().await?;

                    let error_json = serde_json::from_str::<ApiError>(&error_body)?;

                    return Err(Error::Api(Box::new(error_json)));
                }

                let body = response.text().await?;

                let mut pages = serde_json::from_str::<ListResponse<PageResponse>>(&body)?;

                results.extend(pages.results);

                if pages.has_more.unwrap_or(false) {
                    self.body.start_cursor = pages.next_cursor;
                } else {
                    pages.results = results;
                    return Ok(pages);
                }
            }
        } else {
            let url = String::from("https://api.notion.com/v1/search");

            let request_body = self.body.to_json().to_string();

            let request = self
                .reqwest_client
                .post(url)
                .header("Content-Type", "application/json")
                .body(request_body);

            let response = request.send().await?;

            if !response.status().is_success() {
                let error_body = response.text().await?;

                let error_json = serde_json::from_str::<ApiError>(&error_body)?;

                return Err(Error::Api(Box::new(error_json)));
            }

            let body = response.text().await?;

            let pages = serde_json::from_str::<ListResponse<PageResponse>>(&body)?;

            Ok(pages)
        }
    }

    pub fn query<T: AsRef<str>>(mut self, query: T) -> Self {
        self.body.query = Some(query.as_ref().to_string());
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

    /// Normally, you can only retrieve up to 100 records in one query,
    /// but by setting fetch_all to true, you can retrieve all the data.
    pub fn fetch_all(mut self) -> Self {
        self.fetch_all = true;
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
