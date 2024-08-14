use notionrs::error::NotionError;
use notionrs::page::properties::PageTitleProperty;
use notionrs::to_json::ToJson;
use notionrs::{client, page::properties::PageProperty};

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

/// This integration test cannot be run unless explicit permission
/// for user reading is granted in the Notion API key issuance settings.
///
/// To conduct integration testing, write the following in the `.env` file.
/// ```ini
/// NOTION_PAGE_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
#[tokio::test]
#[ignore]
async fn integration_test_get_page() -> Result<(), NotionError> {
    dotenv().ok();
    let page_id = env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .get_page()
        .page_id(page_id)
        .send::<HashMap<String, PageProperty>>()
        .await?;
    println!("{}", res.to_json());

    Ok(())
}

// # --------------------------------------------------------------------------------
//
// working with struct
//
// # --------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct MyResponse {
    #[serde(rename = "Title")]
    title: PageTitleProperty,
}

#[tokio::test]
async fn integration_test_get_page_with_struct() -> Result<(), NotionError> {
    dotenv().ok();
    let page_id = env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .get_page()
        .page_id(page_id)
        .send::<MyResponse>()
        .await?;
    println!("{:?}", res.properties.title);

    Ok(())
}
