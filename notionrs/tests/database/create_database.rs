mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            DatabaseProperty::Title(DataSourceTitleProperty::default()),
        );

        properties.insert(
            "My Checkbox".to_string(),
            DatabaseProperty::Checkbox(DataSourceCheckboxProperty::default()),
        );

        properties.insert(
            "Created User".to_string(),
            DatabaseProperty::CreatedBy(DataSourceCreatedByProperty::default()),
        );

        properties.insert(
            "Created Time".to_string(),
            DatabaseProperty::CreatedTime(DataSourceCreatedTimeProperty::default()),
        );

        properties.insert(
            "Date".to_string(),
            DatabaseProperty::Date(DataSourceDateProperty::default()),
        );

        properties.insert(
            "email".to_string(),
            DatabaseProperty::Email(DataSourceEmailProperty::default()),
        );

        properties.insert(
            "Files & Media".to_string(),
            DatabaseProperty::Files(DataSourceFilesProperty::default()),
        );

        properties.insert(
            "formula".to_string(),
            DatabaseProperty::Formula(
                DataSourceFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),
            ),
        );

        properties.insert(
            "Last Edited User".to_string(),
            DatabaseProperty::LastEditedBy(DataSourceLastEditedByProperty::default()),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            DatabaseProperty::LastEditedTime(DataSourceLastEditedTimeProperty::default()),
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
            DatabaseProperty::MultiSelect(
                DataSourceMultiSelectProperty::default().options(options.clone()),
            ),
        );

        properties.insert(
            "Number".to_string(),
            DatabaseProperty::Number(DataSourceNumberProperty::default()),
        );

        properties.insert(
            "People".to_string(),
            DatabaseProperty::People(DataSourcePeopleProperty::default()),
        );

        properties.insert(
            "Phone".to_string(),
            DatabaseProperty::PhoneNumber(DataSourcePhoneNumberProperty::default()),
        );

        properties.insert(
            "Rich Text".to_string(),
            DatabaseProperty::RichText(DataSourceRichTextProperty::default()),
        );

        properties.insert(
            "Select".to_string(),
            DatabaseProperty::Select(DataSourceSelectProperty::default().options(options.clone())),
        );

        properties.insert(
            "URL".to_string(),
            DatabaseProperty::Url(DataSourceUrlProperty::default()),
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
