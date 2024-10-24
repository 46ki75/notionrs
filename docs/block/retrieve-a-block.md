# Retrieve a block

This method is used to retrieve information about a block.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/retrieve-a-block).

## Basic Usage

To retrieve information about a block, you can use the following example:

```rs
use notionrs::{block::Block, error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.get_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    if let Block::Paragraph { paragraph } = response.block {
        print!(
            "{}",
            paragraph
                .rich_text
                .into_iter()
                .map(|t| t.to_string())
                .collect::<String>()
        );
        Ok(())
    } else {
        Err(notionrs::error::Error::Custom(
            "Unexpected block type.".to_string(),
        ))
    }
}
```

Since the block is returned as an `enum`, use either `match` or `if let` to specify the expected block type.
