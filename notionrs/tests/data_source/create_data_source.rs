mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_data_source() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_database
        //
        // # --------------------------------------------------------------------------------

        let response = client.create_database().page_id(page_id).send().await?;

        let database_id = response.id;

        // # --------------------------------------------------------------------------------
        //
        // create_data_source
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "Title".to_string(),
            DataSourceProperty::Title(DataSourceTitleProperty::default()),
        );

        properties.insert(
            "My Checkbox".to_string(),
            DataSourceProperty::Checkbox(DataSourceCheckboxProperty::default()),
        );

        properties.insert(
            "Created User".to_string(),
            DataSourceProperty::CreatedBy(DataSourceCreatedByProperty::default()),
        );

        properties.insert(
            "Created Time".to_string(),
            DataSourceProperty::CreatedTime(DataSourceCreatedTimeProperty::default()),
        );

        properties.insert(
            "Date".to_string(),
            DataSourceProperty::Date(DataSourceDateProperty::default()),
        );

        properties.insert(
            "email".to_string(),
            DataSourceProperty::Email(DataSourceEmailProperty::default()),
        );

        properties.insert(
            "Files & Media".to_string(),
            DataSourceProperty::Files(DataSourceFilesProperty::default()),
        );

        properties.insert(
            "formula".to_string(),
            DataSourceProperty::Formula(
                DataSourceFormulaProperty::from(r#"{{notion:block_property:BtVS:00000000-0000-0000-0000-000000000000:8994905a-074a-415f-9bcf-d1f8b4fa38e4}}/2"#),
            ),
        );

        properties.insert(
            "Last Edited User".to_string(),
            DataSourceProperty::LastEditedBy(DataSourceLastEditedByProperty::default()),
        );

        properties.insert(
            "Last Edited Time".to_string(),
            DataSourceProperty::LastEditedTime(DataSourceLastEditedTimeProperty::default()),
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
            DataSourceProperty::MultiSelect(
                DataSourceMultiSelectProperty::default().options(options.clone()),
            ),
        );

        properties.insert(
            "Number".to_string(),
            DataSourceProperty::Number(DataSourceNumberProperty::default()),
        );

        properties.insert(
            "People".to_string(),
            DataSourceProperty::People(DataSourcePeopleProperty::default()),
        );

        properties.insert(
            "Phone".to_string(),
            DataSourceProperty::PhoneNumber(DataSourcePhoneNumberProperty::default()),
        );

        properties.insert(
            "Rich Text".to_string(),
            DataSourceProperty::RichText(DataSourceRichTextProperty::default()),
        );

        properties.insert(
            "Select".to_string(),
            DataSourceProperty::Select(
                DataSourceSelectProperty::default().options(options.clone()),
            ),
        );

        properties.insert(
            "URL".to_string(),
            DataSourceProperty::Url(DataSourceUrlProperty::default()),
        );

        let request = client
            .create_data_source()
            .database_id(&database_id)
            .title(vec![RichText::from("Database Title")])
            .properties(properties)
            .icon(notionrs_types::object::icon::Icon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ));

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        // # --------------------------------------------------------------------------------
        //
        // cleanup
        //
        // # --------------------------------------------------------------------------------

        let _ = client.delete_block().block_id(&database_id).send().await?;

        Ok(())
    }
}
