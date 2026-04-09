mod integration_tests {

    // use notionrs_types::prelude::*;

    /// <https://www.notion.so/33da03d79b26804a9627dd3a8e15a3f2>
    static DATABASE_ID: &str = "33da03d79b26804a9627dd3a8e15a3f2";

    #[tokio::test]
    async fn retrieve_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.retrieve_database().database_id(DATABASE_ID);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
