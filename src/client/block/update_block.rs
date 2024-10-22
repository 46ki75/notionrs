use serde::{Deserialize, Serialize};

use crate::error::{api_error::ApiError, Error};

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
    pub async fn send(self) -> Result<crate::block::BlockResponse, Error> {
        let block_id = self.block_id.ok_or(Error::RequestParameter(
            "`block_id` has not been set.".to_string(),
        ))?;

        let block = self.block.ok_or(Error::RequestParameter(
            "`block` has not been set.".to_string(),
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

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.text().await?;

            let error_json = serde_json::from_str::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.text().await?;

        let block = serde_json::from_str::<crate::block::BlockResponse>(&body)?;

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
