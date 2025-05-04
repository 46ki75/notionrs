mod integration_tests {

    use notionrs_schema::prelude::*;

    #[tokio::test]
    async fn crud_page_with_database() -> Result<(), notionrs::Error> {
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

        let options = vec![
            notionrs_schema::object::select::Select::default()
                .color(notionrs_schema::object::select::SelectColor::Blue)
                .name("IT")
                .id("id"),
            notionrs_schema::object::select::Select::default()
                .color(notionrs_schema::object::select::SelectColor::Red)
                .name("SoC")
                .id("id"),
            notionrs_schema::object::select::Select::default()
                .color(notionrs_schema::object::select::SelectColor::Green)
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
            .icon(notionrs_schema::object::icon::Icon::Emoji(
                notionrs_schema::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_schema::object::file::File::External(
                notionrs_schema::object::file::ExternalFile::from(
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
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        properties.insert(
            "My Checkbox".to_string(),
            PageProperty::Checkbox(PageCheckboxProperty::from(true)),
        );

        properties.insert(
            "Rich Text".to_string(),
            PageProperty::RichText(PageRichTextProperty::from("description")),
        );

        properties.insert(
            "Date".to_string(),
            PageProperty::Date(PageDateProperty::from(
                notionrs_schema::object::date::DateOrDateTime::DateTime(
                    time::OffsetDateTime::parse(
                        "2024-10-26T09:03:00.000Z",
                        &time::format_description::well_known::Rfc3339,
                    )
                    .unwrap(),
                ),
            )),
        );

        properties.insert(
            "email".to_string(),
            PageProperty::Email(PageEmailProperty::from("info@example.com")),
        );

        properties.insert(
            "Files & Media".to_string(),
            PageProperty::Files(PageFilesProperty::from("https://example.com/file.txt")),
        );

        let option = notionrs_schema::object::select::Select::from("IT");

        properties.insert(
            "Tags".to_string(),
            PageProperty::MultiSelect(
                PageMultiSelectProperty::default().multi_select(vec![option]),
            ),
        );

        properties.insert(
            "Number".to_string(),
            PageProperty::Number(PageNumberProperty::from(100000)),
        );

        properties.insert(
            "Phone".to_string(),
            PageProperty::PhoneNumber(PagePhoneNumberProperty::from("083-0000-0000")),
        );

        properties.insert(
            "Select".to_string(),
            PageProperty::Select(PageSelectProperty::from("IT")),
        );

        properties.insert(
            "URL".to_string(),
            PageProperty::Url(PageUrlProperty::from("IT")),
        );

        println!("{}", serde_json::to_string(&properties).unwrap());

        let request = client
            .create_page()
            .properties(properties)
            .database_id(response.id)
            .icon(notionrs_schema::object::icon::Icon::Emoji(
                notionrs_schema::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_schema::object::file::File::External(
                notionrs_schema::object::file::ExternalFile::from(
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
