use notionrs::client::Client;
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let request = client
        .get_page_property_item()
        .page_id("PAGE_ID")
        .property_id("PROPERTY_ID");

    let response = request.send().await?;

    if let PageProperty::Title(title_property) = response {
        let title = title_property.to_string();
        println!("Title: {}", title);
    } else {
        return Err("Property is not a title".into());
    }

    Ok(())
}
