mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // search
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        //         let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let _client = notionrs::Client::new(notion_api_key);

        // let request = client.search().query("").sort_timestamp_asc();

        // let response = request.send().await?;

        // println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // search_page
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search_page() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        //         let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let _client = notionrs::Client::new(notion_api_key);

        // let request = client.search_page().query("").sort_timestamp_asc();

        // let response = request.send().await?;

        // println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // search_database
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search_database() -> Result<(), notionrs::Error> {
        dotenvy::from_path(std::path::Path::new("../.env")).ok();

        let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let _client = notionrs::Client::new(notion_api_key);

        // let request = client.search_database().query("").sort_timestamp_asc();

        // let response = request.send().await?;

        // println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
