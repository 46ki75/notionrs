mod client;
mod error;
mod notion_response;
mod user;

use error::NotionError;

use serde::Serialize;

use crate::client::NotionClient;

pub trait ToJson {
    fn to_json(&self) -> String;
}

impl<T> ToJson for T
where
    T: Serialize,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<(), NotionError> {
    let notion = NotionClient::new();
    let results = notion.list_user().send().await?;
    println!("{}", results.to_json());

    Ok(())
}
