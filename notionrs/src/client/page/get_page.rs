use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, notionrs_macro::Setter)]
pub struct GetPageClient<
    T = std::collections::HashMap<String, notionrs_types::prelude::PageProperty>,
> {
    pub(crate) reqwest_client: reqwest::Client,

    /// Specify the ID of the page.
    /// The ID is also included in the Notion page URL.
    pub(crate) page_id: Option<String>,

    #[setter(skip)]
    pub(crate) _phantom: PhantomData<T>,
}

impl<T> Default for GetPageClient<T> {
    fn default() -> Self {
        Self {
            reqwest_client: reqwest::Client::default(),
            page_id: None,
            _phantom: PhantomData,
        }
    }
}

impl<T> GetPageClient<T>
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::page::PageResponse<T>, crate::error::Error> {
        match self.page_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/pages/{}", id);

                let request = self.reqwest_client.get(url);

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

                let page =
                    serde_json::from_slice::<notionrs_types::object::page::PageResponse<T>>(&body)?;

                Ok(page)
            }
            None => Err(crate::error::Error::RequestParameter(
                "`user_id` is not set.".to_string(),
            )),
        }
    }
}
