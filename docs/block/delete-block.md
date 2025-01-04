# Delete Block

This method is used to delete (archive) a block in Notion.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/delete-a-block).

## Basic Usage

To delete (archive) a block, you can use the following example:

```rs
use notionrs::{error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.delete_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```

`API_KEY` and `BLOCK_ID` are placeholders.
