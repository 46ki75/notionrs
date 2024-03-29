pub mod user;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use self::user::list_user::ListUserClient;

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

    pub fn list_user(&self) -> ListUserClient {
        ListUserClient {
            reqwest_client: self.reqwest_client.clone(),
            start_cursor: None,
            page_size: None,
            recursive: false,
        }
    }
}
