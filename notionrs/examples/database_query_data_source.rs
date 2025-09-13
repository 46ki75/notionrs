use notionrs::Client;
use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let filter = Filter::timestamp_past_month();

    let sort = Sort::desc("Created Time");

    let request = client
        .query_data_source()
        .data_source_id("DATA_SOURCE_ID")
        .filter(filter)
        .sorts(vec![sort]);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MyProperties {
        #[serde(rename = "My Title")]
        pub title: PageTitleProperty,
    }

    let response = request.send::<MyProperties>().await?;

    for page in response.results {
        println!("{}", page.properties.title.to_string());
    }

    Ok(())
}
