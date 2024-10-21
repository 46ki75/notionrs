use notionrs::{block::Block, error::Error, NotionClient};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let client = NotionClient::new();

    // Here, we're retrieving the ID from an environment variable,
    // but you can change the method of retrieval to suit your needs.
    let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

    let request = client.get_block().block_id(block_id);

    let response = request.send().await?;

    let block = match response.block {
        Block::Audio { audio } => Block::Audio {
            audio: audio.url("https://example.com/foobar.wav"),
        },
        e => panic!("{:?}", e),
    };

    let request = client
        .update_block()
        .block_id(response.id.clone())
        .block(block);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
