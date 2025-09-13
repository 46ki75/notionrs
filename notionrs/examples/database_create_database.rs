use std::collections::HashMap;

use notionrs::Error;
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = notionrs::Client::new(notion_api_key);

    let title = vec![RichText::from("Database Title")];

    let mut properties = HashMap::new();

    properties.insert(
        "email".to_string(),
        DataSourceProperty::Email(DataSourceEmailProperty::default()),
    );

    let request = client.create_database().page_id("PAGE_ID").title(title);

    let _response = request.send().await?;

    Ok(())
}
