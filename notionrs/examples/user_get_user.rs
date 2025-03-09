#[tokio::main]
async fn main() -> Result<(), notionrs::error::Error> {
    let client = notionrs::client::Client::new();

    let request = client.get_user().user_id("USER_ID");

    let response = request.send().await?;

    let name = match response {
        notionrs::object::user::User::Bot(bot) => bot.name,
        notionrs::object::user::User::Person(person) => person.name,
    };

    match name {
        None => println!("No name found"),
        Some(name) => println!("{}", name),
    }

    Ok(())
}
