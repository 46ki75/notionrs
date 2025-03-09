use notionrs::{client::Client, error::Error, object::rich_text::RichText};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let rich_text = RichText::from("rich text");

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
