use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct UpdateBlockClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Identifier for a block. Also accepts a page ID.
    pub(crate) block_id: Option<String>,

    /// The ID of the existing block that the new block should be appended after.
    pub(crate) archived: Option<bool>,

    pub(crate) block: Option<crate::block::Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBlockRequestBody {
    #[serde(flatten)]
    pub(crate) block: crate::block::Block,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) archived: Option<bool>,
}

impl UpdateBlockClient {
    // TODO: docs for send
    pub async fn send(self) -> Result<crate::block::BlockResponse, crate::error::Error> {
        let block_id = self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
        ))?;

        let block = self.block.ok_or(crate::error::Error::RequestParameter(
            "`block` is not set.".to_string(),
        ))?;

        let request_body_struct = UpdateBlockRequestBody {
            block,
            archived: self.archived,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = format!("https://api.notion.com/v1/blocks/{}", block_id);

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(request_body);

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

        let block = serde_json::from_slice::<crate::block::BlockResponse>(&body)?;

        Ok(block)
    }

    /// Identifier for a block. Also accepts a page ID.
    pub fn block_id<T: AsRef<str>>(mut self, block_id: T) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    pub fn block(mut self, block: crate::block::Block) -> Self {
        self.block = Some(block);
        self
    }

    // TODO: docs for after
    pub fn archived(mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self
    }
}
