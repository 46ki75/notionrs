# Integration Tests

- Notion Page: "`46ki75/notionrs` Integration Test Page Root"
  - Notion Full Page Database: "Read-only: Integration Test"
  - Notion Full Page Database: "Mutable: Integration Test"

## Directory Structure

- `mutable/`: Integration tests that include mutable operations.
- `mutable/block/crud_<BLOCK_TYPE>_block.rs`: Creates, reads, updates, and deletes each block type. Parent pages are prepared and independent.
- `mutable/comment/create_comment.rs`: Creates a comment on a prepared page.
- `mutable/data_source/crud_data_source.rs`: Creates a database on a prepared page, adds a data source to the database, and then deletes both the database and the data source.
- `mutable/database/create_database.rs`: Creates a database on a prepared page.
- `mutable/database/crud_database.rs`: Creates a database on a prepared page, updates it, and then deletes it.
- `mutable/file_upload/upload_file.rs`: Uploads a file and creates a block using it.
- `mutable/page/create_page_on_data_source.rs`: Creates a page on the data source.
- `mutable/page/crud_page.rs`: Creates, reads, updates, and deletes a page on a prepared page.
- `mutable/page/move_page.rs`: Creates two pages on a prepared page and moves the created page to another one.
- `readonly/`: Integration tests that don't update Notion data.
- `readonly/block/get_<BLOCK_TYPE>_block.rs`: Gets each block type.
- `readonly/comment/retrieve_comments.rs`: Gets comments from a prepared page.
- `readonly/custom_emojis/list_custom_emojis.rs`: Gets all emojis in the Notion workspace.
- `readonly/data_source/list_data_source_templates.rs`: Lists data source templates.
- `readonly/data_source/query_data_source.rs`: Queries a data source for a single page of results.
- `readonly/data_source/query_data_source_all.rs`: Queries a data source and fetches all paginated results.
- `readonly/data_source/query_data_source_with_struct.rs`: Queries a data source into a custom Rust struct.
- `readonly/data_source/retrieve_data_source.rs`: Gets a specific data source.
- `readonly/database/retrieve_database.rs`: Gets a specific database.
- `readonly/file_upload/list_file_upload.rs`: Lists file uploads in the workspace.
- `readonly/page/get_page_property_item.rs`: Gets a specific property item from a page.
- `readonly/page/get_page.rs`: Gets a specific page.
- `readonly/user/get_self.rs`: Gets the bot's own user object.
- `readonly/user/get_user.rs`: Gets a specific workspace user by ID.
- `readonly/user/get_users.rs`: Gets all users in the workspace.
- `readonly/search.rs`: Searches the workspace for pages or databases.
- `readonly/to_markdown.rs`: Converts Notion objects to Markdown.
- `data_source_schema.rs`: Defines a schema for the prepared data source. Note: Some data source properties aren't supported for creation via the Notion API.
- `integration_test_mutable.rs`: Entry point of mutable integration tests.
- `integration_test_readonly.rs`: Entry point of read-only integration tests.

## Readonly

Read-only integration tests can be run by AI agents.

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

Mutable integration tests MUST NOT be run by AI agents.

```bash
cargo test --test integration_test_mutable
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
