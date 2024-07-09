pub mod page;
pub mod user;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use self::{
    page::get_page::GetPageClient,
    user::{get_self::GetSelfClient, get_user::GetUserClient, list_users::ListUsersClient},
};

#[derive(Default)]
pub struct NotionClient {
    reqwest_client: reqwest::Client,
}

impl NotionClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        dotenv().ok();

        let secret = env::var("NOTION_API_KEY").unwrap_or_else(|_| String::new());

        headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", secret)).expect("Invalid header value"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        NotionClient {
            reqwest_client: client,
        }
    }

    pub fn secret<T: AsRef<str>>(mut self, notion_api_key: T) -> Self {
        let mut headers = HeaderMap::new();
        let secret = notion_api_key.as_ref().to_string();

        headers.insert("Notion-Version", HeaderValue::from_static("2022-06-28"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", secret)).expect("Invalid header value"),
        );

        self.reqwest_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        self
    }

    // # --------------------------------------------------------------------------------
    //
    // User
    //
    // # --------------------------------------------------------------------------------

    /// <https://developers.notion.com/reference/get-users>
    ///
    /// This method can fetch a list of users present in the current Notion workspace.
    /// The user list includes humans (Notion accounts) and bots (Notion API integrations).
    ///
    /// ## Basic Usage
    /// It adopts the builder pattern, allowing you to add options and then execute
    /// the API call by invoking the `send()` method at the end.
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
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
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
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
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
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
    pub fn list_users(&self) -> ListUsersClient {
        ListUsersClient {
            reqwest_client: self.reqwest_client.clone(),
            start_cursor: None,
            page_size: None,
            recursive: false,
        }
    }

    /// <https://developers.notion.com/reference/get-user>
    ///
    /// Can be used to retrieve a specific user.
    ///
    /// ## Usage
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion
    ///         .get_user()
    ///         .user_id("c4e69ebe-3c42-4916-8ec4-285e5cb5bcb0")
    ///         .send()
    ///         .await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// ## Sample response
    ///
    /// ```json
    /// {
    ///     "object": "user",
    ///     "id": "c4e69ebe-3c42-4916-8ec4-285e5cb5bcb0",
    ///     "name": "default",
    ///     "avatar_url": null,
    ///     "type": "bot",
    ///     "bot": {
    ///         "owner": {
    ///             "type": "workspace",
    ///             "workspace": true
    ///         },
    ///         "workspace_name": "MyWorkspace"
    ///     },
    ///     "request_id": "1739014e-262a-4592-b2c3-9b76491a5ed1"
    /// }
    /// ```
    pub fn get_user(&self) -> GetUserClient {
        GetUserClient {
            reqwest_client: self.reqwest_client.clone(),
            user_id: None,
        }
    }

    /// <https://developers.notion.com/reference/get-self>
    ///
    /// Retrieves information about the user (bot) associated with the currently used token.
    ///
    /// ## Usage
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// use notionrs::error::NotionError;
    /// use notionrs::prelude::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let notion = NotionClient::new();
    ///     let result = notion.get_self().send().await?;
    ///     println!("{}", result.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    /// ## Sample response
    ///
    /// ```json
    /// {
    ///     "object": "user",
    ///     "id": "3571265d-852e-4aec-b529-75947e7842d6",
    ///     "name": "default",
    ///     "avatar_url": null,
    ///     "type": "bot",
    ///     "bot": {
    ///         "owner": {
    ///             "type": "workspace",
    ///             "workspace": true
    ///         },
    ///         "workspace_name": "MyWorkspace"
    ///     },
    ///     "request_id": "1739014e-262a-4592-b2c3-9b76491a5ed1"
    /// }
    /// ```
    pub fn get_self(&self) -> GetSelfClient {
        GetSelfClient {
            reqwest_client: self.reqwest_client.clone(),
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Page
    //
    // # --------------------------------------------------------------------------------

    /// <https://developers.notion.com/reference/retrieve-a-page>
    ///
    /// Specify the ID of the Page to retrieve a specific page.
    /// Only the information of the page will be retrieved, not the blocks.
    ///
    /// ## Usage
    /// ```no_run
    /// use notionrs::client;
    /// use notionrs::error::NotionError;
    /// use notionrs::to_json::ToJson;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), NotionError> {
    ///     let client = client::NotionClient::new();
    ///     let res = client
    ///         .get_page()
    ///         .page_id("ea9c82c5-9f21-4c58-bd0d-8473d5227906")
    ///         .send()
    ///         .await?;
    ///     println!("{}", res.to_json());
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Sample response
    /// ```json
    /// {
    ///     "id": "ea9c82c5-9f21-4c58-bd0d-8473d5227906",
    ///     "created_time": "2024-07-09T18:33:00.000Z",
    ///     "last_edited_time": "2024-07-09T18:33:00.000Z",
    ///     "created_by": {
    ///         "object": "user",
    ///         "id": "c4aca386-8965-4c0c-befe-4ae8699604b0",
    ///         "name": null,
    ///         "avatar_url": null,
    ///         "type": null,
    ///         "person": null
    ///     },
    ///     "last_edited_by": {
    ///         "object": "user",
    ///         "id": "c4aca386-8965-4c0c-befe-4ae8699604b0",
    ///         "name": null,
    ///         "avatar_url": null,
    ///         "type": null,
    ///         "person": null
    ///     },
    ///     "cover": null,
    ///     "icon": null,
    ///     "parent": {
    ///         "type": "database_id",
    ///         "database_id": "3d1cbceb-57d0-46b0-98c0-45cd49fbd4b0"
    ///     },
    ///     "archived": false,
    ///     "properties": {
    ///         "Tags": {
    ///             "type": "multi_select",
    ///             "id": "oydx",
    ///             "multi_select": [
    ///                 {
    ///                     "id": "4811585f-5229-4ee3-a03a-5cfa19a9eb3a",
    ///                     "name": "Test",
    ///                     "color": "default"
    ///                 }
    ///             ]
    ///         },
    ///         "Title": {
    ///             "type": "title",
    ///             "id": "title",
    ///             "title": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": {
    ///                         "content": "Page test",
    ///                         "link": null
    ///                     },
    ///                     "annotations": {
    ///                         "bold": false,
    ///                         "italic": false,
    ///                         "strikethrough": false,
    ///                         "underline": false,
    ///                         "code": false,
    ///                         "color": "default"
    ///                     },
    ///                     "plain_text": "Page test",
    ///                     "href": null
    ///                 }
    ///             ]
    ///         }
    ///     },
    ///     "url": "https://www.notion.so/Page-test-1f7f2976-4fec46d4bfcbd35d9225775a",
    ///     "public_url": null,
    ///     "developer_survey": null,
    ///     "request_id": "ac76f325-6b50-4663-92fc-741164f56156"
    /// }
    /// ```
    pub fn get_page(&self) -> GetPageClient {
        GetPageClient {
            reqwest_client: self.reqwest_client.clone(),
            page_id: None,
        }
    }
}
