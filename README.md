# Notion API Client for Rust

[![Unit Test](https://github.com/46ki75/notionrs/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/46ki75/notionrs/actions/workflows/unit-tests.yml)
[![Check Documentation Build](https://github.com/46ki75/notionrs/actions/workflows/build-documentation.yml/badge.svg)](https://github.com/46ki75/notionrs/actions/workflows/build-documentation.yml)
[![Crates.io](https://img.shields.io/crates/v/notionrs?logo=rust)](https://crates.io/crates/notionrs/)

![ogp](./assets/ogp.webp)

This project is currently under active development and is not yet ready for production use. Features and API stability may change without notice. Contributions and feedback are welcome!

- [♻ Release Notes](https://github.com/46ki75/notionrs/releases)
- [🛠️ API Reference (docs.rs)](https://docs.rs/notionrs/latest/notionrs/)

## Features currently released

As part of the alpha release, the following features are available. Please note that API changes may occur before the official release.

- Blocks
  - Append block children
  - Retrieve a block
  - Retrieve block children
  - Update a block
  - Delete a block
- Databases
  - Create a database
  - Update a database
  - Retrieve a database
- Data sources
  - Create a data source
  - Update a data source
  - Retrieve a data source
  - Query a data source
- Pages
  - Create a page
  - Retrieve a page property item
  - Retrieve a page
  - Update page properties
- Users
  - List all users
  - Retrieve a user
  - Retrieve your token's bot user
- Comments
  - Create comment
  - Retrieve comments
- Search
  - Search by title

## Basic Usage

Below is a basic example.

`Cargo.toml`:

```toml
notionrs = { version = "0" }
notionrs_types = { version = "0" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

`src/main.rs`:

```rs
use notionrs::Client;
use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let notion_api_key = std::env::var("NOTION_TOKEN").unwrap();
    let client = Client::new(notion_api_key);

    let filter = Filter::timestamp_past_month();

    let sort = Sort::desc("Created Time");

    let request = client
        .query_data_source()
        .data_source_id("DATA_SOURCE_ID")
        .filter(filter)
        .sorts(vec![sort]);

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct MyProperties {
        #[serde(rename = "My Title")]
        pub title: PageTitleProperty,
    }

    let response = request.send::<MyProperties>().await?;

    for page in response.results {
        println!("{}", page.properties.title.to_string());
    }

    Ok(())
}
```
