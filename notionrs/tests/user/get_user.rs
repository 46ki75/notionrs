mod integration_tests {

    #[tokio::test]
    async fn get_user() -> Result<(), notionrs::error::Error> {
        dotenvy::dotenv().ok();

        let client = notionrs::client::Client::new();

        let res = client.get_self().send().await?;

        let user_id = res.id;

        let client = notionrs::client::Client::new();

        let request = client.get_user().user_id(user_id);

        let response = request.send().await?;

        println!("{}", serde_json::to_string(&response)?);

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
