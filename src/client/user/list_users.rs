use serde::Serialize;

use crate::{
    error::{NotionApiError, NotionError},
    notion_response::NotionListResponse,
    user::User,
};

/// A request builder for performing `list_users` operations.
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
    /// https://developers.notion.com/reference/get-users
    ///
    /// This method can fetch a list of users present in the current Notion workspace.
    /// The user list includes humans (Notion accounts) and bots (Notion API integrations).
    ///
    /// ## Basic Usage
    /// It adopts the builder pattern, allowing you to add options and then execute
    /// the API call by invoking the `send()` method at the end.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion.list_users().send().await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ### Recursive Fetching
    /// By default, only up to 100 users can be fetched. To fetch all users
    /// without manual pagination, use the `recursive()` method for recursive fetching.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion.list_users().recursive().send().await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ### Limiting Fetch Amount
    /// Though it might not be very useful, the `page_size()` method can limit the number of fetched users.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion.list_users().recursive().send().await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Response Sample
    /// ```json
    /// {
    ///   "object": "list",
    ///   "results": [
    ///     {
    ///       "object": "user",
    ///       "id": "8b6c553a-ab6d-462e-8ae1-689487438822",
    ///       "name": "46ki75",
    ///       "avatar_url": "https://lh3.googleusercontent.com/a/XXXXXXXXXX",
    ///       "type": "person",
    ///       "person": {
    ///         "email": "user@example.com"
    ///       }
    ///     },
    ///     {
    ///       "object": "user",
    ///       "id": "57a16579-8773-4069-b448-ac3cc48cc9a3",
    ///       "name": "spec",
    ///       "avatar_url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/XXXXXXXXXX",
    ///       "type": "bot",
    ///       "bot": {
    ///         "owner": {
    ///           "type": "workspace",
    ///           "workspace": true
    ///         },
    ///         "workspace_name": "MyWorkspace"
    ///       }
    ///     },
    ///     {
    ///       "object": "user",
    ///       "id": "9fa18978-20fa-4291-a22d-44e1ece1fd3c",
    ///       "name": "stg",
    ///       "avatar_url": null,
    ///       "type": "bot",
    ///       "bot": {}
    ///     }
    ///   ],
    ///   "next_cursor": null,
    ///   "has_more": false,
    ///   "type": "user",
    ///   "developer_survey": null,
    ///   "request_id": null
    /// }
    /// ```
    pub async fn send(&mut self) -> Result<NotionListResponse<User>, NotionError> {
        let url = "https://api.notion.com/v1/users";
        let mut results = Vec::new();

        if self.recursive {
            loop {
                let params = LinsUserQueryParams {
                    start_cursor: self.start_cursor.clone(),
                    page_size: Some(100),
                };

                let res = self.reqwest_client.get(url).query(&params).send().await?;

                if !res.status().is_success() {
                    let api_error = res.json::<NotionApiError>().await?;
                    return Err(NotionError::NotionApiError(Box::new(api_error)));
                }

                let body = res.json::<NotionListResponse<User>>().await?;

                results.extend(body.results);

                match body.next_cursor {
                    Some(next_cursor) => {
                        self.start_cursor = Some(next_cursor.clone());
                    }
                    None => break,
                }
            }

            Ok(NotionListResponse {
                object: "list".to_string(),
                r#type: Some("user".to_string()),
                results,
                next_cursor: None,
                has_more: Some(false),
                developer_survey: None,
                request_id: None,
            })
        } else {
            let params = LinsUserQueryParams {
                start_cursor: self.start_cursor.clone(),
                page_size: self.page_size,
            };

            let res = self.reqwest_client.get(url).query(&params).send().await?;

            if !res.status().is_success() {
                let api_error = res.json::<NotionApiError>().await?;
                return Err(NotionError::NotionApiError(Box::new(api_error)));
            }

            let body = res.json::<NotionListResponse<User>>().await?;

            Ok(body)
        }
    }

    /// Performs cursor-based pagination when data cannot be fetched in one go.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion
    ///         .list_users()
    ///         .start_cursor("4f5ceec2-c402-41f3-9fb0-6789f526e4b5")
    ///         .send()
    ///         .await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn start_cursor<T: AsRef<str>>(mut self, start_cursor: T) -> Self {
        self.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }

    /// Can be used to limit the number of items fetched. Valid range for the number is 1~100.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion.list_users().page_size(10).send().await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    ///```
    pub fn page_size(mut self, page_size: u8) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Use this for recursive data fetching. By default, pagination only fetches up to 100 records,
    /// but this allows fetching all records.
    /// ```rs
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let response = notion.list_users().recursive().send().await?;
    ///     println!("{}", response.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
}
