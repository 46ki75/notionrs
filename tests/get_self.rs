use notionrs::client;
use notionrs::error::NotionError;
use notionrs::to_json::ToJson;

/// This integration test cannot be run unless explicit permission
/// for user reading is granted in the Notion API key issuance settings.
#[tokio::test]
#[ignore]
async fn integration_test_get_self() -> Result<(), NotionError> {
    let client = client::NotionClient::new();
    let res = client.get_self().send().await?;
    println!("{}", res.to_json());

    Ok(())
}
