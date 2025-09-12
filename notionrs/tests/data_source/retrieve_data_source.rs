mod integration_tests {

    #[tokio::test]
    async fn retrieve_data_source() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let data_source_id =
            std::env::var("NOTION_IT_DATABASE_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::Client::new();

        // # --------------------------------------------------------------------------------
        //
        // retrieve_data_source
        //
        // # --------------------------------------------------------------------------------

        let request = client.retrieve_data_source().data_source_id(data_source_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response).unwrap());

        Ok(())
    }
}
