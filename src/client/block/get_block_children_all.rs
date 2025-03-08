#[derive(Debug)]
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
            "`block_id` has not been set.".to_string(),
        ))?;

        let mut start_cursor = self.start_cursor;

        loop {
            let url = format!("https://api.notion.com/v1/blocks/{}/children", block_id);

            let mut query_params: Vec<(String, String)> = vec![];

            if let Some(ref cursor) = start_cursor {
                query_params.push(("start_cursor".to_string(), cursor.to_string()))
            }

            let request = self.reqwest_client.get(url).query(&query_params);

            let response = request.send().await?;

            if !response.status().is_success() {
                return Err(crate::error::Error::try_from_response_async(response).await);
            }

            let body = response.bytes().await?;

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

    // TODO: docs for block_id
    pub fn block_id<T>(mut self, page_id: T) -> Self
    where
        T: AsRef<str>,
    {
        self.block_id = Some(page_id.as_ref().to_string());
        self
    }

    // TODO: docs for start_cursor
    pub fn start_cursor<T>(mut self, start_cursor: T) -> Self
    where
        T: AsRef<str>,
    {
        self.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }
}
