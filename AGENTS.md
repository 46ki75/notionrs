# notionrs

A Notion API client implemented in Rust.

## Directory Structure

### Workspace Members

- `notionrs/`: Main crate. Contains a client that can send API requests to Notion API endpoints.
- `notionrs_macro/`: Provides derive macros. Published to crates.io, but its API is intended for internal use only.
- `notionrs_types/`: Defines structs used in other crates.
- `notionrs_webhooks/`: Verifies Notion webhook signatures and deserializes webhook events.

### Internal Dependencies

- `notionrs` ← `notionrs_types`
- `notionrs` ← `notionrs_macro`
- `notionrs_types` ← `notionrs_macro`
- `notionrs_webhooks` ← `notionrs_types`

### The Others

- `scripts/check-upstream-release`: Checks releases in the `makenotion/notion-sdk-js` repository to stay up to date with API changes. If new releases are detected, create an issue on this repository.
