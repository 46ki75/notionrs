# Integration Tests

- Page: "`46ki75/notionrs` Integration Test Page Root"
  - Full Page Database: "Read-only: Integration Test"
  - Full Page Database: "Mutable: Integration Test"

## Directory Structure

- `mutable/`: Integration tests that include mutable operations.
- `mutable/block/crud_*_block.rs`: Creates, reads, updates, and deletes each block type. Parent pages are prepared and independent.
- `mutable/comment/create_comment.rs`: Creates a comment on a prepared page.
- `mutable/data_source/crud_data_source.rs`: Creates a database on a prepared page, adds a data source to the database, and then deletes both the database and the data source.
- `mutable/database/create_database.rs`: Creates a database on a prepared page.
- `mutable/database/crud_database.rs`: Creates a database on a prepared page, updates it, and then deletes it.
- `mutable/file_upload/upload_file.rs`: Uploads a file and creates a block using it.
- `mutable/page/create_page_on_data_source.rs`: Creates a page on the data source.
- `mutable/page/crud_page.rs`: Creates, reads, updates, and deletes a page on a prepared page.
- `mutable/page/move_page.rs`: Creates two pages on a prepared page and moves the created page to another one.
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
