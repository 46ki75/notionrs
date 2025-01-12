use notionrs::{database::Sort, error::Error, filter::Filter, page::PageProperty, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
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
            .ok_or(Error::Custom("Property not found".to_string()))?;

        if let PageProperty::Title(title) = title_property {
            println!("Title: {}", title);
        }
    }

    Ok(())
}
