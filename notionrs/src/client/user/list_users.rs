use serde::Serialize;

use crate::{list_response::ListResponse, user::User};

/// A request builder for performing `list_users` operations.

#[derive(Debug)]
pub struct ListUsersClient {
    /// The reqwest http client
    pub(crate) reqwest_client: reqwest::Client,

    /// The starting cursor position for pagination
    pub(crate) start_cursor: Option<String>,

    /// The number of items to fetch at once. Defaults to 100 if not specified.
    /// Minimum 1, maximum 100
    pub(crate) page_size: Option<u8>,

    /// Whether to fetch data recursively.
    /// If `true`, fetches all data without pagination.
    /// If `false`, performs pagination.
    pub(crate) recursive: bool,
}

/// Query parameters for making requests to the endpoint.
#[derive(Serialize)]
struct LinsUserQueryParams {
    /// If supplied, this endpoint will return a page of results starting after the cursor provided.
    /// If not supplied, this endpoint will return the first page of results.
    start_cursor: Option<String>,

    /// The number of items from the full list desired in the response. Maximum: 100
    page_size: Option<u8>,
}

impl ListUsersClient {
    /// Send a request to the API endpoint of Notion.
    pub async fn send(&mut self) -> Result<ListResponse<User>, crate::error::Error> {
        let url = "https://api.notion.com/v1/users";
        let mut results = Vec::new();

        if self.recursive {
            loop {
                let params = LinsUserQueryParams {
                    start_cursor: self.start_cursor.clone(),
                    page_size: Some(100),
                };

                let request = self.reqwest_client.get(url).query(&params);

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

                let users_response = serde_json::from_slice::<ListResponse<User>>(&body)?;

                results.extend(users_response.results);

                match users_response.next_cursor {
                    Some(next_cursor) => {
                        self.start_cursor = Some(next_cursor.clone());
                    }
                    None => break,
                }
            }

            Ok(ListResponse {
                object: "list".to_string(),
                r#type: Some("user".to_string()),
                results,
                next_cursor: None,
                has_more: Some(false),
            })
        } else {
            let params = LinsUserQueryParams {
                start_cursor: self.start_cursor.clone(),
                page_size: self.page_size,
            };

            let request = self.reqwest_client.get(url).query(&params);

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

            let users = serde_json::from_slice::<ListResponse<User>>(&body)?;

            Ok(users)
        }
    }

    /// Performs cursor-based pagination when data cannot be fetched in one go.
    pub fn start_cursor<T: AsRef<str>>(mut self, start_cursor: T) -> Self {
        self.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }

    /// Can be used to limit the number of items fetched. Valid range for the number is 1~100.
    pub fn page_size(mut self, page_size: u8) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Use this for recursive data fetching. By default, pagination only fetches up to 100 records,
    /// but this allows fetching all records.
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
}
