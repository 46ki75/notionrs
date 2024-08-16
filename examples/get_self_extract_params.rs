#[tokio::main]
async fn main() -> Result<(), notionrs::error::NotionError> {
    let client = notionrs::client::NotionClient::new();

    let request = client.get_self();

    let response = request.send().await?;

    match response {
        notionrs::user::User::Bot(bot) => {
            let id = bot.id;

            let name = bot.name.unwrap();

            println!("The integration name being used is `{}` (ID: {})", name, id);
        }
        notionrs::user::User::Person(_) => panic!("Unexpected !"),
    }

    Ok(())
}
