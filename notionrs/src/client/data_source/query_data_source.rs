use std::marker::PhantomData;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use notionrs_types::prelude::*;

#[derive(Debug, Clone)]
pub struct QueryDataSourceClient<T = std::collections::HashMap<String, notionrs_types::prelude::PageProperty>> {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) data_source_id: Option<String>,

    pub(crate) filter: Option<notionrs_types::object::request::filter::Filter>,

    pub(crate) sorts: Vec<notionrs_types::object::request::sort::Sort>,

    pub(crate) start_cursor: Option<String>,

    pub(crate) page_size: Option<u32>,

    pub(crate) _phantom: PhantomData<T>,
}

impl<T> Default for QueryDataSourceClient<T> {
    fn default() -> Self {
        Self {
            reqwest_client: reqwest::Client::default(),
            data_source_id: None,
            filter: None,
            sorts: Vec::new(),
            start_cursor: None,
            page_size: None,
            _phantom: PhantomData,
        }
    }
}

impl<T> QueryDataSourceClient<T> {
    /// Set the value of the `data_source_id` field.
    pub fn data_source_id<S: AsRef<str>>(mut self, data_source_id: S) -> Self {
        self.data_source_id = Some(data_source_id.as_ref().to_string());
        self
    }

    /// Set the value of the `filter` field.
    pub fn filter(mut self, filter: notionrs_types::object::request::filter::Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// Set the value of the `sorts` field.
    pub fn sorts(mut self, sorts: Vec<notionrs_types::object::request::sort::Sort>) -> Self {
        self.sorts = sorts;
        self
    }

    /// Set the value of the `start_cursor` field.
    pub fn start_cursor<S: AsRef<str>>(mut self, start_cursor: S) -> Self {
        self.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }

    /// Set the value of the `page_size` field.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

impl<T> crate::r#trait::Paginate<notionrs_types::object::page::PageResponse<T>>
    for QueryDataSourceClient<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    fn paginate_start_cursor(self, start_cursor: Option<String>) -> Self {
        match start_cursor {
            Some(c) => self.start_cursor(c),
            None => self,
        }
    }

    fn paginate_send(
        self,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<
                        notionrs_types::object::response::ListResponse<
                            notionrs_types::object::page::PageResponse<T>,
                        >,
                        crate::error::Error,
                    >,
                > + Send
                + Sync,
        >,
    > {
        Box::pin(async { Ok(self.send().await?) })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryDataSourceRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<notionrs_types::object::request::filter::Filter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) sorts: Vec<notionrs_types::object::request::sort::Sort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start_cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) page_size: Option<u32>,
}

impl<T> QueryDataSourceClient<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    pub async fn send(
        self,
    ) -> Result<ListResponse<notionrs_types::object::page::PageResponse<T>>, crate::error::Error>
    {
        match self.data_source_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/data_sources/{}/query", id);

                let request_body = serde_json::to_string(&QueryDataSourceRequestBody {
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

                let pages = serde_json::from_slice::<
                    ListResponse<notionrs_types::object::page::PageResponse<T>>,
                >(&body)?;

                Ok(pages)
            }
            None => Err(crate::error::Error::RequestParameter(
                "`database_id` is not set".to_string(),
            )),
        }
    }
}
