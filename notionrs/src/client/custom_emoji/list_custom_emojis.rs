use serde::Serialize;

use notionrs_types::prelude::*;

/// A request builder for performing `list_custom_emojis` operations.
#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ListCustomEmojisClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The starting cursor position for pagination
    pub(crate) start_cursor: Option<String>,

    /// The number of items to fetch at once. Defaults to 100 if not specified.
    /// Minimum 1, maximum 100
    pub(crate) page_size: Option<u8>,

    /// Filter custom emojis by exact name match.
    pub(crate) name: Option<String>,
}

/// Query parameters for making requests to the endpoint.
#[derive(Serialize, Debug, Clone)]
struct ListCustomEmojisQueryParams {
    /// If supplied, this endpoint will return a page of results starting after the cursor provided.
    /// If not supplied, this endpoint will return the first page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,

    /// The number of items from the full list desired in the response. Maximum: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,

    /// If supplied, filters custom emojis by exact name match.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

crate::impl_paginate!(ListCustomEmojisClient, CustomEmojiContent);

impl ListCustomEmojisClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<ListResponse<CustomEmojiContent>, crate::error::Error> {
        let url = "https://api.notion.com/v1/custom_emojis";

        let params = ListCustomEmojisQueryParams {
            start_cursor: self.start_cursor.clone(),
            page_size: self.page_size,
            name: self.name.clone(),
        };

        let request = self.reqwest_client.get(url).query(&params);

        let response: ListResponse<CustomEmojiContent> =
            crate::util::send_and_convert(request).await?;

        Ok(response)
    }
}
