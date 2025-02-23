use crate::{
    error::{Error, api_error::ApiError},
    page::page_response::PageResponse,
};

#[derive(Debug)]
pub struct GetPageClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) page_id: Option<String>,
}

impl GetPageClient {
    pub async fn send(self) -> Result<PageResponse, Error> {
        match self.page_id {
            Some(id) => {
                let url = format!("https://api.notion.com/v1/pages/{}", id);

                let request = self.reqwest_client.get(url);

                let response = request.send().await?;

                if !response.status().is_success() {
                    let error_body = response.bytes().await?;

                    let error_json = serde_json::from_slice::<ApiError>(&error_body)?;

                    return Err(Error::Api(Box::new(error_json)));
                }

                let body = response.bytes().await?;

                let page: PageResponse = serde_json::from_slice::<PageResponse>(&body)?;

                Ok(page)
            }
            None => Err(Error::RequestParameter("user_id is empty".to_string())),
        }
    }

    /// Specify the ID of the page.
    /// The ID is also included in the Notion page URL.
    pub fn page_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.page_id = Some(page_id.as_ref().to_string());
        self
    }
}
