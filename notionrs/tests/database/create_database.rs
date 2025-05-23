mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            DatabaseProperty::Title(DatabaseTitleProperty::default()),
        );

        properties.insert(
            "My Checkbox".to_string(),
            DatabaseProperty::Checkbox(DatabaseCheckboxProperty::default()),
        );

        properties.insert(
            "Created User".to_string(),
            DatabaseProperty::CreatedBy(DatabaseCreatedByProperty::default()),
        );

        properties.insert(
            "Created Time".to_string(),
            DatabaseProperty::CreatedTime(DatabaseCreatedTimeProperty::default()),
        );

        properties.insert(
            "Date".to_string(),
            DatabaseProperty::Date(DatabaseDateProperty::default()),
        );

        properties.insert(
            "email".to_string(),
            DatabaseProperty::Email(DatabaseEmailProperty::default()),
        );

        properties.insert(
            "Files & Media".to_string(),
            DatabaseProperty::Files(DatabaseFilesProperty::default()),
        );

        properties.insert(
            "formula".to_string(),
            DatabaseProperty::Formula(
                DatabaseFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),
            ),
        );

        properties.insert(
            "Last Edited User".to_string(),
            DatabaseProperty::LastEditedBy(DatabaseLastEditedByProperty::default()),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            DatabaseProperty::LastEditedTime(DatabaseLastEditedTimeProperty::default()),
        );

        let options = vec![
            notionrs_types::object::select::Select::default()
                .color(notionrs_types::object::select::SelectColor::Blue)
                .name("IT")
                .id("id"),
            notionrs_types::object::select::Select::default()
                .color(notionrs_types::object::select::SelectColor::Red)
                .name("SoC")
                .id("id"),
            notionrs_types::object::select::Select::default()
                .color(notionrs_types::object::select::SelectColor::Green)
                .name("SoC")
                .id("id"),
        ];

        properties.insert(
            "Tags".to_string(),
            DatabaseProperty::MultiSelect(
                DatabaseMultiSelectProperty::default().options(options.clone()),
            ),
        );

        properties.insert(
            "Number".to_string(),
            DatabaseProperty::Number(DatabaseNumberProperty::default()),
        );

        properties.insert(
            "People".to_string(),
            DatabaseProperty::People(DatabasePeopleProperty::default()),
        );

        properties.insert(
            "Phone".to_string(),
            DatabaseProperty::PhoneNumber(DatabasePhoneNumberProperty::default()),
        );

        properties.insert(
            "Rich Text".to_string(),
            DatabaseProperty::RichText(DatabaseRichTextProperty::default()),
        );

        properties.insert(
            "Select".to_string(),
            DatabaseProperty::Select(DatabaseSelectProperty::default().options(options.clone())),
        );

        properties.insert(
            "URL".to_string(),
            DatabaseProperty::Url(DatabaseUrlProperty::default()),
        );

        let request = client
            .create_database()
            .page_id(page_id)
            .title(vec![RichText::from("Database Title")])
            .description(vec![RichText::from("Description of the Database")])
            .properties(properties)
            .icon(notionrs_types::object::icon::Icon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_types::object::file::File::External(
                notionrs_types::object::file::ExternalFile::from(
                    "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
                ),
            ));

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        let request = client.delete_block().block_id(response.id);

        let _ = request.send().await?;

        Ok(())
    }
}
