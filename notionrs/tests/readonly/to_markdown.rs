mod integration_tests {

    /// <https://www.notion.so/297a03d79b2680c8acfed411d83d8c9c>
    static PAGE_ID: &str = "297a03d79b2680c8acfed411d83d8c9c";

    #[tokio::test]
    async fn to_markdown() {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client.to_markdown(PAGE_ID).await.unwrap().join("\n");

        println!("{response}");
    }
}
