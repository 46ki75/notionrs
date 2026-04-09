mod integration_tests {

    use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b2680628cf0ebeb854b8846>
    static PAGE_ID: &str = "33da03d79b2680628cf0ebeb854b8846";

    #[tokio::test]
    async fn crud_page() -> Result<(), notionrs::Error> {
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
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .page_id(PAGE_ID)
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
        // update_page
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            PageProperty::Title(PageTitleProperty::from("My Page Title (Changed)")),
        );

        let request = client
            .create_page()
            .properties(properties)
            .page_id(response.id);

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
}
