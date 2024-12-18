// ## Extract Params
//
#[tokio::main]
async fn main() -> Result<(), notionrs::error::Error> {
    let client = notionrs::client::Client::new();

    let request = client.get_self();

    let response = request.send().await?;

    let id = response.id;
    let name = response.name.unwrap();

    println!("The integration name being used is `{}` (ID: {})", name, id);

    Ok(())
}
// Expected output:
//
// ```text
// The integration name being used is `integration-name` (ID: b610aa5b-800e-4c2e-9d5c-72b72b5dedc0)
// ```
