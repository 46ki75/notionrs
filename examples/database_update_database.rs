use notionrs::{
    database::{
        DatabaseMultiSelectProperty, DatabaseProperty, DatabaseRichTextProperty,
        DatabaseUrlProperty,
    },
    error::Error,
    Client, Emoji, ExternalFile, File, Icon, RichText, Select, SelectColor,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new().secret("NOTION_TOKEN");

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
            .name("SoC")
            .id("id"),
    ];

    properties.insert(
        "Tags".to_string(),
        Some(DatabaseProperty::MultiSelect(
            DatabaseMultiSelectProperty::default().options(options.clone()),
        )),
    );

    properties.insert(
        "Rich Text".to_string(),
        Some(DatabaseProperty::RichText(
            DatabaseRichTextProperty::default(),
        )),
    );

    properties.insert(
        "URL".to_string(),
        Some(DatabaseProperty::Url(DatabaseUrlProperty::default())),
    );

    let request = client
        .update_database()
        .databse_id("DATABASE_ID")
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
