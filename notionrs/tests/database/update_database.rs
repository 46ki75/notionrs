mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

                let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_database
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Old Title".to_string(),
            DatabaseProperty::Title(DatabaseTitleProperty::default()),
        );

        properties.insert(
            "My Checkbox".to_string(),
            DatabaseProperty::Checkbox(DatabaseCheckboxProperty::default()),
        );

        let request = client
            .create_database()
            .page_id(page_id)
            .title(vec![RichText::from("Database Title")])
            .description(vec![RichText::from("Description of the Database")])
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
            Some(DatabaseProperty::Title(
                DatabaseTitleProperty::default().name("New Title"),
            )),
        );

        properties.insert("My Checkbox".to_string(), None);

        properties.insert(
            "Created User".to_string(),
            Some(DatabaseProperty::CreatedBy(
                DatabaseCreatedByProperty::default(),
            )),
        );

        properties.insert(
            "Created Time".to_string(),
            Some(DatabaseProperty::CreatedTime(
                DatabaseCreatedTimeProperty::default(),
            )),
        );

        properties.insert(
            "Date".to_string(),
            Some(DatabaseProperty::Date(DatabaseDateProperty::default())),
        );

        properties.insert(
            "email".to_string(),
            Some(DatabaseProperty::Email(DatabaseEmailProperty::default())),
        );

        properties.insert(
            "Files & Media".to_string(),
            Some(DatabaseProperty::Files(DatabaseFilesProperty::default())),
        );

        properties.insert(
            "formula".to_string(),
            Some(DatabaseProperty::Formula(DatabaseFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),)),
        );

        properties.insert(
            "Last Edited User".to_string(),
            Some(DatabaseProperty::LastEditedBy(
                DatabaseLastEditedByProperty::default(),
            )),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            Some(DatabaseProperty::LastEditedTime(
                DatabaseLastEditedTimeProperty::default(),
            )),
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
                .name("TPM")
                .id("id"),
        ];

        properties.insert(
            "Tags".to_string(),
            Some(DatabaseProperty::MultiSelect(
                DatabaseMultiSelectProperty::default().options(options.clone()),
            )),
        );

        properties.insert(
            "Number".to_string(),
            Some(DatabaseProperty::Number(DatabaseNumberProperty::default())),
        );

        properties.insert(
            "People".to_string(),
            Some(DatabaseProperty::People(DatabasePeopleProperty::default())),
        );

        properties.insert(
            "Phone".to_string(),
            Some(DatabaseProperty::PhoneNumber(
                DatabasePhoneNumberProperty::default(),
            )),
        );

        properties.insert(
            "Rich Text".to_string(),
            Some(DatabaseProperty::RichText(
                DatabaseRichTextProperty::default(),
            )),
        );

        properties.insert(
            "Select".to_string(),
            Some(DatabaseProperty::Select(
                DatabaseSelectProperty::default().options(options.clone()),
            )),
        );

        properties.insert(
            "URL".to_string(),
            Some(DatabaseProperty::Url(DatabaseUrlProperty::default())),
        );

        let request = client
            .update_database()
            .database_id(response.id)
            .title(vec![RichText::from("Database Title (changed)")])
            .description(vec![RichText::from(
                "Description of the Database (changed)",
            )])
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

        // cleanup
        let request = client.delete_block().block_id(response.id);

        let _ = request.send().await?;

        Ok(())
    }
}
