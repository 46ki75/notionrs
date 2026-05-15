use notionrs::{Client, Error};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    struct MyPageProperty {
        #[serde(rename = "Name")]
        name: String,
    }

    let properties = MyPageProperty {
        name: "New Page".to_string(),
    };

    let request = client
        .update_page()
        .page_id("PAGE_ID")
        .properties(properties);

    let response = request.send().await?;

    println!("This block's id is {}", response.id);

    Ok(())
}
