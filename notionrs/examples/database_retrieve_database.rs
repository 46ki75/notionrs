use notionrs::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().secret("NOTION_TOKEN");

    let request = client
        .retrieve_data_source()
        .data_source_id("DATA_SOURCE_ID");

    let _response = request.send().await?;

    Ok(())
}
