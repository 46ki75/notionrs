# Audio Block

> [!INFO]
> At the time of writing, there is no mention of the Audio Block in the official Notion API [documentation](https://developers.notion.com/reference/block). However, its existence can be confirmed in the [codebase](https://github.com/makenotion/notion-sdk-js/blob/main/src/api-endpoints.ts) of the Notion SDK for NodeJS.

## Create Audio Block

To create an Audio Block, follow these steps:

```rust
use notionrs::{block::Block, error::Error, File};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let client = notionrs::client::NotionClient::new();

    // Here, we're retrieving the ID from an environment variable,
    // but you can change the method of retrieval to suit your needs.
    let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

    let block = Block::Audio {
        audio: File::new()
            .url("https://example.com/sample.wav")
            .caption(vec![notionrs::RichText::from("enter your caption")]),
    };

    let request = client
        .append_block_children()
        .block_id(block_id.clone())
        .children(vec![block]);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```

> [!WARNING]
> The Notion API **DOES NOT** provide functionality for **file uploads**; it only allows resources to be created as references to external links.

## Update Audio Block

Below is an example of sending a request to update an Audio Block by modifying some of its parameters after retrieving it.

```rs
use notionrs::{block::Block, error::Error, NotionClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let client = NotionClient::new();

    // Here, we're retrieving the ID from an environment variable,
    // but you can change the method of retrieval to suit your needs.
    let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

    let request = client.get_block().block_id(block_id);

    let response = request.send().await?;

    let block = match response.block {
        Block::Audio { audio } => Block::Audio {
            audio: audio.url("https://example.com/foobar.wav"),
        },
        e => panic!("{:?}", e),
    };

    let request = client
        .update_block()
        .block_id(response.id.clone())
        .block(block);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```
