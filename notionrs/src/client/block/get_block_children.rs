#[derive(Debug, notionrs_macro::Setter)]
pub struct GetBlockChildrenClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    pub(crate) block_id: Option<String>,

    pub(crate) page_size: u64,

    pub(crate) start_cursor: Option<String>,
}

impl GetBlockChildrenClient {
    // TODO: docs for send
    pub async fn send(
        self,
    ) -> Result<
        crate::object::response::ListResponse<crate::object::block::BlockResponse>,
        crate::error::Error,
    > {
        let mut result_blocks: Vec<crate::object::block::BlockResponse> = vec![];

        let block_id = &self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
        ))?;

        let start_cursor = self.start_cursor;

        let url = format!("https://api.notion.com/v1/blocks/{}/children", block_id);

        let mut query_params: Vec<(String, String)> =
            vec![("page_size".to_string(), self.page_size.to_string())];

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

        let list_response = serde_json::from_slice::<
            crate::object::response::ListResponse<crate::object::block::BlockResponse>,
        >(&body)?;

        result_blocks.extend(list_response.results);

        Ok(crate::object::response::ListResponse {
            object: "list".into(),
            results: result_blocks,
            next_cursor: start_cursor.clone(),
            has_more: Some(start_cursor.is_some()),
            r#type: Some("list".into()),
        })
    }
}
