use notionrs::{Client, Error};
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let mut properties = std::collections::HashMap::new();

    let options = vec![
        Select::default()
            .color(SelectColor::Blue)
            .name("IT")
            .id("id"),
        Select::default()
            .color(SelectColor::Red)
            .name("SoC")
            .id("id"),
        Select::default()
            .color(SelectColor::Green)
            .name("TPM")
            .id("id"),
    ];

    properties.insert(
        "Tags".to_string(),
        Some(DatabaseProperty::MultiSelect(
            DataSourceMultiSelectProperty::default().options(options.clone()),
        )),
    );

    properties.insert(
        "Rich Text".to_string(),
        Some(DatabaseProperty::RichText(
            DataSourceRichTextProperty::default(),
        )),
    );

    properties.insert(
        "URL".to_string(),
        Some(DatabaseProperty::Url(DataSourceUrlProperty::default())),
    );

    let request = client
        .update_database()
        .database_id("DATABASE_ID")
        .title(vec![RichText::from("Database Title (changed)")])
        .description(vec![RichText::from(
            "Description of the Database (changed)",
        )])
        .properties(properties)
        .icon(Icon::Emoji(Emoji::from("🚧")))
        .cover(File::External(ExternalFile::from(
            "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
        )));

    let _response = request.send().await?;

    Ok(())
}
