use notionrs::{block::Block, error::Error, Client, File};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let client = Client::new();

    // Here, we're retrieving the ID from an environment variable,
    // but you can change the method of retrieval to suit your needs.
    let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

    let block = Block::Audio {
        audio: File::default()
            .url("https://example.com/sample.wav")
            .caption(vec![notionrs::RichText::from("my caption")]),
    };

    let request = client
        .append_block_children()
        .block_id(block_id.clone())
        .children(vec![block]);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
