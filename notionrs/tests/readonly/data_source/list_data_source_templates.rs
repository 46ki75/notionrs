mod integration_tests {

    #[tokio::test]
    async fn list_data_source_templates() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client
            .list_data_source_templates()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .page_size(10)
            .send()
            .await?;

        assert!(response.templates.len() >= 1);

        Ok(())
    }
}
