mod integration_tests {

    static PAGE_ID: &str = "33da03d79b2680d88273ff07e7a688ed";

    #[tokio::test]
    async fn get_page() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client
            .get_page::<std::collections::HashMap<String, notionrs_types::prelude::PageProperty>>()
            .page_id(PAGE_ID);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
