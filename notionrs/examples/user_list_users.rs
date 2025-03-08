#[tokio::main]
async fn main() -> Result<(), notionrs::error::Error> {
    let client = notionrs::client::Client::new();

    let mut request = client.list_users();

    let response = request.send().await?;

    for user in response.results {
        let name = match user {
            notionrs::User::Bot(bot) => bot.name,
            notionrs::User::Person(person) => person.name,
        };

        match name {
            None => println!("No name found"),
            Some(name) => println!("{}", name),
        }
    }

    Ok(())
}
