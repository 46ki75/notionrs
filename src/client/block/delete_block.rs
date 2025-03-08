#[derive(Debug)]
pub struct DeleteBlockClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,
}

impl DeleteBlockClient {
    // TODO: docs for send
    pub async fn send(self) -> Result<crate::block::BlockResponse, crate::error::Error> {
        let block_id = self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` has not been set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/blocks/{}", block_id);

        let request = self.reqwest_client.delete(url);

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(crate::error::Error::try_from_response_async(response).await);
        }

        let body = response.bytes().await?;

        let block = serde_json::from_slice::<crate::block::BlockResponse>(&body)?;

        Ok(block)
    }

    // TODO: docs for block_id
    pub fn block_id<T: AsRef<str>>(mut self, block_id: T) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }
}
