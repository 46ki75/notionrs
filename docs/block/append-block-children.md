# Append Block Children

This method is used to create and append new children blocks to the parent block.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/patch-block-children).

## Basic Usage

```rs
use notionrs::{error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let rich_text = notionrs::object::rich_text::RichText::from("rich text");

    let block = notionrs::object::block::Block::Paragraph {
        paragraph: notionrs::object::block::ParagraphBlock::default()
            .rich_text(vec![rich_text.clone()])
            .blue_background(),
    };

    let request = client
        .append_block_children()
        .block_id("PARENT_BLOCK_ID")
        .children(vec![block]);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```
