use notionrs::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let request = client.delete_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
