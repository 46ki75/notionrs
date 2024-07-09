use notionrs::client;
use notionrs::error::NotionError;
use notionrs::to_json::ToJson;

use dotenv::dotenv;
use std::env;

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
    let res = client.get_page().page_id(page_id).send().await?;
    println!("{}", res.to_json());

    Ok(())
}
