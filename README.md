To export environment variables, write them in the `.env` file as follows.

```ini
NOTION_TOKEN=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

## Integration Test

To perform testing while outputting to standard output, do the following.

```bash
cargo test integration_tests
cargo test integration_tests -- --nocapture
```

- `RUST_TEST_THREADS=1`: Sets the number of concurrent test threads to 1. This is to ensure you can check the values in the standard output.
- `integration_test`: Runs only the tests that start with the `integration_test` prefix.
- `--`: Used to separate the options of the `cargo test` command from the options of the test runner (the program that actually runs the tests).
- `--nocapture`: Prevents capturing of standard output and standard error, allowing the output to be visible during test execution.
- `--ignored`: Runs only the tests that are marked with the `#[ignore]` attribute.

To perform all integration tests, write the following in the .env file.

```ini
NOTION_TOKEN=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
NOTION_USER_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
NOTION_PAGE_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
NOTION_DATABASE_ID=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```
