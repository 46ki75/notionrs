# notionrs_types

Structs for serializing Notion API requests and deserializing Notion API responses. This crate contains only structs.

## Directory Structure

- `src/macro/`: Macros used internally.
- `src/object/`: Main structs for serializing/deserializing requests/responses.
- `src/serde/`: Custom serializer/deserializer for serde.
- `src/prelude.ts`: Re-exports all objects.

## Testing

### Unit Tests

To run unit tests, run the following command:

```bash
cargo test
```

### Integration Tests

There are no integration tests.
