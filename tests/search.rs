mod integration_tests {

    use notionrs::to_json::ToJson;

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

        let request = client.search().query("My Title").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", response.to_json());

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

        let request = client.search_page().query("title").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", response.to_json());

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

        println!("{}", response.to_json());

        Ok(())
    }
}
