mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // query_data_source (struct)
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn query_data_source_with_struct() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new(".env.readonly")).ok();

        let notion_api_key = std::env::var("NOTION_API_KEY").unwrap();
        let client = notionrs::Client::new(notion_api_key);
        let res = client
            .query_data_source()
            .data_source_id(crate::readonly::DATA_SOURCE_ID)
            .send::<crate::IntegrationTestReadOnlyDataSourceSchema>()
            .await?;

        println!("{}", serde_json::to_string(&res)?);

        Ok(())
    }
}
