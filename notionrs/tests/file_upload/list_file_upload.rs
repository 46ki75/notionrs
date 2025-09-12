mod integration_tests {

    #[tokio::test]
    async fn list_file_upload() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let _response = client.list_file_uploads().send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn list_file_upload_all() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let _response = notionrs::Client::paginate(client.list_file_uploads()).await?;

        Ok(())
    }
}
