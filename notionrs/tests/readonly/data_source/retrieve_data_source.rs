mod integration_tests {

    #[tokio::test]
    async fn retrieve_data_source() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        // # --------------------------------------------------------------------------------
        //
        // retrieve_data_source
        //
        // # --------------------------------------------------------------------------------

        let request = client
            .retrieve_data_source()
            .data_source_id(crate::readonly::DATA_SOURCE_ID);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        Ok(())
    }
}
