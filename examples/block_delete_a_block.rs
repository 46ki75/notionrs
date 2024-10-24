use notionrs::{error::Error, Client};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("API_KEY");

    let request = client.delete_block().block_id("BLOCK_ID");

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
