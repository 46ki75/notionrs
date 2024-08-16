use notionrs::to_json::ToJson;

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
    dotenv::dotenv().ok();

    let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();

    let request = client.get_page().page_id(page_id);

    let response = request
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
        .await?;

    let id_property = response.properties.get("ID").unwrap();

    let unique_id = match id_property {
        notionrs::page::properties::PageProperty::UniqueId(i) => i.unique_id.number,
        _ => todo!(),
    };

    println!("ID is {}", unique_id);

    println!("{}", response.to_json());

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
    dotenv::dotenv().ok();

    let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();

    let request = client.get_page().page_id(page_id);

    let response = request.send::<MyResponse>().await?;

    println!("{:?}", response.properties.title);

    Ok(())
}
