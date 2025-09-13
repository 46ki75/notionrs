# Examples

To run the files located under the `examples/` directory, first create a `.env` file in the root directory and input your `NOTION_TOKEN`. Detailed instructions on how to obtain the token can be found in [Notion's documentation](https://developers.notion.com/docs/authorization#internal-integration-auth-flow-set-up).

```ini
NOTION_TOKEN=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

By doing this, the `NOTION_TOKEN` environment variable will be automatically loaded when you initialize the client.

```rs
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);
```

Although `NOT recommended`, you can also hard-code the token using the `.secret()` method.

```rs
let client =
    notionrs::client::Client::new().secret("secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
```

Example of retrieving the secret from a secret store:

```rs
let secret = get_notion_token_from_secret_store().await.unwrap();
let client = notionrs::client::Client::new().secret(secret);
```
