use notionrs::client::Client;
use notionrs_schema::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().secret("NOTION_TOKEN");

    let filter = Filter::timestamp_past_month();

    let sort = Sort::desc("Created Time");

    let request = client
        .query_database()
        .database_id("DATABASE_ID")
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
