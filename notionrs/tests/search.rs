mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // search
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let client = notionrs::client::Client::new();

        let request = client.search().query("").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // search_page
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search_page() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let client = notionrs::client::Client::new();

        let request = client.search_page().query("").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // search_database
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    #[serial_test::serial]
    async fn search_database() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let client = notionrs::client::Client::new();

        let request = client.search_database().query("").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
