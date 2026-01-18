use futures::TryStreamExt;
use notionrs::{Client, PaginateExt};
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let filter = Filter::timestamp_past_month();

    let sort = Sort::desc("Created Time");

    let pages = client
        .query_data_source()
        .data_source_id("DATA_SOURCE_ID")
        .filter(filter)
        .sorts(vec![sort])
        .into_stream()
        .try_collect::<Vec<PageResponse>>()
        .await
        .unwrap();

    for page in pages {
        println!("{:#?}", page.properties);
    }

    Ok(())
}
