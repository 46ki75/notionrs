mod integration_tests {

    #[tokio::test]
    async fn get_page() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env"))
            .expect("Failed to load ../.env file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

                let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.get_page().page_id(page_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
