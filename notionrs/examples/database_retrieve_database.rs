use notionrs::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let request = client
        .retrieve_data_source()
        .data_source_id("DATA_SOURCE_ID");

    let _response = request.send().await?;

    Ok(())
}
