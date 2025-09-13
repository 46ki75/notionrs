use notionrs::Client;

#[tokio::main]
async fn main() -> Result<(), notionrs::error::Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let request = client.get_self();

    let response = request.send().await?;

    let name = response.name;

    match name {
        None => println!("No name found"),
        Some(name) => println!("{}", name),
    }

    Ok(())
}
