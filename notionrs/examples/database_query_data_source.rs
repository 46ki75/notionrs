use notionrs::Client;
use notionrs_types::prelude::*;

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

    let response = request.send().await?;

    for page in response.results {
        let title_property = page
            .properties
            .get("Name")
            .ok_or("Property not found".to_string())?;

        if let PageProperty::Title(title) = title_property {
            println!("Title: {}", title);
        }
    }

    Ok(())
}
