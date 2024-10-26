use notionrs::{block::Block, error::Error, Client, ToPlainText};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.get_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    if let Block::Paragraph { paragraph } = response.block {
        print!("{}", paragraph.rich_text.to_plain_text());
        Ok(())
    } else {
        Err(notionrs::error::Error::Custom(
            "Unexpected block type.".to_string(),
        ))
    }
}
