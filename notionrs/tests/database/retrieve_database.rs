mod integration_tests {

    // use notionrs_types::prelude::*;

    #[tokio::test]
    async fn retrieve_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.retrieve_database().database_id(database_id);

        let _response = request.send().await?;

        Ok(())
    }
}
