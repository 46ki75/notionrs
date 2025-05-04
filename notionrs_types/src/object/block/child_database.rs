use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#child-database>
///
/// To create or update child_database type blocks,
/// use the [Create a database](https://developers.notion.com/reference/create-a-database)
/// and the [Update a database](https://developers.notion.com/reference/update-a-database) endpoints,
/// specifying the ID of the parent page in the parent body param.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChildDatabaseBlock {
    /// The plain text title of the database.
    pub title: String,
}

impl std::fmt::Display for ChildDatabaseBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
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
