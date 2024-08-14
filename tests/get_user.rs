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
/// NOTION_USER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
/// ```
#[tokio::test]
async fn integration_test_get_user() -> Result<(), NotionError> {
    dotenv().ok();
    let user_id = env::var("NOTION_USER_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client.get_user().user_id(user_id).send().await?;
    println!("{}", res.to_json());

    Ok(())
}
