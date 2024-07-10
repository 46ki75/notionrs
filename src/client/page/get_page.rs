use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::{
    error::{NotionApiError, NotionError},
    page::{page_response::PageResponse, properties::PageProperty},
};

pub struct GetPageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,
}

impl GetPageClient {
    /// Send a request specifying generics.
    /// If you are not using generics, use the `send_default()` method.
    /// ```no_run
    /// use notionrs::client;
    /// use notionrs::error::NotionError;
    /// use notionrs::page::properties::title::PageTitleProperty;
    /// use notionrs::to_json::ToJson;
    ///
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct MyResponse {
    ///     #[serde(rename = "Title")]
    ///     title: PageTitleProperty,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///
    ///     let client = client::NotionClient::new();
    ///     let res = client
    ///         .get_page()
    ///         .page_id("7ae4e830-e5bd-4d2c-80d9-ca09ea397c11")
    ///         .send::<MyResponse>()
    ///         .await?;
    ///     println!("{:?}", res.properties.title);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn send<T>(self) -> Result<PageResponse<T>, NotionError>
    where
        T: DeserializeOwned,
    {
        match self.page_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/pages/{}", id);

                let res = self.reqwest_client.get(url).send().await?;

                if !res.status().is_success() {
                    let api_error = res.json::<NotionApiError>().await?;
                    return Err(NotionError::NotionApiError(Box::new(api_error)));
                }

                let body = res.json::<PageResponse<T>>().await?;

                Ok(body)
            }
            None => Err(NotionError::NotionRequestParameterError(
                "user_id is empty".to_string(),
            )),
        }
    }

    /// Send a request without specifying type generics.
    /// Rust does not recognize which properties exist.
    pub async fn send_default(
        self,
    ) -> Result<PageResponse<HashMap<String, PageProperty>>, NotionError> {
        self.send::<HashMap<String, PageProperty>>().await
    }

    /// Specify the ID of the page.
    /// The ID is also included in the Notion page URL.
    pub fn page_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.page_id = Some(user_id.as_ref().to_string());
        self
    }
}
