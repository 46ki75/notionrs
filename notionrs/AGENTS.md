# notionrs

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

There are no unit tests. You can check for compilation errors with the following command:

```bash
cargo check
```

### Integration Tests

#### Readonly Integration Tests

Read-only integration tests can be run by AI agents.

```bash
cargo test --test integration_test_readonly
```

#### Mutable Integration Tests

Mutable integration tests MUST NOT be run by AI agents.

```bash
cargo test --test integration_test_mutable
```
