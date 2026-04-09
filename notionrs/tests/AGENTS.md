# Integration Tests

- Page: "`46ki75/notionrs` Integration Test Page Root"
  - Full Page Database: "Read-only: Integration Test"
  - Full Page Database: "Mutable: Integration Test"

## Directory Structure

- `mutable/`: Integration tests that include mutable operations.
- `readonly/`: Integration tests that don't update Notion data.
- `integration_test_mutable.rs`: Entry point of mutable integration tests.
- `integration_test_readonly.rs`: Entry point of read-only integration tests.

## Readonly

```bash
cargo test --test integration_test_readonly
```

- Notion database name: "Read-only: Integration Test"
- Notion API key name: `integration-test-readonly`
- `.env` file path: `notionrs/.env.readonly`
- Capabilities
  - Content capabilities
    - [x] Read content
    - [ ] Update content
    - [ ] Insert content
  - Comment capabilities
    - [x] Read comments
    - [ ] Insert comments
  - User capabilities
    - [ ] No user information
    - [x] Read user information without email addresses
    - [ ] Read user information including email addresses

## Mutable

```bash
cargo test --test integration_test_mutable --jobs 1
```

- Notion database name: "Mutable: Integration Test"
- Notion API key name: `integration-test-mutable`
- `.env` file path: `notionrs/.env.mutable`
- Capabilities
  - Content capabilities
    - [x] Read content
    - [x] Update content
    - [x] Insert content
  - Comment capabilities
    - [x] Read comments
    - [x] Insert comments
  - User capabilities
    - [ ] No user information
    - [ ] Read user information without email addresses
    - [x] Read user information including email addresses
