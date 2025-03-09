mod integration_tests {

    #[tokio::test]
    async fn create_database() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            notionrs::object::database::DatabaseProperty::Title(
                notionrs::object::database::DatabaseTitleProperty::default(),
            ),
        );

        properties.insert(
            "My Checkbox".to_string(),
            notionrs::object::database::DatabaseProperty::Checkbox(
                notionrs::object::database::DatabaseCheckboxProperty::default(),
            ),
        );

        properties.insert(
            "Created User".to_string(),
            notionrs::object::database::DatabaseProperty::CreatedBy(
                notionrs::object::database::DatabaseCreatedByProperty::default(),
            ),
        );

        properties.insert(
            "Created Time".to_string(),
            notionrs::object::database::DatabaseProperty::CreatedTime(
                notionrs::object::database::DatabaseCreatedTimeProperty::default(),
            ),
        );

        properties.insert(
            "Date".to_string(),
            notionrs::object::database::DatabaseProperty::Date(
                notionrs::object::database::DatabaseDateProperty::default(),
            ),
        );

        properties.insert(
            "email".to_string(),
            notionrs::object::database::DatabaseProperty::Email(
                notionrs::object::database::DatabaseEmailProperty::default(),
            ),
        );

        properties.insert(
            "Files & Media".to_string(),
            notionrs::object::database::DatabaseProperty::Files(
                notionrs::object::database::DatabaseFilesProperty::default(),
            ),
        );

        properties.insert(
            "formula".to_string(),
            notionrs::object::database::DatabaseProperty::Formula(
                notionrs::object::database::DatabaseFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),
            ),
        );

        properties.insert(
            "Last Edited User".to_string(),
            notionrs::object::database::DatabaseProperty::LastEditedBy(
                notionrs::object::database::DatabaseLastEditedByProperty::default(),
            ),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            notionrs::object::database::DatabaseProperty::LastEditedTime(
                notionrs::object::database::DatabaseLastEditedTimeProperty::default(),
            ),
        );

        let options = vec![
            notionrs::object::select::Select::default()
                .color(notionrs::object::select::SelectColor::Blue)
                .name("IT")
                .id("id"),
            notionrs::object::select::Select::default()
                .color(notionrs::object::select::SelectColor::Red)
                .name("SoC")
                .id("id"),
            notionrs::object::select::Select::default()
                .color(notionrs::object::select::SelectColor::Green)
                .name("SoC")
                .id("id"),
        ];

        properties.insert(
            "Tags".to_string(),
            notionrs::object::database::DatabaseProperty::MultiSelect(
                notionrs::object::database::DatabaseMultiSelectProperty::default()
                    .options(options.clone()),
            ),
        );

        properties.insert(
            "Number".to_string(),
            notionrs::object::database::DatabaseProperty::Number(
                notionrs::object::database::DatabaseNumberProperty::default(),
            ),
        );

        properties.insert(
            "People".to_string(),
            notionrs::object::database::DatabaseProperty::People(
                notionrs::object::database::DatabasePeopleProperty::default(),
            ),
        );

        properties.insert(
            "Phone".to_string(),
            notionrs::object::database::DatabaseProperty::PhoneNumber(
                notionrs::object::database::DatabasePhoneNumberProperty::default(),
            ),
        );

        properties.insert(
            "Rich Text".to_string(),
            notionrs::object::database::DatabaseProperty::RichText(
                notionrs::object::database::DatabaseRichTextProperty::default(),
            ),
        );

        properties.insert(
            "Select".to_string(),
            notionrs::object::database::DatabaseProperty::Select(
                notionrs::object::database::DatabaseSelectProperty::default()
                    .options(options.clone()),
            ),
        );

        properties.insert(
            "URL".to_string(),
            notionrs::object::database::DatabaseProperty::Url(
                notionrs::object::database::DatabaseUrlProperty::default(),
            ),
        );

        let request = client
            .create_database()
            .page_id(page_id)
            .title(vec![notionrs::object::rich_text::RichText::from(
                "Database Title",
            )])
            .description(vec![notionrs::object::rich_text::RichText::from(
                "Description of the Database",
            )])
            .properties(properties)
            .icon(notionrs::object::icon::Icon::Emoji(
                notionrs::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs::object::file::File::External(
                notionrs::object::file::ExternalFile::from(
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
