mod integration_tests {

    #[tokio::test]
    async fn get_user() -> Result<(), notionrs::Error> {
        dotenvy::dotenv().ok();

                let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let res = client.get_self().send().await?;

        let user_id = res.id;

                let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
        let client = notionrs::Client::new(notion_api_key);

        let request = client.get_user().user_id(user_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

        assert_eq!(response.r#type, Some("bot".to_string()));

        Ok(())
    }
}
