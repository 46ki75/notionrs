mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn update_database() -> Result<(), notionrs::Error> {
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
            DataSourceProperty::Title(DataSourceTitleProperty::default()),
        );

        properties.insert(
            "My Checkbox".to_string(),
            DataSourceProperty::Checkbox(DataSourceCheckboxProperty::default()),
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

        let request = client
            .update_database()
            .database_id(response.id)
            .title(vec![RichText::from("Database Title (changed)")])
            .description(vec![RichText::from(
                "Description of the Database (changed)",
            )])
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
