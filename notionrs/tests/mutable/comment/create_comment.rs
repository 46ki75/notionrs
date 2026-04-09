mod integration_tests {

    use notionrs_types::prelude::*;

    #[tokio::test]
    async fn create_comment() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();
        dotenvy::from_path(std::path::Path::new("../.env")).expect("Failed to load ../.env file");

        let page_id = std::env::var("NOTION_IT_SANDBOX_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let rich_text = vec![RichText::from("Test Comment!")];

        let request = client
            .create_comment()
            .page_id(page_id)
            .rich_text(rich_text);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
