mod integration_tests {

    #[tokio::test]
    async fn create_database() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // create_database
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Old Title".to_string(),
            notionrs::database::DatabaseProperty::Title(
                notionrs::database::DatabaseTitleProperty::default(),
            ),
        );

        properties.insert(
            "My Checkbox".to_string(),
            notionrs::database::DatabaseProperty::Checkbox(
                notionrs::database::DatabaseCheckboxProperty::default(),
            ),
        );

        let request = client
            .create_database()
            .page_id(page_id)
            .title(vec![notionrs::RichText::from("Database Title")])
            .description(vec![notionrs::RichText::from(
                "Description of the Database",
            )])
            .properties(properties);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        // # --------------------------------------------------------------------------------
        //
        // update_database
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Old Title".to_string(),
            Some(notionrs::database::DatabaseProperty::Title(
                notionrs::database::DatabaseTitleProperty::default().name("New Title"),
            )),
        );

        properties.insert("My Checkbox".to_string(), None);

        properties.insert(
            "Created User".to_string(),
            Some(notionrs::database::DatabaseProperty::CreatedBy(
                notionrs::database::DatabaseCreatedByProperty::default(),
            )),
        );

        properties.insert(
            "Created Time".to_string(),
            Some(notionrs::database::DatabaseProperty::CreatedTime(
                notionrs::database::DatabaseCreatedTimeProperty::default(),
            )),
        );

        properties.insert(
            "Date".to_string(),
            Some(notionrs::database::DatabaseProperty::Date(
                notionrs::database::DatabaseDateProperty::default(),
            )),
        );

        properties.insert(
            "email".to_string(),
            Some(notionrs::database::DatabaseProperty::Email(
                notionrs::database::DatabaseEmailProperty::default(),
            )),
        );

        properties.insert(
            "Files & Media".to_string(),
            Some(notionrs::database::DatabaseProperty::Files(
                notionrs::database::DatabaseFilesProperty::default(),
            )),
        );

        properties.insert(
            "formula".to_string(),
            Some(notionrs::database::DatabaseProperty::Formula(notionrs::database::DatabaseFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),)),
        );

        properties.insert(
            "Last Edited User".to_string(),
            Some(notionrs::database::DatabaseProperty::LastEditedBy(
                notionrs::database::DatabaseLastEditedByProperty::default(),
            )),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            Some(notionrs::database::DatabaseProperty::LastEditedTime(
                notionrs::database::DatabaseLastEditedTimeProperty::default(),
            )),
        );

        let options = vec![
            notionrs::Select::default()
                .color(notionrs::SelectColor::Blue)
                .name("IT")
                .id("id"),
            notionrs::Select::default()
                .color(notionrs::SelectColor::Red)
                .name("SoC")
                .id("id"),
            notionrs::Select::default()
                .color(notionrs::SelectColor::Green)
                .name("SoC")
                .id("id"),
        ];

        properties.insert(
            "Tags".to_string(),
            Some(notionrs::database::DatabaseProperty::MultiSelect(
                notionrs::database::DatabaseMultiSelectProperty::default().options(options.clone()),
            )),
        );

        properties.insert(
            "Number".to_string(),
            Some(notionrs::database::DatabaseProperty::Number(
                notionrs::database::DatabaseNumberProperty::default(),
            )),
        );

        properties.insert(
            "People".to_string(),
            Some(notionrs::database::DatabaseProperty::People(
                notionrs::database::DatabasePeopleProperty::default(),
            )),
        );

        properties.insert(
            "Phone".to_string(),
            Some(notionrs::database::DatabaseProperty::PhoneNumber(
                notionrs::database::DatabasePhoneNumberProperty::default(),
            )),
        );

        properties.insert(
            "Rich Text".to_string(),
            Some(notionrs::database::DatabaseProperty::RichText(
                notionrs::database::DatabaseRichTextProperty::default(),
            )),
        );

        properties.insert(
            "Select".to_string(),
            Some(notionrs::database::DatabaseProperty::Select(
                notionrs::database::DatabaseSelectProperty::default().options(options.clone()),
            )),
        );

        properties.insert(
            "URL".to_string(),
            Some(notionrs::database::DatabaseProperty::Url(
                notionrs::database::DatabaseUrlProperty::default(),
            )),
        );

        let request = client
            .update_database()
            .databse_id(response.id)
            .title(vec![notionrs::RichText::from("Database Title (changed)")])
            .description(vec![notionrs::RichText::from(
                "Description of the Database (changed)",
            )])
            .properties(properties)
            .icon(notionrs::Icon::Emoji(notionrs::Emoji::from('🚧')))
            .cover(notionrs::File::External(notionrs::ExternalFile::from(
                "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
            )));

        let response = request.send().await?;

        // cleanup
        let request = client.delete_block().block_id(response.id);

        let _ = request.send().await?;

        Ok(())
    }
}
