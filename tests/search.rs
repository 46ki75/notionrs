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

        let request = client.search().query("sdfasdfas");

        let response = request.send().await?;

        println!("{}", response.to_json());

        Ok(())
    }
}
