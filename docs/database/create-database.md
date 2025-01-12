# Create Database

This method is used to create database.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/create-a-database).

## Basic Usage 

```rs
use std::collections::HashMap;

use notionrs::{
    database::{DatabaseEmailProperty, DatabaseProperty},
    error::Error,
    Client, RichText,
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
```
