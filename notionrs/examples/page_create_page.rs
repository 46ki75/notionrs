use notionrs::{Client, Error};
use notionrs_types::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let mut properties = std::collections::HashMap::new();

    properties.insert(
        "Name".to_string(),
        PageProperty::Title(PageTitleProperty::from("My Page")),
    );

    properties.insert(
        "Tags".to_string(),
        PageProperty::MultiSelect(PageMultiSelectProperty {
            multi_select: vec![
                Select {
                    name: "My Tag".to_string(),
                    color: Some(SelectColor::Blue),
                    ..Default::default()
                },
                Select {
                    name: "My Option".to_string(),
                    color: Some(SelectColor::Green),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }),
    );

    let request = client
        .create_page()
        .page_id("PAGE_ID")
        .properties(properties);

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    Ok(())
}
