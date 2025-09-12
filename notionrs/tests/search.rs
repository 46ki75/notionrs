mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // search
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search() -> Result<(), notionrs::Error> {
        // dotenvy::dotenv().ok();

        // let client = notionrs::Client::new();

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
        // dotenvy::dotenv().ok();

        // let client = notionrs::Client::new();

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
        // dotenvy::dotenv().ok();

        // let client = notionrs::Client::new();

        // let request = client.search_database().query("").sort_timestamp_asc();

        // let response = request.send().await?;

        // println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
