mod integration_tests {

    use notionrs::to_json::ToJson;

    #[tokio::test]
    async fn crud_page() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new(".env.test"))
            .expect("Failed to load .env.test file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // create_page
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            notionrs::page::PageProperty::Title(notionrs::page::PageTitleProperty::from(
                "My Page Title",
            )),
        );

        let request = client
            .create_page()
            .properties(properties)
            .page_id(page_id)
            .icon(notionrs::Icon::Emoji(notionrs::Emoji::from("🚧")))
            .cover(notionrs::File::External(notionrs::ExternalFile::from(
                "https://upload.wikimedia.org/wikipedia/commons/6/62/Tuscankale.jpg",
            )));

        let response = request.send().await?;

        // # --------------------------------------------------------------------------------
        //
        // update_page
        //
        // # --------------------------------------------------------------------------------

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            notionrs::page::PageProperty::Title(notionrs::page::PageTitleProperty::from(
                "My Page Title (Changed)",
            )),
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

        println!("{}", response.to_json());

        Ok(())
    }
}
