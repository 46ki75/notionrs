use serde::Serialize;

use notionrs_types::prelude::*;

/// A request builder for performing `list_users` operations.

#[derive(Debug, Default, Clone, notionrs_macro::Setter)]
pub struct ListUsersClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The starting cursor position for pagination
    pub(crate) start_cursor: Option<String>,

    /// The number of items to fetch at once. Defaults to 100 if not specified.
    /// Minimum 1, maximum 100
    pub(crate) page_size: Option<u8>,
}

/// Query parameters for making requests to the endpoint.
#[derive(Serialize, Debug, Clone)]
struct ListUserQueryParams {
    /// If supplied, this endpoint will return a page of results starting after the cursor provided.
    /// If not supplied, this endpoint will return the first page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,

    /// The number of items from the full list desired in the response. Maximum: 100
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u8>,
}

crate::impl_paginate!(ListUsersClient, User);

impl ListUsersClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(self) -> Result<ListResponse<User>, crate::error::Error> {
        let url = "https://api.notion.com/v1/users";

        let params = ListUserQueryParams {
            start_cursor: self.start_cursor.clone(),
            page_size: self.page_size,
        };

        let request = self.reqwest_client.get(url).query(&params);

        let response: ListResponse<User> = crate::util::send_and_convert(request).await?;

        Ok(response)
    }
}
