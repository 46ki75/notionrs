// This endpoint requires an integration to have user information capabilities.
// Attempting to call this API without user information capabilities will
// return an HTTP response with a 403 status code. For more information
// on integration capabilities, see the
// [capabilities guide](https://developers.notion.com/reference/capabilities#user-capabilities).
//
// [Notion API Reference](https://developers.notion.com/reference/get-self)
//
// This method can fetch a list of users present in the current Notion workspace.
// The user list includes humans (Notion accounts) and bots (Notion API integrations).
//
// ## Basic Usage
// It adopts the builder pattern, allowing you to add options and then execute
// the API call by invoking the `send()` method at the end.

use notionrs::to_json::ToJson; // When you need to use the `.to_json()` method

#[tokio::main]
async fn main() -> Result<(), notionrs::error::NotionError> {
    let client = notionrs::client::NotionClient::new();

    let request = client.get_self();

    let response = request.send().await?;

    println!("{}", response.to_json());

    Ok(())
}

// An example of a JSON response is as follows:
//
// ```json
// {
//     "object": "user",
//     "id": "b610aa5b-800e-4c2e-9d5c-72b72b5dedc0",
//     "name": "integration-name",
//     "avatar_url": null,
//     "type": "bot",
//     "bot": {
//         "owner": {
//             "type": "workspace",
//             "workspace": true
//         },
//         "workspace_name": "Workspace Name"
//     }
// }
// ```
