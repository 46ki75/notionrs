use notionrs::to_json::ToJson;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

/// This integration test cannot be run unless explicit permission
/// for user reading is granted in the Notion API key issuance settings.
///
/// To conduct integration testing, write the following in the `.env` file.
/// ```ini
/// NOTION_PAGE_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
#[tokio::test]
async fn integration_test_get_page() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .get_page()
        .page_id(page_id)
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
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
    title: notionrs::page::properties::title::PageTitleProperty,
}

#[tokio::test]
async fn integration_test_get_page_with_struct() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .get_page()
        .page_id(page_id)
        .send::<MyResponse>()
        .await?;
    println!("{:?}", res.properties.title);

    Ok(())
}
