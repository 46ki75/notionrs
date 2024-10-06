mod integration_tests {

    use notionrs::to_json::ToJson;

    /// This integration test cannot be run unless explicit permission
    /// for user reading is granted in the Notion API key issuance settings.
    ///
    /// To conduct integration testing, write the following in the `.env` file.
    /// ```ini
    /// NOTION_USER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    /// ```
    #[tokio::test]
    async fn get_user() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();
        let user_id = std::env::var("NOTION_USER_ID").unwrap_or_else(|_| String::new());

        let client = notionrs::client::NotionClient::new();

        let request = client.get_user().user_id(user_id);

        let response = request.send().await?;

        println!("{}", response.to_json());

        match response {
            notionrs::user::User::Bot(bot) => {
                assert_eq!(bot.r#type, Some("bot".to_string()))
            }
            notionrs::user::User::Person(person) => {
                assert_eq!(person.r#type, Some("person".to_string()))
            }
        }

        Ok(())
    }
}
