use serde::{Deserialize, Serialize};

/// Block positioning for the `appendBlockChildren` API (2026-03-11+).
///
/// Replaces the deprecated `after` parameter.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AppendBlockChildrenPosition {
    /// Insert after a specific block.
    AfterBlock {
        after_block: AppendBlockChildrenAfterBlock,
    },
    /// Insert at the start of the parent block.
    Start,
    /// Insert at the end of the parent block (default).
    End,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppendBlockChildrenAfterBlock {
    pub id: String,
}

#[derive(Debug, Default, notionrs_macro::Setter)]
pub struct AppendBlockChildrenClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// Identifier for a block. Also accepts a page ID.
    pub(crate) block_id: Option<String>,

    /// The ID of the existing block that the new block should be appended after.
    ///
    /// **Deprecated**: Use `position` instead.
    #[skip]
    pub(crate) after: Option<String>,

    /// Block insertion position (2026-03-11+). Replaces the deprecated `after` field.
    #[skip]
    pub(crate) position: Option<AppendBlockChildrenPosition>,

    pub(crate) children: Vec<notionrs_types::object::block::Block>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppendBlockChildrenRequestBody {
    pub(crate) children: Vec<notionrs_types::object::block::Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use `position` instead.")]
    pub(crate) after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) position: Option<AppendBlockChildrenPosition>,
}

impl AppendBlockChildrenClient {
    /// Set the ID of an existing block that the new blocks should be appended after.
    ///
    /// **Deprecated**: Use [`position_after_block`](Self::position_after_block),
    /// [`position_start`](Self::position_start), or [`position_end`](Self::position_end) instead.
    #[deprecated(note = "Use `position_after_block`, `position_start`, or `position_end` instead.")]
    pub fn after<T: AsRef<str>>(mut self, after: T) -> Self {
        self.after = Some(after.as_ref().to_string());
        self.position = None;
        self
    }

    /// Insert the new blocks after the block with the given ID.
    pub fn position_after_block<T: AsRef<str>>(mut self, block_id: T) -> Self {
        self.position = Some(AppendBlockChildrenPosition::AfterBlock {
            after_block: AppendBlockChildrenAfterBlock {
                id: block_id.as_ref().to_string(),
            },
        });
        self.after = None;
        self
    }

    /// Insert the new blocks at the start of the parent block.
    pub fn position_start(mut self) -> Self {
        self.position = Some(AppendBlockChildrenPosition::Start);
        self.after = None;
        self
    }

    /// Insert the new blocks at the end of the parent block (default behavior).
    pub fn position_end(mut self) -> Self {
        self.position = Some(AppendBlockChildrenPosition::End);
        self.after = None;
        self
    }

    // TODO: docs for send
    pub async fn send(
        self,
    ) -> Result<
        notionrs_types::object::response::ListResponse<
            notionrs_types::object::block::BlockResponse,
        >,
        crate::error::Error,
    > {
        let block_id = self.block_id.ok_or(crate::error::Error::RequestParameter(
            "`block_id` is not set.".to_string(),
        ))?;

        if self.after.is_some() && self.position.is_some() {
            return Err(crate::error::Error::RequestParameter(
                "You cannot specify both `after` and `position` in the same request.".to_string(),
            ));
        }

        #[allow(deprecated)]
        let request_body_struct = AppendBlockChildrenRequestBody {
            children: self.children,
            after: self.after,
            position: self.position,
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
            notionrs_types::object::response::ListResponse<
                notionrs_types::object::block::BlockResponse,
            >,
        >(&body)?;

        Ok(block)
    }
}
