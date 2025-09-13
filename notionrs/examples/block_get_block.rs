use notionrs::client::Client;
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let request = client.get_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    if let Block::Paragraph { paragraph } = response.block {
        let text = paragraph.to_string();
        print!("{}", text);
        Ok(())
    } else {
        Err("Unexpected block type.".into())
    }
}
