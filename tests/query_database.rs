use notionrs::client;
use notionrs::error::NotionError;
use notionrs::filter::Filter;
use notionrs::page::properties::title::PageTitleProperty;
use notionrs::page::properties::PageProperty;
use notionrs::to_json::ToJson;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[tokio::test]
#[ignore]
async fn integration_test_query_database() -> Result<(), NotionError> {
    dotenv().ok();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
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
#[ignore]
async fn integration_test_query_database_with_struct() -> Result<(), NotionError> {
    dotenv().ok();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
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
#[ignore]
async fn integration_test_query_database_page_size() -> Result<(), NotionError> {
    dotenv().ok();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .page_size(1)
        .send::<HashMap<String, PageProperty>>()
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
#[ignore]
async fn integration_test_query_database_recursive() -> Result<(), NotionError> {
    dotenv().ok();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .recursive()
        .send::<HashMap<String, PageProperty>>()
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
#[ignore]
async fn integration_test_query_database_filter() -> Result<(), NotionError> {
    dotenv().ok();
    let database_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| String::new());

    let client = client::NotionClient::new();
    let res = client
        .query_database()
        .database_id(database_id)
        .filter(Filter::or(vec![Filter::date_before(
            "CreatedAt",
            "2024-07-01",
        )]))
        .send::<HashMap<String, PageProperty>>()
        .await?;
    println!("{}", res.to_json());

    Ok(())
}
