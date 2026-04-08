# notionrs

## Directory Structure

- `notionrs/examples/`: Code examples for crate users.
- `notionrs/src/`: Main library source.
- `notionrs/tests/`: Integration tests.

## Testing

### Unit Tests

There are no unit tests. You can check for compilation errors with the following command:

```bash
cargo check
```

### Integration Tests

You need to create a `.env` file at repository root, and AI agents must not run integration tests without human approval.

```ini
NOTION_TOKEN="ntn_***"
NOTION_IT_SANDBOX_ID="UUIDv8"
NOTION_IT_DATA_SOURCE_ID="UUIDv8"
NOTION_IT_DATABASE_ID="UUIDv8"
NOTION_IT_CRUD_PAGE_ID="UUIDv8"
NOTION_IT_MARKDOWN_PAGE_ID="UUIDv8"
```

To run the integration tests, run the following command:

```bash
RUST_TEST_THREADS=1 cargo test
```

Integration tests take a long time. We recommend running only the specific test case you need:

```bash
RUST_TEST_THREADS=1 cargo test [TESTNAME]
```
