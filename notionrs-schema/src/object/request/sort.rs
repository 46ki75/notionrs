use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct Sort {
    /// Specify the name of the property
    pub property: String,

    /// Specify the sort order
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl Sort {
    /// Helper function to create an ascending order filter
    /// - property_name: The name of the database property to sort by
    pub fn asc<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            property: property_name.as_ref().to_string(),
            direction: SortDirection::Ascending,
        }
    }

    /// Helper function to create a descending order filter
    /// - property_name: The name of the database property to sort by
    pub fn desc<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            property: property_name.as_ref().to_string(),
            direction: SortDirection::Descending,
        }
    }
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn serialize_database_sort() {
        let sort = Sort::asc("My Property");

        // Expected JSON output
        let expected_json = r#"{
            "property": "My Property",
            "direction": "ascending"
        }"#;

        // Serialize the Sort struct to JSON
        let serialized = serde_json::to_string(&sort).expect("Failed to serialize Sort");

        // Parse the expected JSON to ensure formatting matches
        let expected: serde_json::Value =
            serde_json::from_str(expected_json).expect("Failed to parse expected JSON");
        let serialized_value: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        // Compare the serialized value with the expected value
        assert_eq!(
            serialized_value, expected,
            "Serialized JSON does not match the expected JSON"
        );
    }
}
