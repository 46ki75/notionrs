# notionrs

Documentation for crate developers (not for crate users — see the top-level README / API docs for that).

## Directory Structure

- `notionrs/examples/`: Code examples for crate users.
- `notionrs/src/`: Main library source.
- `notionrs/tests/`: Integration tests.

## Building

```bash
cargo build
```

## Testing

### Unit Tests

Unit tests live alongside the code in `src/` (`#[cfg(test)] mod tests` / `mod unit_tests` blocks).

```bash
cargo test --lib
```

### Integration Tests

If you create a new client method, you should add integration tests.

To run integration tests, create a `.env` file in this directory (`notionrs/`, not the workspace root) with:

- `NOTION_API_KEY_READONLY`: Required to run read-only integration tests.
- `NOTION_API_KEY_MUTABLE`: Required to run mutable integration tests.

#### Mutable Integration Tests

Mutable integration tests may be run by AI agents when working in a sandboxed environment. Do not run them against a production Notion workspace.

```bash
cargo test --test integration_test_mutable
```

##### Mutable Capabilities

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

#### Readonly Integration Tests

Read-only integration tests can be run by AI agents.

```bash
cargo test --test integration_test_readonly
```

##### Readonly Capabilities

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

### Directory Structure (Integration Tests)

- `mutable/`: Integration tests that include mutable operations.
- `mutable/block/crud_<BLOCK_TYPE>_block.rs`: Creates, reads, updates, and deletes each block type. Parent pages are prepared and independent.
- `mutable/comment/create_comment.rs`: Creates a comment on a prepared page.
- `mutable/data_source/crud_data_source.rs`: Creates a database on a prepared page, adds a data source to the database, and then deletes both the database and the data source.
- `mutable/database/create_database.rs`: Creates a database on a prepared page.
- `mutable/database/crud_database.rs`: Creates a database on a prepared page, updates it, and then deletes it.
- `mutable/file_upload/upload_file.rs`: Uploads a file and creates a block using it.
- `mutable/page/create_page_allow_async.rs`: Creates a page with `allow_async(true)` on a prepared data source; handles both the synchronous page response and the async task response (polling `get_async_task` until it resolves).
- `mutable/page/create_page_on_data_source.rs`: Creates a page on the data source.
- `mutable/page/crud_page.rs`: Creates, reads, updates, and deletes a page on a prepared page.
- `mutable/page/move_page.rs`: Creates two pages on a prepared page and moves the created page to another one.
- `mutable/page/update_page_markdown_allow_async.rs`: Updates a page's markdown with `allow_async(true)`; handles both the synchronous and async task response.
- `readonly/`: Integration tests that don't update Notion data.
- `readonly/async_task.rs`: Exercises `get_async_task`'s error path with an unknown task ID (there's no way to deterministically obtain a real task ID without a mutable async operation).
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
