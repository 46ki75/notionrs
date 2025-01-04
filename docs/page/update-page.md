# Update Page

This method is used to update page (properties, icon and cover) in Notion.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/patch-page).

## Basic Usage

```rs
use notionrs::{
    error::Error,
    page::{PageProperty, PageTitleProperty},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

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
```
