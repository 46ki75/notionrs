use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#table>
///
/// Table block objects are parent blocks for table row children.
/// Table block objects contain the following fields within the table property:
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Default)]
pub struct TableBlock {
    /// The number of columns in the table.
    /// Note that this cannot be changed via the public API once a table is created.
    pub table_width: u16,

    /// Whether the table has a column header. If true,
    /// then the first row in the table appears visually distinct from the other rows.
    pub has_column_header: bool,

    /// Whether the table has a header row. If true,
    /// then the first column in the table appears visually distinct from the other columns.
    pub has_row_header: bool,
}

impl TableBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Table { table: self }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::TableBlock;

    #[test]
    fn deserialize_block_table() {
        let json_data = r#"
        {
            "table_width": 2,
            "has_column_header": false,
            "has_row_header": false
        }
        "#;

        let table = serde_json::from_str::<TableBlock>(json_data).unwrap();

        assert_eq!(table.table_width, 2);
        assert!(!table.has_column_header);
        assert!(!table.has_row_header);
    }
}
