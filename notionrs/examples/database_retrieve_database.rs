use notionrs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new().secret("NOTION_TOKEN");

    let request = client.retrieve_database().database_id("DATABASE_ID");

    let response = request.send().await?;

    let properties = response.properties;

    let tags_property = properties
        .get("Tags")
        .ok_or("Tags property not found".to_string())?;

    if let notionrs::object::database::DatabaseProperty::MultiSelect(tags) = tags_property {
        for tag in tags.multi_select.options.clone() {
            println!("Tag: {}", tag.name);
        }
    }

    Ok(())
}
