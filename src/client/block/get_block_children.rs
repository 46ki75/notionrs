use crate::error::{Error, api_error::ApiError};

#[derive(Debug)]
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
    ) -> Result<crate::list_response::ListResponse<crate::block::BlockResponse>, Error> {
        let mut result_blocks: Vec<crate::block::BlockResponse> = vec![];

        let block_id = &self.block_id.ok_or(Error::RequestParameter(
            "`block_id` has not been set.".to_string(),
        ))?;

        let start_cursor = self.start_cursor;

        let url = format!("https://api.notion.com/v1/blocks/{}/children", block_id);

        let mut query_params: Vec<(String, String)> =
            vec![("page_size".to_string(), self.page_size.to_string())];

        if let Some(ref cursor) = start_cursor {
            query_params.push(("start_cursor".to_string(), cursor.to_string()))
        }

        let request = self.reqwest_client.get(url).query(&query_params);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.bytes().await?;

            let error_json = serde_json::from_slice::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.bytes().await?;

        let block_list_response = serde_json::from_slice::<
            crate::list_response::ListResponse<crate::block::BlockResponse>,
        >(&body)?;

        result_blocks.extend(block_list_response.results);

        Ok(crate::list_response::ListResponse {
            object: "list".into(),
            results: result_blocks,
            next_cursor: start_cursor.clone(),
            has_more: Some(start_cursor.is_some()),
            r#type: Some("list".into()),
        })
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

    // TODO: docs for page_size
    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = page_size;
        self
    }
}
