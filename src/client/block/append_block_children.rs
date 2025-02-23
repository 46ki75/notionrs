use serde::{Deserialize, Serialize};

use crate::error::{Error, api_error::ApiError};

#[derive(Debug)]
pub struct AppendBlockChildrenClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Identifier for a block. Also accepts a page ID.
    pub(crate) block_id: Option<String>,

    /// The ID of the existing block that the new block should be appended after.
    pub(crate) after: Option<String>,

    pub(crate) children: Vec<crate::block::Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendBlockChildrenRequestBody {
    pub(crate) children: Vec<crate::block::Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) after: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub(crate) page_size: Option<u32>,
}

impl AppendBlockChildrenClient {
    // TODO: docs for send
    pub async fn send(
        self,
    ) -> Result<crate::list_response::ListResponse<crate::block::BlockResponse>, Error> {
        let block_id = self.block_id.ok_or(Error::RequestParameter(
            "`block_id` has not been set.".to_string(),
        ))?;

        let request_body_struct = AppendBlockChildrenRequestBody {
            children: self.children,
            after: self.after,
        };

        let request_body = serde_json::to_string(&request_body_struct)?;

        let url = format!("https://api.notion.com/v1/blocks/{}/children", block_id);

        let request = self
            .reqwest_client
            .patch(url)
            .header("Content-Type", "application/json")
            .body(request_body);

        let response = request.send().await?;

        if !response.status().is_success() {
            let error_body = response.bytes().await?;

            let error_json = serde_json::from_slice::<ApiError>(&error_body)?;

            return Err(Error::Api(Box::new(error_json)));
        }

        let body = response.bytes().await?;

        let block = serde_json::from_slice::<
            crate::list_response::ListResponse<crate::block::BlockResponse>,
        >(&body)?;

        Ok(block)
    }

    /// Identifier for a block. Also accepts a page ID.
    pub fn block_id<T: AsRef<str>>(mut self, block_id: T) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    // TODO: docs for after
    pub fn after<T: AsRef<str>>(mut self, after: T) -> Self {
        self.after = Some(after.as_ref().to_string());
        self
    }

    /// The ID of the existing block that the new block should be appended after.
    pub fn children(mut self, children: Vec<crate::block::Block>) -> Self {
        self.children = children;
        self
    }
}
