use notionrs::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().secret("API_KEY");

    let request = client.create_page().page_id("PAGE_ID");

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    let properties = response.properties;

    let title = properties
        .get("Name")
        .ok_or("`Name` property not found".to_string())?;

    println!("Title: {}", title);

    Ok(())
}
