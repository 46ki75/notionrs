use notionrs::{Client, Error};
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let mut properties = std::collections::HashMap::new();

    properties.insert(
        "Name".to_string(),
        PageProperty::Title(PageTitleProperty::from("New Page")),
    );

    let request = client
        .update_page()
        .page_id("PAGE_ID")
        .properties(properties);

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    Ok(())
}
