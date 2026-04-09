mod integration_tests {

    #[tokio::test]
    async fn to_markdown() {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let block_id = std::env::var("NOTION_IT_MARKDOWN_PAGE_ID").unwrap();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.to_markdown(block_id).await.unwrap().join("\n");

        println!("{response}");
    }
}
