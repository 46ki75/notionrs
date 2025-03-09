# Update Block

This method is used to update a block in Notion.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/update-a-block).

## Basic Usage

```rs
use notionrs::{error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let rich_text = notionrs::RichText::from("rich text");

    let block = notionrs::object::block::Block::Paragraph {
        paragraph: notionrs::object::block::ParagraphBlock::default()
            .rich_text(vec![rich_text.clone()])
            .blue_background(),
    };

    let request = client.update_block().block_id("BLOCK_ID").block(block);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```
