pub mod database;
pub mod page;
pub mod user;

use database::query_database::{QueryDatabaseClient, QueryDatabaseRequestBody};
use page::get_page_property_item::GetPagePropertyItemClient;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use self::{
    page::get_page::GetPageClient,
    user::{get_self::GetSelfClient, get_user::GetUserClient, list_users::ListUsersClient},
};

#[derive(Default, Debug)]
pub struct NotionClient {
    reqwest_client: reqwest::Client,
}

impl NotionClient {
    // TODO: docs: new method
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();

        dotenvy::dotenv().ok();

        let secret = env::var("NOTION_TOKEN").unwrap_or_else(|_| String::new());

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

    /// This method sets the token used for calling the Notion API.
    /// If you don't set it, the client will automatically read
    /// and use the environment variable named `NOTION_TOKEN` during initialization.
    ///
    /// For details on obtaining a Notion token, please refer to the
    /// [Notion Developer Documentation](https://developers.notion.com/docs/authorization).
    ///
    /// ```no_run
    /// use notionrs::client::NotionClient;
    /// // ...
    /// let client = NotionClient::new().secret("secret_XXXXXXXXXXXXXX");
    /// ```
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

    // TODO: docs: list_users method
    pub fn list_users(&self) -> ListUsersClient {
        ListUsersClient {
            reqwest_client: self.reqwest_client.clone(),
            start_cursor: None,
            page_size: None,
            recursive: false,
        }
    }

    // TODO: docs: get_user method
    pub fn get_user(&self) -> GetUserClient {
        GetUserClient {
            reqwest_client: self.reqwest_client.clone(),
            user_id: None,
        }
    }

    // TODO: docs: get_self method
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

    // TODO: docs: get_page method
    pub fn get_page(&self) -> GetPageClient {
        GetPageClient {
            reqwest_client: self.reqwest_client.clone(),
            page_id: None,
        }
    }

    // TODO: docs: get_page_property_item method
    pub fn get_page_property_item(&self) -> GetPagePropertyItemClient {
        GetPagePropertyItemClient {
            reqwest_client: self.reqwest_client.clone(),
            page_id: None,
            property_id: None,
        }
    }

    // # --------------------------------------------------------------------------------
    //
    // Database
    //
    // # --------------------------------------------------------------------------------

    // TODO: docs
    pub fn query_database(&self) -> QueryDatabaseClient {
        QueryDatabaseClient {
            reqwest_client: self.reqwest_client.clone(),
            database_id: None,
            body: QueryDatabaseRequestBody {
                filter: None,
                start_cursor: None,
                page_size: None,
            },
            recursive: false,
        }
    }
}
