mod integration_tests {

    #[tokio::test]
    async fn crud_page_with_database() -> Result<(), notionrs::error::Error> {
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

        // # --------------------------------------------------------------------------------
        //
        // create_page
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            notionrs::object::page::PageProperty::Title(
                notionrs::object::page::PageTitleProperty::from("My Page Title"),
            ),
        );

        properties.insert(
            "My Checkbox".to_string(),
            notionrs::object::page::PageProperty::Checkbox(
                notionrs::object::page::PageCheckboxProperty::from(true),
            ),
        );

        properties.insert(
            "Rich Text".to_string(),
            notionrs::object::page::PageProperty::RichText(
                notionrs::object::page::PageRichTextProperty::from("description"),
            ),
        );

        properties.insert(
            "Date".to_string(),
            notionrs::object::page::PageProperty::Date(
                notionrs::object::page::PageDateProperty::from(
                    chrono::DateTime::parse_from_rfc3339("2024-10-26T09:03:00.000Z").unwrap(),
                ),
            ),
        );

        properties.insert(
            "email".to_string(),
            notionrs::object::page::PageProperty::Email(
                notionrs::object::page::PageEmailProperty::from("info@example.com"),
            ),
        );

        properties.insert(
            "Files & Media".to_string(),
            notionrs::object::page::PageProperty::Files(
                notionrs::object::page::PageFilesProperty::from("https://example.com/file.txt"),
            ),
        );

        let option = notionrs::object::select::Select::from("IT");

        properties.insert(
            "Tags".to_string(),
            notionrs::object::page::PageProperty::MultiSelect(
                notionrs::object::page::PageMultiSelectProperty::default()
                    .multi_select(vec![option]),
            ),
        );

        properties.insert(
            "Number".to_string(),
            notionrs::object::page::PageProperty::Number(
                notionrs::object::page::PageNumberProperty::from(100000),
            ),
        );

        properties.insert(
            "Phone".to_string(),
            notionrs::object::page::PageProperty::PhoneNumber(
                notionrs::object::page::PagePhoneNumberProperty::from("083-0000-0000"),
            ),
        );

        properties.insert(
            "Select".to_string(),
            notionrs::object::page::PageProperty::Select(
                notionrs::object::page::PageSelectProperty::from("IT"),
            ),
        );

        properties.insert(
            "URL".to_string(),
            notionrs::object::page::PageProperty::Url(
                notionrs::object::page::PageUrlProperty::from("IT"),
            ),
        );

        println!("{}", serde_json::to_string(&properties).unwrap());

        let request = client
            .create_page()
            .properties(properties)
            .database_id(response.id)
            .icon(notionrs::object::icon::Icon::Emoji(
                notionrs::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs::object::file::File::External(
                notionrs::object::file::ExternalFile::from(
                    "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
                ),
            ));

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // cleanup
        //
        // # --------------------------------------------------------------------------------

        let request = client.delete_block().block_id(response.id);

        let _ = request.send().await?;

        Ok(())
    }
}
