mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_page_on_data_source() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_page
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .icon(notionrs_types::object::emoji_and_icon::EmojiAndIcon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_types::object::file::File::External(
                notionrs_types::object::file::ExternalFile::from(
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

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    #[tokio::test]
    async fn create_page_on_data_source_with_template_default() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_MUTABLE").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // create_page with template default
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "My Title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .data_source_id(crate::mutable::DATA_SOURCE_ID)
            .icon(notionrs_types::object::emoji_and_icon::EmojiAndIcon::Emoji(
                notionrs_types::object::emoji::Emoji::from("🚧"),
            ))
            .cover(notionrs_types::object::file::File::External(
                notionrs_types::object::file::ExternalFile::from(
                    "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
                ),
            ))
            .template_default();

        let response = request.send().await?;

        // // # --------------------------------------------------------------------------------
        // //
        // // cleanup
        // //
        // // # --------------------------------------------------------------------------------

        let request = client.delete_block().block_id(response.id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
