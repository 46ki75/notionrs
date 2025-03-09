use serde::{Deserialize, Serialize};

#[derive(Debug, notionrs_macro::Setter)]
pub struct AppendBlockChildrenClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Identifier for a block. Also accepts a page ID.
    pub(crate) block_id: Option<String>,

    /// The ID of the existing block that the new block should be appended after.
    pub(crate) after: Option<String>,

    pub(crate) children: Vec<crate::object::block::Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendBlockChildrenRequestBody {
    pub(crate) children: Vec<crate::object::block::Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) after: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub(crate) page_size: Option<u32>,
}

impl AppendBlockChildrenClient {
    // TODO: docs for send
    pub async fn send(
        self,
    ) -> Result<
        crate::list_response::ListResponse<crate::object::block::BlockResponse>,
        crate::error::Error,
    > {
        let block_id = self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
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

        let block = serde_json::from_slice::<
            crate::list_response::ListResponse<crate::object::block::BlockResponse>,
        >(&body)?;

        Ok(block)
    }
}
