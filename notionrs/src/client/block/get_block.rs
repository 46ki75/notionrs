#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct GetBlockClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,
}

impl GetBlockClient {
    // TODO: docs for send
    pub async fn send(
        self,
    ) -> Result<notionrs_types::object::block::BlockResponse, crate::error::Error> {
        let block_id = self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
        ))?;

        let url = format!("https://api.notion.com/v1/blocks/{}", block_id);

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

        let block = serde_json::from_slice::<notionrs_types::object::block::BlockResponse>(&body)?;

        Ok(block)
    }
}
