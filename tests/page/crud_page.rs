mod integration_tests {

    use notionrs::to_json::ToJson;

    #[tokio::test]
    async fn crud_page() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let page_id = std::env::var("NOTION_PAGE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::Client::new();

        let mut properties = std::collections::HashMap::new();

        properties.insert(
            "title".to_string(),
            notionrs::page::PageProperty::Title(notionrs::page::PageTitleProperty::from(
                "My Page Title",
            )),
        );

        let request = client.create_page().properties(properties).page_id(page_id);

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }
}
