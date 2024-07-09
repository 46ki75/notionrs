use crate::{
    error::{NotionApiError, NotionError},
    page::page_response::PageResponse,
};

pub struct GetPageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,
}

impl GetPageClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<PageResponse, NotionError> {
        match self.page_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/pages/{}", id);

                let res = self.reqwest_client.get(url).send().await?;

                if !res.status().is_success() {
                    let api_error = res.json::<NotionApiError>().await?;
                    return Err(NotionError::NotionApiError(Box::new(api_error)));
                }

                let body = res.json::<PageResponse>().await?;

                Ok(body)
            }
            None => Err(NotionError::NotionRequestParameterError(
                "user_id is empty".to_string(),
            )),
        }
    }

    /// Specify the ID of the page.
    /// The ID is also included in the Notion page URL.
    pub fn page_id<T: AsRef<str>>(mut self, user_id: T) -> Self {
        self.page_id = Some(user_id.as_ref().to_string());
        self
    }
}
