mod integration_tests {

    #[tokio::test]
    async fn retrieve_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let database_id = std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // retrieve_database
        //
        // # --------------------------------------------------------------------------------

        let request = client.retrieve_database().database_id(database_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        // cleanup
        let request = client.delete_block().block_id(response.id);

        let _ = request.send().await?;

        Ok(())
    }
}
