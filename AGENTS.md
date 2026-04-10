# notionrs

A Notion API client implemented in Rust.

## Official Documentation Index

Notion provides an official documentation index at [llms.txt](https://developers.notion.com/llms.txt). You must check this documentation index and the documentation every single time you implement something.

## Directory Structure

- `notionrs/`: Main crate. Contains a client that sends API requests to Notion API endpoints.
- `notionrs_macro/`: Provides derive macros. It is published to crates.io, but its API is intended for internal use only.
- `notionrs_types/`: Defines structs used in other crates.
- `notionrs_webhooks/`: Verifies Notion webhook signatures and deserializes webhook events.

### Internal Dependencies

- `notionrs` ← `notionrs_types`
- `notionrs` ← `notionrs_macro`
- `notionrs_types` ← `notionrs_macro`
- `notionrs_webhooks` ← `notionrs_types`

### Other Files

- `scripts/check-upstream-release`: Checks releases in the `makenotion/notion-sdk-js` repository to stay up to date with API changes. If new releases are detected, create an issue in this repository.

## MSRV

MSRV (Minimum Supported Rust Version) is specified in `Cargo.toml` and `rust-toolchain.toml`.

## Git Tag Rules

- `notionrs` → without prefix (e.g., v0.1.0)
- `notionrs_macro`, `notionrs_types`, `notionrs_webhooks` → with prefix (e.g., macro-v0.1.0, webhooks-v0.2.0)
