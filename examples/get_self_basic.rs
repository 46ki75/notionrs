use notionrs::to_json::ToJson; // When you need to use the `.to_json()` method

#[tokio::main]
async fn main() -> Result<(), notionrs::error::NotionError> {
    let client = notionrs::client::NotionClient::new();

    let request = client.get_self();

    let response = request.send().await?;

    println!("{}", response.to_json());

    Ok(())
}
