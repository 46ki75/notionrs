# Get Page

This method is used to retrieve a page in Notion.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/retrieve-a-page).

## Basic Usage

```rs
use notionrs::{error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.create_page().page_id("PAGE_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    let properties = response.properties;

    let title = properties
        .get("Name")
        .ok_or(Error::Custom("`Name` property not found".to_string()))?;

    println!("Title: {}", title);

    Ok(())
}
```