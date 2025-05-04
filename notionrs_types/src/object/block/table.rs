use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#table>
///
/// Table block objects are parent blocks for table row children.
/// Table block objects contain the following fields within the table property:
#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
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

    /// Only `table_row` can be specified.
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl From<u16> for TableBlock {
    fn from(table_width: u16) -> Self {
        Self::default().table_width(table_width)
    }
}

impl std::fmt::Display for TableBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "TableBlock {{ table_width: {}, has_column_header: {}, has_row_header: {} }}",
            self.table_width, self.has_column_header, self.has_row_header
        )
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
