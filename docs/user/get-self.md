# Get Self

This method is used to get information about `Authorization` header token's bot user.

> [!INFO]
> You can find the official documentation [here](https://developers.notion.com/reference/get-self).
> 
## Basic Usage

```rs
#[tokio::main]
async fn main() -> Result<(), notionrs::error::Error> {
    let client = notionrs::client::Client::new();

    let request = client.get_self();

    let response = request.send().await?;

    let name = response.name;

    match name {
        None => println!("No name found"),
        Some(name) => println!("{}", name),
    }

    Ok(())
}
```