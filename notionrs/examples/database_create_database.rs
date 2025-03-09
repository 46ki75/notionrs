use std::collections::HashMap;

use notionrs::{
    Client, RichText,
    error::Error,
    object::database::{DatabaseEmailProperty, DatabaseProperty},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("NOTION_TOKEN");

    let title = vec![RichText::from("Database Title")];

    let mut properties = HashMap::new();

    properties.insert(
        "email".to_string(),
        DatabaseProperty::Email(DatabaseEmailProperty::default()),
    );

    let request = client.create_database().page_id("PAGE_ID").title(title);

    let _response = request.send().await?;

    Ok(())
}
