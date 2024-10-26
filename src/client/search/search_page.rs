use serde::{Deserialize, Serialize};

use crate::{
    error::{api_error::ApiError, Error},
    prelude::ToJson,
};

#[derive(Debug, Default)]
pub struct SearchPageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) body: SearchPageRequestBody,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchPageRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<crate::search::SearchFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort: Option<crate::search::SearchSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl SearchPageClient {
    pub async fn send(
        mut self,
    ) -> Result<crate::list_response::ListResponse<crate::page::PageResponse>, Error> {
        let url = String::from("https://api.notion.com/v1/search");

        self.body.filter = Some(crate::search::SearchFilter::page());

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

        let pages = serde_json::from_str::<
            crate::list_response::ListResponse<crate::page::PageResponse>,
        >(&body)?;

        Ok(pages)
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

    pub fn sort_timestamp_asc(self) -> Self {
        self.sort(crate::search::SearchSort::asc())
    }

    pub fn sort_timestamp_desc(self) -> Self {
        self.sort(crate::search::SearchSort::desc())
    }

    fn sort(mut self, sort: crate::search::SearchSort) -> Self {
        self.body.sort = Some(sort);
        self
    }
}
