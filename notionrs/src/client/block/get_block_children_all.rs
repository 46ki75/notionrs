#[derive(Debug, notionrs_macro::Setter)]
pub struct GetBlockChildrenAllClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,

    pub(crate) start_cursor: Option<String>,
}

impl GetBlockChildrenAllClient {
    // TODO: docs for send
    pub async fn send(self) -> Result<Vec<crate::block::BlockResponse>, crate::error::Error> {
        let mut result_blocks: Vec<crate::block::BlockResponse> = vec![];

        let block_id = &self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
        ))?;

        let mut start_cursor = self.start_cursor;

        loop {
            let url = format!("https://api.notion.com/v1/blocks/{}/children", block_id);

            let mut query_params: Vec<(String, String)> = vec![];

            if let Some(ref cursor) = start_cursor {
                query_params.push(("start_cursor".to_string(), cursor.to_string()))
            }

            let request = self.reqwest_client.get(url).query(&query_params);

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

            let block_list_response = serde_json::from_slice::<
                crate::list_response::ListResponse<crate::block::BlockResponse>,
            >(&body)?;

            result_blocks.extend(block_list_response.results);

            start_cursor = block_list_response.next_cursor;

            if start_cursor.is_none() {
                return Ok(result_blocks);
            }
        }
    }
}
