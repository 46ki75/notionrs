use notionrs::to_json::ToJson;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn integration_test_query_database() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
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
async fn integration_test_query_database_with_struct() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .send::<MyResponse>()
        .await?;
    println!("{:?}", res);

    Ok(())
}

// # --------------------------------------------------------------------------------
//
// working with page_size
//
// # --------------------------------------------------------------------------------

#[tokio::test]
async fn integration_test_query_database_page_size() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .page_size(1)
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
        .await?;
    println!("{}", res.to_json());

    Ok(())
}

// # --------------------------------------------------------------------------------
//
// working with recursive
//
// # --------------------------------------------------------------------------------

#[tokio::test]
async fn integration_test_query_database_recursive() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .recursive()
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
        .await?;
    println!("{}", res.to_json());

    Ok(())
}

// # --------------------------------------------------------------------------------
//
// working with Filter
//
// # --------------------------------------------------------------------------------

#[tokio::test]
async fn integration_test_query_database_filter_1() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();

    let filter = notionrs::filter::Filter::or(vec![
        notionrs::filter::Filter::date_after("Created time", "2024-07-01"),
        notionrs::filter::Filter::date_before("Created time", "2024-07-01"),
        notionrs::filter::Filter::date_equals("Created time", "2024-07-01"),
        notionrs::filter::Filter::date_is_empty("Created time"),
        notionrs::filter::Filter::date_is_not_empty("Created time"),
        notionrs::filter::Filter::date_next_month("Created time"),
        notionrs::filter::Filter::date_next_week("Created time"),
        notionrs::filter::Filter::date_next_year("Created time"),
        notionrs::filter::Filter::date_on_or_after("Created time", "2024-07-01"),
        notionrs::filter::Filter::date_on_or_before("Created time", "2024-07-01"),
        notionrs::filter::Filter::date_past_month("Created time"),
        notionrs::filter::Filter::date_past_week("Created time"),
        notionrs::filter::Filter::date_past_year("Created time"),
        notionrs::filter::Filter::date_this_week("Created time"),
    ]);

    let request = client
        .query_database()
        .database_id(database_id)
        .filter(filter);

    let response = request
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
        .await?;

    println!("{}", response.to_json());

    Ok(())
}

#[tokio::test]
async fn integration_test_query_database_filter_2() -> Result<(), notionrs::error::NotionError> {
    dotenv().ok();
    let database_id = std::env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = notionrs::client::NotionClient::new();

    let filter = notionrs::filter::Filter::date_before("Created time", "2024-07-01");

    let request = client
        .query_database()
        .database_id(database_id)
        .filter(filter);

    let response = request
        .send::<std::collections::HashMap<String, notionrs::page::properties::PageProperty>>()
        .await?;

    println!("{}", response.to_json());

    Ok(())
}
