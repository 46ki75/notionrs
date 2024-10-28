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
                .iter()
                .map(|rt| rt.to_string())
                .collect::<String>()
        );
        Ok(())
    } else {
        Err(notionrs::error::Error::Custom(
            "Unexpected block type.".to_string(),
        ))
    }
}
