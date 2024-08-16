# Unit Tests

Unit tests primarily focus on verifying the deserialization of JSON.

These tests are written within `unit_tests` modules in the files where each `struct` is defined.

### Example of a Unit Test

```rust
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_page_checkbox_property() {
        let json_data = r#"
        {
            "Task completed": {
                "id": "ZI%40W",
                "type": "checkbox",
                "checkbox": true
            }
        }
        "#;

        let checkbox_map = serde_json::from_str::<
            std::collections::HashMap<String, PageCheckboxProperty>,
        >(json_data)
        .unwrap();

        let checkbox = checkbox_map.get("Task completed").unwrap();

        assert_eq!(checkbox.id, "ZI%40W".to_string());
        assert!(checkbox.checkbox);
    }
}
```

### Running the Tests

To run only the unit tests within the `unit_tests` modules, use the following command, specifying the `unit_tests` module:

```bash
cargo test unit_tests
```

This command will execute all tests defined within the `unit_tests` modules.

::: warning
The integration tests under the `tests/` directory make actual API requests. Therefore, you need to have a Notion workspace set up to run the integration tests. (Details will be explained on the integration tests page.)
:::
