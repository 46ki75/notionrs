mod integration_tests {

    static DATA_SOURCE_ID: &str = "33da03d7-9b26-81cb-90c7-000b8fb827a8";

    #[tokio::test]
    async fn retrieve_data_source() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly"))
            .expect("Failed to load .env.readonly file");

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // retrieve_data_source
        //
        // # --------------------------------------------------------------------------------

        let request = client.retrieve_data_source().data_source_id(DATA_SOURCE_ID);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        Ok(())
    }
}
