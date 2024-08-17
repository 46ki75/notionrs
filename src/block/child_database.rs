use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#child-database>
///
/// Child database block objects contain the following
/// information within the child_database property:
#[derive(Deserialize, Serialize, Debug)]
pub struct ChildDatabaseBlock {
    /// The plain text title of the database.
    pub title: String,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::ChildDatabaseBlock;

    #[test]
    fn deserialize_block_child_database() {
        let json_data = r#"
        {
            "title": "My Database"
        }
        "#;

        let child_database = serde_json::from_str::<ChildDatabaseBlock>(json_data).unwrap();

        assert_eq!(child_database.title, "My Database")
    }
}
