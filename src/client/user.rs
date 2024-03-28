use serde::Serialize;

use crate::{
    error::{NotionApiError, NotionError},
    notion_response::NotionResponse,
    user::User,
};

pub struct ListUserClient {
    pub(crate) reqwest_client: reqwest::Client,
    pub(crate) start_cursor: Option<String>,
    pub(crate) page_size: Option<u8>,
    pub(crate) recursive: bool,
}

#[derive(Serialize)]
struct LinsUserQueryParams {
    start_cursor: Option<String>,
    page_size: Option<u8>,
}

impl ListUserClient {
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
    ///     let results = notion.list_user().send().await?;
    ///     println!("{}", results.to_json());
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
    ///     let results = notion.list_user().recursive().send().await?;
    ///     println!("{}", results.to_json());
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
    ///     let results = notion.list_user().recursive().send().await?;
    ///     println!("{}", results.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Response Sample
    /// ```json
    /// [
    ///   {
    ///     "object": "user",
    ///     "id": "8b6c553a-ab6d-462e-8ae1-689487438822",
    ///     "name": "46ki75",
    ///     "avatar_url": "https://lh3.googleusercontent.com/a/XXXXXXXXXX",
    ///     "type": "person",
    ///     "person": {
    ///       "email": "user@example.com"
    ///     }
    ///   },
    ///   {
    ///     "object": "user",
    ///     "id": "57a16579-8773-4069-b448-ac3cc48cc9a3",
    ///     "name": "spec",
    ///     "avatar_url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/XXXXXXXXXX",
    ///     "type": "bot",
    ///     "bot": {
    ///       "owner": {
    ///         "type": "workspace",
    ///         "workspace": true
    ///       },
    ///       "workspace_name": "MyWorkspace"
    ///     }
    ///   },
    ///   {
    ///     "object": "user",
    ///     "id": "9fa18978-20fa-4291-a22d-44e1ece1fd3c",
    ///     "name": "stg",
    ///     "avatar_url": null,
    ///     "type": "bot",
    ///     "bot": {}
    ///   }
    /// ]
    /// ```
    ///
    pub async fn send(&mut self) -> Result<Vec<User>, NotionError> {
        let mut results = Vec::new();
        let url = "https://api.notion.com/v1/users";

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

                let body = res.json::<NotionResponse<User>>().await?;

                results.extend(body.results.into_iter());

                match body.next_cursor {
                    Some(next_cursor) => self.start_cursor = Some(next_cursor),
                    None => break,
                }
            }
            Ok(results)
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

            let body = res.json::<NotionResponse<User>>().await?;

            results.extend(body.results.into_iter());

            Ok(results)
        }
    }

    pub fn start_cursor<T: AsRef<str>>(mut self, start_cursor: T) -> Self {
        self.start_cursor = Some(start_cursor.as_ref().to_string());
        self
    }

    pub fn page_size(mut self, page_size: u8) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
}
