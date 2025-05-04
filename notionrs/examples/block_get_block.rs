use notionrs::client::Client;
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().secret("API_KEY");

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
