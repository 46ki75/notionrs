use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#table-rows>
///
/// Follow these steps to fetch the table_rows of a table:
///
/// 1. Get the table ID from a query to Retrieve block children for the parent page.
/// 2. Get the table_rows from a query to Retrieve block children for the table.
///
/// A table_row block object contains the following fields within the table_row property:
#[derive(Deserialize, Serialize, Debug, Clone, Default, notionrs_macro::Setter)]
pub struct TableRowBlock {
    /// An array of cell contents in horizontal display order.
    /// Each cell is an array of rich text objects.
    pub cells: Vec<Vec<crate::object::rich_text::RichText>>,
}

impl std::fmt::Display for TableRowBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .map(|t| { t.iter().map(|t| t.to_string()).collect::<String>() })
                .collect::<String>()
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

    use super::TableRowBlock;

    #[test]
    fn deserialize_block_table_row() {
        let json_data = r#"
        {
            "cells": [
                [
                    {
                        "type": "text",
                        "text": {
                            "content": "A1-thead",
                            "link": null
                        },
                        "annotations": {
                            "bold": false,
                            "italic": false,
                            "strikethrough": false,
                            "underline": false,
                            "code": false,
                            "color": "default"
                        },
                        "plain_text": "A1-thead",
                        "href": null
                    }
                ],
                [
                    {
                        "type": "text",
                        "text": {
                            "content": "B1-thead",
                            "link": null
                        },
                        "annotations": {
                            "bold": false,
                            "italic": false,
                            "strikethrough": false,
                            "underline": false,
                            "code": false,
                            "color": "default"
                        },
                        "plain_text": "B1-thead",
                        "href": null
                    }
                ]
            ]
        }
        "#;

        let table_rows = serde_json::from_str::<TableRowBlock>(json_data).unwrap();

        let cell = table_rows.cells.first().unwrap().first().unwrap();

        match cell {
            crate::object::rich_text::RichText::Text { plain_text, .. } => {
                assert_eq!(plain_text, "A1-thead");
            }
            _ => panic!(),
        }
    }
}
