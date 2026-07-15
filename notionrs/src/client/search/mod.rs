pub mod search_database;
pub mod search_page;

pub use search_database::*;
pub use search_page::*;

use serde::{Deserialize, Serialize};

use notionrs_types::object::response::ListResponse;

#[derive(Debug, Default)]
pub struct SearchClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) body: SearchRequestBody,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<notionrs_types::object::request::search::SearchFilterRequest>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) sort: Option<notionrs_types::object::request::search::SearchSort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl SearchClient {
    pub async fn send(
        self,
    ) -> Result<ListResponse<notionrs_types::object::response::SearchResultItem>, crate::error::Error>
    {
        let url = String::from("https://api.notion.com/v1/search");

        let request_body = serde_json::to_string(&self.body)?;

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
            ListResponse<notionrs_types::object::response::SearchResultItem>,
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
        self.sort(notionrs_types::object::request::search::SearchSort::asc())
    }

    pub fn sort_timestamp_desc(self) -> Self {
        self.sort(notionrs_types::object::request::search::SearchSort::desc())
    }

    /// Sort by relevance to the search query.
    pub fn sort_relevance(self) -> Self {
        self.sort(notionrs_types::object::request::search::SearchSort::relevance())
    }

    /// Restricts search results to only database types.
    /// It is recommended to use the search_database method, which returns results that are not in an enum format.
    pub fn filter_database(self) -> Self {
        self.filter_object(notionrs_types::object::request::search::SearchFilterType::DataSource)
    }

    /// Restricts search results to only page types.
    /// It is recommended to use the search_page method, which returns results that are not in an enum format.
    pub fn filter_page(self) -> Self {
        self.filter_object(notionrs_types::object::request::search::SearchFilterType::Page)
    }

    /// Restricts search results to trashed or live objects.
    pub fn filter_in_trash(mut self, in_trash: bool) -> Self {
        self.body.filter = Some(
            self.body
                .filter
                .unwrap_or_else(|| {
                    notionrs_types::object::request::search::SearchFilterRequest::trash(in_trash)
                })
                .in_trash(in_trash),
        );
        self
    }

    fn filter_object(
        mut self,
        value: notionrs_types::object::request::search::SearchFilterType,
    ) -> Self {
        let in_trash = match self.body.filter {
            Some(notionrs_types::object::request::search::SearchFilterRequest::Object {
                in_trash,
                ..
            }) => in_trash,
            Some(notionrs_types::object::request::search::SearchFilterRequest::InTrash {
                in_trash,
            }) => Some(in_trash),
            None => None,
        };
        self.body.filter = Some(
            notionrs_types::object::request::search::SearchFilterRequest::Object {
                value,
                property: String::from("object"),
                in_trash,
            },
        );
        self
    }

    fn sort(mut self, sort: notionrs_types::object::request::search::SearchSort) -> Self {
        self.body.sort = Some(sort);
        self
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn filter_in_trash_serializes_supported_filter_shapes() {
        let filter = SearchClient::default().filter_in_trash(true).body.filter;
        assert_eq!(
            serde_json::to_value(filter).unwrap(),
            serde_json::json!({"in_trash": true})
        );

        let filter = SearchClient::default()
            .filter_page()
            .filter_in_trash(true)
            .body
            .filter;
        assert_eq!(
            serde_json::to_value(filter).unwrap(),
            serde_json::json!({"property": "object", "value": "page", "in_trash": true})
        );

        let filter = SearchClient::default()
            .filter_in_trash(false)
            .filter_database()
            .body
            .filter;
        assert_eq!(
            serde_json::to_value(filter).unwrap(),
            serde_json::json!({"property": "object", "value": "data_source", "in_trash": false})
        );
    }
}
