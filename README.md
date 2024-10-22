# Notion API Client for Rust

[![Unit Test](https://github.com/46ki75/notionrs/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/46ki75/notionrs/actions/workflows/unit-tests.yml)
[![Check Documentation Build](https://github.com/46ki75/notionrs/actions/workflows/build-documentation.yml/badge.svg)](https://github.com/46ki75/notionrs/actions/workflows/build-documentation.yml)
[![Crates.io](https://img.shields.io/crates/v/notionrs?logo=rust)](https://crates.io/crates/notionrs/)

**Status: Alpha Release! (Under Construction**) 🚧

This project is currently under active development and is not yet ready for production use. Features and API stability may change without notice. Contributions and feedback are welcome!

## Features currently released

As part of the alpha release, the following features are available. Please note that API changes may occur before the official release.

- Append block children
- Retrieve a block
- Retrieve block children
- Update a block
- Delete a block

## Basic Usage

Below is a basic example. (More detailed documentation is coming soon, so please stay tuned!)

```rs
use notionrs::{
    block::{Block, ParagraphBlock},
    error::Error,
    Client, RichText,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    let client = Client::new();

    // Here, we're retrieving the ID from an environment variable,
    // but you can change the method of retrieval to suit your needs.
    let block_id = std::env::var("NOTION_PAGE_ID").unwrap();

    let block = Block::Paragraph {
        paragraph: ParagraphBlock::new()
            .rich_text(vec![RichText::from("Time to start with Notion in Rust")]),
    };

    let request = client
        .append_block_children()
        .block_id(block_id.clone())
        .children(vec![block]);

    let response = request.send().await?;

    println!("{:?}", response);

    Ok(())
}
```
