mod integration_tests {

    // # --------------------------------------------------------------------------------
    //
    // search
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn search() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.search().query("").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }

    // # --------------------------------------------------------------------------------
    //
    // search (sort by relevance)
    //
    // # --------------------------------------------------------------------------------

    #[tokio::test]
    async fn search_sort_relevance() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.search().query("").sort_relevance();

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
    async fn search_page() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

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
    async fn search_database() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

        let notion_api_key = std::env::var("NOTION_API_KEY_READONLY").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.search_database().query("").sort_timestamp_asc();

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        Ok(())
    }
}
