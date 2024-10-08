use crate::error::{api_error::NotionApiError, NotionError};

#[derive(Debug)]
pub struct GetBlockClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,
}

impl GetBlockClient {
    // TODO: docs for send
    pub async fn send(self) -> Result<crate::block::Block, NotionError> {
        let block_id = self
            .block_id
            .ok_or(NotionError::NotionRequestParameterError(
                "`block_id` has not been set.".to_string(),
            ))?;

        let url = format!("https://api.notion.com/v1/blocks/{}", block_id);

        let request = self.reqwest_client.get(url);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<NotionApiError>(&error_body)?;

            return Err(NotionError::NotionApiError(Box::new(error_json)));
        }

        let body = response.text().await?;

        let block = serde_json::from_str::<crate::block::Block>(&body)?;

        Ok(block)
    }

    // TODO: docs for block_id
    pub fn block_id<T: AsRef<str>>(mut self, page_id: T) -> Self {
        self.block_id = Some(page_id.as_ref().to_string());
        self
    }
}
