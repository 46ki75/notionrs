use notionrs::{block::Block, error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.get_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    if let Block::Paragraph { paragraph } = response.block {
        let text = paragraph.to_string();
        print!("{}", text);
        Ok(())
    } else {
        Err(notionrs::error::Error::Custom(
            "Unexpected block type.".to_string(),
        ))
    }
}
