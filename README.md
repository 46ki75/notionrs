To export environment variables, write them in the `.env` file as follows.

```ini
NOTION_API_KEY=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

## Integration Test

Write integration tests for each method in the same file and add the **`integration_test_`** prefix to the test functions.

To perform testing while outputting to standard output, do the following.

```bash
RUST_TEST_THREADS=1 cargo test integration_test -- --nocapture --ignored
```

To perform all integration tests, write the following in the .env file.

```ini
NOTION_API_KEY=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
NOTION_USER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```
