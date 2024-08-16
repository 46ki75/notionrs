# Integration Tests

Integration tests are written within the `integration_tests` module and stored in the `/tests/` directory.

## Integration Test Strategy

Integration tests **send actual requests** to Notion API endpoints. Therefore, you need to prepare a workspace before running the tests.

::: info
Since `notionrs` supports CRUD operations, it's possible to automate all tests. However, detecting issues across multiple methods can be challenging, so we follow a strategy that includes manual preparation.

In the future, we plan to create a test that accepts a single `page_id` and API secret to comprehensively execute all methods and handle cleanup. This will simplify the testing process.
:::

## Running Integration Tests

First, create a `.env` file in the root directory and set the `NOTION_TOKEN`.

```ini
NOTION_TOKEN=secret_XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

::: info
You can also export the `NOTION_TOKEN` environment variable or pass it directly as a shell variable when running the tests.
:::

To run only the integration tests, you can use the following command, specifying the `integration_tests` module:

```bash
cargo test integration_tests
```

This command will execute all tests defined within the `integration_tests` module.

// TODO: docs: Document the preparation of a Notion workspace for integration tests
