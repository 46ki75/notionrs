mod integration_tests {

    #[tokio::test]
    async fn list_file_upload() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let secret = std::env::var("NOTION_TOKEN").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new().secret(secret);

        let _response = client.list_file_upload().send().await?;

        Ok(())
    }

    #[tokio::test]
    async fn list_file_upload_all() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let secret = std::env::var("NOTION_TOKEN").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new().secret(secret);

        let _response = notionrs::Client::paginate(client.list_file_upload()).await?;

        Ok(())
    }
}
