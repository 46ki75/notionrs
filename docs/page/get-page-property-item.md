# Get Page Property Item

This method is used to retrieve a page in Notion.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/retrieve-a-page-property).

## Basic Usage

```rs
use notionrs::{error::Error, page::PageProperty, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client
        .get_page_property_item()
        .page_id("PAGE_ID")
        .property_id("PROPERTY_ID");

    let response = request.send().await?;

    if let PageProperty::Title(title_property) = response {
        let title = title_property.to_string();
        println!("Title: {}", title);
    } else {
        return Err(Error::Custom("Property is not a title".to_string()));
    }

    Ok(())
}
```
