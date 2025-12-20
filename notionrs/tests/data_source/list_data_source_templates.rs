mod integration_tests {

    // use notionrs_types::prelude::*;

    #[tokio::test]
    async fn list_data_source_templates() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let data_source_id =
            std::env::var("NOTION_IT_DATA_SOURCE_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let response = client
            .list_data_source_templates()
            .data_source_id(data_source_id)
            .page_size(10)
            .send()
            .await?;

        assert!(response.templates.len() >= 1);

        Ok(())
    }
}
