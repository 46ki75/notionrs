mod integration_test {

    /// <https://www.notion.so/33ea03d79b2680639969f545681d2184>
    static DATABASE_ID: &str = "33ea03d79b2680639969f545681d2184";
    static DATA_SOURCE_ID: &str = "33ea03d7-9b26-805b-8b1a-000b460c02e9";

    #[tokio::test]
    async fn list_views_by_database_id() {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        let views = client
            .list_views()
            .database_id(DATABASE_ID)
            .send()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&views).unwrap());
    }

    #[tokio::test]
    async fn list_views_by_data_source_id() {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();

        let client = notionrs::Client::new(notion_api_key);

        let views = client
            .list_views()
            .data_source_id(DATA_SOURCE_ID)
            .send()
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&views).unwrap());
    }
}
