use serde::{Deserialize, Serialize};

use crate::rich_text;

pub mod audio;
pub mod bookmark;
pub mod bulleted_list_item;
pub mod callout;
pub mod child_database;
pub mod child_page;
pub mod code;
pub mod column;
pub mod column_list;
pub mod embed;
pub mod equation;
pub mod file;
pub mod heading;
pub mod image;
pub mod link_preview;
pub mod numbered_list_item;
pub mod paragraph;
pub mod pdf;
pub mod quote;
pub mod synced_block;
pub mod table;
pub mod table_row;
pub mod template;
pub mod to_do;
pub mod toggle;
pub mod video;

/// ```json
/// {
///         "object": "block",
///         "id": "b943dc57-3260-4486-a1c8-f83cf8c12fc3",
///         "parent": {
///                 "type": "database_id",
///                 "database_id": "e287685c-fe92-43e6-8803-9d1733ae9986"
///         },
///         "created_time": "2024-07-09T18:33:00.000Z",
///         "last_edited_time": "2024-08-15T06:18:00.000Z",
///         "created_by": {
///                 "object": "user",
///                 "id": "570aad4f-f114-4309-882f-baaa3cd55e95"
///         },
///         "last_edited_by": {
///                 "object": "user",
///                 "id": "570aad4f-f114-4309-882f-baaa3cd55e95"
///         },
///         "has_children": true,
///         "archived": false,
///         "in_trash": false,
///         "type": "child_page",
///         "child_page": {
///                 "title": "My Page Title"
///         },
///         "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
///         "request_id": "8a67eed6-3a1b-4e8c-90cc-ea87539ef9bc"
/// }
/// ```
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockResponse {
    pub object: String,

    pub id: String,

    pub parent: crate::others::parent::Parent,

    pub created_time: String,

    pub last_edited_time: String,

    pub created_by: crate::user::User,

    pub last_edited_by: crate::user::User,

    pub has_children: bool,

    pub archived: bool,

    pub in_trash: bool,

    #[serde(flatten)]
    pub details: Block,
}

/// <https://developers.notion.com/reference/block#block-type-objects>
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Audio(audio::AudioBlock),
    Bookmark {
        bookmark: bookmark::BookmarkBlock,
    },
    Breadcrumb {
        breadcrumb: std::collections::HashMap<(), ()>,
    },
    BulletedListItem {
        bulleted_list_item: bulleted_list_item::BulletedListItemBlock,
    },
    Callout {
        callout: callout::CalloutBlock,
    },
    ChildDatabase {
        child_database: child_database::ChildDatabaseBlock,
    },
    ChildPage {
        child_page: child_page::ChildPageBlock,
    },
    Code {
        code: code::CodeBlock,
    },
    ColumnList {
        column_list: column_list::ColumnListBlock,
    },
    Column {
        column: column::ColumnBlock,
    },
    Divider {
        divider: std::collections::HashMap<(), ()>,
    },
    Embed {
        embed: embed::EmbedBlock,
    },
    Equation {
        equation: equation::EquationBlock,
    },
    File(file::FileBlock),
    #[serde(rename = "heading_1")]
    Heading1 {
        heading_1: heading::HeadingBlock,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        heading_2: heading::HeadingBlock,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        heading_3: heading::HeadingBlock,
    },
    Image(image::ImageBlock),
    LinkPreview {
        link_preview: link_preview::LinkPreviewBlock,
    },
    NumberedListItem {
        numbered_list_item: numbered_list_item::NumberedListItemBlock,
    },
    Paragraph {
        paragraph: paragraph::ParagraphBlock,
    },
    Pdf(pdf::PdfBlock),
    Quote {
        quote: quote::QuoteBlock,
    },
    SyncedBlock {
        synced_block: synced_block::SyncedBlock,
    },
    Table {
        table: table::TableBlock,
    },
    TableRow {
        table_row: table_row::TableRowBlock,
    },
    Template {
        template: template::TemplateBlock,
    },
    ToDo {
        to_do: to_do::ToDoBlock,
    },
    Toggle {
        toggle: toggle::ToggleBlock,
    },
    Video(video::VideoBlock),
    Unknown(serde_json::Value),
}

impl Block {
    // AudioBlock

    pub fn new_audio() -> audio::AudioBlock {
        audio::AudioBlock::new()
    }

    pub fn new_audio_with_url<T>(url: T) -> audio::AudioBlock
    where
        T: AsRef<str>,
    {
        audio::AudioBlock::new().url(url.as_ref())
    }

    // BookmarkBlock

    pub fn new_bookmark() -> bookmark::BookmarkBlock {
        bookmark::BookmarkBlock::new()
    }

    pub fn new_bookmark_with_url<T>(url: T) -> bookmark::BookmarkBlock
    where
        T: AsRef<str>,
    {
        bookmark::BookmarkBlock::new().url(url)
    }

    // BulletedListItemBlock

    pub fn new_bulleted_list_item() -> bulleted_list_item::BulletedListItemBlock {
        bulleted_list_item::BulletedListItemBlock::new()
    }

    pub fn new_bulleted_list_item_with_children(
        children: Vec<Block>,
    ) -> bulleted_list_item::BulletedListItemBlock {
        bulleted_list_item::BulletedListItemBlock::new().children(children)
    }

    pub fn new_bulleted_list_item_with_rich_text<T>(
        rich_text: Vec<crate::others::rich_text::RichText>,
    ) -> bulleted_list_item::BulletedListItemBlock {
        bulleted_list_item::BulletedListItemBlock::new().rich_text(rich_text)
    }

    pub fn new_bulleted_list_item_with_plain_text<T>(
        plain_text: T,
    ) -> bulleted_list_item::BulletedListItemBlock
    where
        T: AsRef<str>,
    {
        bulleted_list_item::BulletedListItemBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // CalloutBlock

    pub fn new_callout() -> callout::CalloutBlock {
        callout::CalloutBlock::new()
    }

    pub fn new_callout_with_plain_text<T>(plain_text: T) -> callout::CalloutBlock
    where
        T: AsRef<str>,
    {
        callout::CalloutBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // BreadcrumbBlock

    pub fn build_breadcrumb() -> Block {
        Block::Breadcrumb {
            breadcrumb: std::collections::HashMap::new(),
        }
    }

    // CodeBlock

    pub fn new_code() -> code::CodeBlock {
        code::CodeBlock::new()
    }

    pub fn new_code_with_plain_text<T>(plain_text: T) -> code::CodeBlock
    where
        T: AsRef<str>,
    {
        code::CodeBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // ColumnListBlock

    pub fn new_column_list() -> column_list::ColumnListBlock {
        column_list::ColumnListBlock::new()
    }

    pub fn new_column_list_with_children_columns(
        children: Vec<column::ColumnBlock>,
    ) -> column_list::ColumnListBlock {
        column_list::ColumnListBlock::new().children(
            children
                .into_iter()
                .map(|column_block| column_block.build())
                .collect(),
        )
    }

    // ColumnBlock

    pub fn new_column() -> column::ColumnBlock {
        column::ColumnBlock::new()
    }

    pub fn new_column_with_children(children: Vec<Block>) -> column::ColumnBlock {
        column::ColumnBlock::new().children(children)
    }

    // DividerBlock

    pub fn build_divider() -> Block {
        Block::Divider {
            divider: std::collections::HashMap::new(),
        }
    }

    // EmbedBlock

    pub fn new_embed() -> embed::EmbedBlock {
        embed::EmbedBlock::new()
    }

    pub fn new_embed_with_url<T>(url: T) -> embed::EmbedBlock
    where
        T: AsRef<str>,
    {
        embed::EmbedBlock::new().url(url)
    }

    // EquationBlock

    pub fn new_equation() -> equation::EquationBlock {
        equation::EquationBlock::new()
    }

    pub fn new_equation_with_expression<T>(expression: T) -> equation::EquationBlock
    where
        T: AsRef<str>,
    {
        equation::EquationBlock::new().expression(expression)
    }

    // FileBlock

    pub fn new_file() -> file::FileBlock {
        file::FileBlock::new()
    }

    pub fn new_file_with_url<T>(url: T) -> file::FileBlock
    where
        T: AsRef<str>,
    {
        file::FileBlock::new().url(url)
    }

    // HeadingBlock

    pub fn new_heading() -> heading::HeadingBlock {
        heading::HeadingBlock::new()
    }

    pub fn new_heading_with_plain_text<T>(plain_text: T) -> heading::HeadingBlock
    where
        T: AsRef<str>,
    {
        heading::HeadingBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // ImageBlock

    pub fn new_image() -> image::ImageBlock {
        image::ImageBlock::new()
    }

    pub fn new_image_with_url<T>(url: T) -> image::ImageBlock
    where
        T: AsRef<str>,
    {
        image::ImageBlock::new().url(url)
    }

    // NumberedListItemBlock

    pub fn new_numbered_list_item() -> numbered_list_item::NumberedListItemBlock {
        numbered_list_item::NumberedListItemBlock::new()
    }

    pub fn new_numbered_list_item_with_children(
        children: Vec<Block>,
    ) -> numbered_list_item::NumberedListItemBlock {
        numbered_list_item::NumberedListItemBlock::new().children(children)
    }

    pub fn new_numbered_list_item_with_rich_text<T>(
        rich_text: Vec<crate::others::rich_text::RichText>,
    ) -> numbered_list_item::NumberedListItemBlock {
        numbered_list_item::NumberedListItemBlock::new().rich_text(rich_text)
    }

    pub fn new_numbered_list_item_with_plain_text<T>(
        plain_text: T,
    ) -> numbered_list_item::NumberedListItemBlock
    where
        T: AsRef<str>,
    {
        numbered_list_item::NumberedListItemBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // ParagraphBlock

    pub fn new_paragraph() -> paragraph::ParagraphBlock {
        paragraph::ParagraphBlock::new()
    }

    pub fn new_paragraph_with_rich_text(
        rich_text: Vec<crate::others::rich_text::RichText>,
    ) -> paragraph::ParagraphBlock {
        paragraph::ParagraphBlock::new().rich_text(rich_text)
    }

    pub fn new_paragraph_with_plain_text<T>(plain_text: T) -> paragraph::ParagraphBlock
    where
        T: AsRef<str>,
    {
        paragraph::ParagraphBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // PdfBlock

    pub fn new_pdf() -> pdf::PdfBlock {
        pdf::PdfBlock::new()
    }

    pub fn new_pdf_with_url<T>(url: T) -> pdf::PdfBlock
    where
        T: AsRef<str>,
    {
        pdf::PdfBlock::new().url(url)
    }

    // QuoteBlock

    pub fn new_quote() -> quote::QuoteBlock {
        quote::QuoteBlock::new()
    }

    pub fn new_quote_with_children(children: Vec<Block>) -> quote::QuoteBlock {
        quote::QuoteBlock::new().children(children)
    }

    pub fn new_quote_with_rich_text(
        rich_text: Vec<crate::others::rich_text::RichText>,
    ) -> quote::QuoteBlock {
        quote::QuoteBlock::new().rich_text(rich_text)
    }

    pub fn new_quote_with_plain_text<T>(plain_text: T) -> quote::QuoteBlock
    where
        T: AsRef<str>,
    {
        quote::QuoteBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // SyncedBlock

    pub fn new_synced_block() -> synced_block::SyncedBlock {
        synced_block::SyncedBlock::new()
    }

    pub fn new_synced_block_whti_block_id<T>(block_id: T) -> synced_block::SyncedBlock
    where
        T: AsRef<str>,
    {
        synced_block::SyncedBlock::new().block_id(block_id)
    }

    // TableRowBlock

    pub fn new_table_row() -> table_row::TableRowBlock {
        table_row::TableRowBlock::new()
    }

    pub fn new_table_row_with_cells(
        cells: Vec<Vec<crate::others::rich_text::RichText>>,
    ) -> table_row::TableRowBlock {
        table_row::TableRowBlock::new().cells(cells)
    }

    // TableBlock

    pub fn new_table() -> table::TableBlock {
        table::TableBlock::new()
    }

    pub fn new_table_with_table_rows(
        table_rows: Vec<table_row::TableRowBlock>,
    ) -> table::TableBlock {
        table::TableBlock::new().children(
            table_rows
                .into_iter()
                .map(|table_row| table_row.build())
                .collect(),
        )
    }

    // ToDoBlock

    pub fn new_to_do() -> to_do::ToDoBlock {
        to_do::ToDoBlock::new()
    }

    // ToggleBlock

    pub fn new_toggle() -> toggle::ToggleBlock {
        toggle::ToggleBlock::new()
    }

    pub fn new_toggle_with_summary_rich_text(
        rich_text: Vec<crate::others::rich_text::RichText>,
    ) -> toggle::ToggleBlock {
        toggle::ToggleBlock::new().rich_text(rich_text)
    }

    pub fn new_toggle_with_summary_plain_text<T>(plain_text: T) -> toggle::ToggleBlock
    where
        T: AsRef<str>,
    {
        toggle::ToggleBlock::new().rich_text(vec![rich_text!(plain_text)])
    }

    // VideoBlock

    pub fn new_video() -> video::VideoBlock {
        video::VideoBlock::new()
    }

    pub fn new_video_with_url<T>(url: T) -> video::VideoBlock
    where
        T: AsRef<str>,
    {
        video::VideoBlock::new().url(url)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_block() {
        let json_data = r#"
        {
            "object": "block",
            "id": "b943dc57-3260-4486-a1c8-f83cf8c12fc3",
            "parent": {
                "type": "page_id",
                "page_id": "8a67eed6-3a1b-4e8c-90cc-ea87539ef9bc"
            },
            "created_time": "2024-08-17T02:50:00.000Z",
            "last_edited_time": "2024-08-17T02:50:00.000Z",
            "created_by": {
                "object": "user",
                "id": "21d48f75-3f61-4c36-8b24-7447ca72fb3a"
            },
            "last_edited_by": {
                "object": "user",
                "id": "21d48f75-3f61-4c36-8b24-7447ca72fb3a"
            },
            "has_children": false,
            "archived": false,
            "in_trash": false,
            "type": "bookmark",
            "bookmark": {
                "caption": [
                    {
                        "type": "text",
                        "text": {
                            "content": "example domain",
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
                        "plain_text": "example domain",
                        "href": null
                    }
                ],
                "url": "https://example.com"
            }
        }
        "#;

        let block = serde_json::from_str::<BlockResponse>(json_data).unwrap();

        assert_eq!(block.object, "block");
        assert_eq!(block.id, "b943dc57-3260-4486-a1c8-f83cf8c12fc3");

        match block.parent {
            crate::others::parent::Parent::PageParent(parent) => {
                assert_eq!(parent.r#type, "page_id");
                assert_eq!(parent.page_id, "8a67eed6-3a1b-4e8c-90cc-ea87539ef9bc");
            }
            _ => panic!(),
        }

        assert_eq!(block.created_time, "2024-08-17T02:50:00.000Z");
        assert_eq!(block.last_edited_time, "2024-08-17T02:50:00.000Z");

        match block.created_by {
            crate::user::User::Bot(bot) => {
                assert_eq!(bot.object, "user");
                assert_eq!(bot.id, "21d48f75-3f61-4c36-8b24-7447ca72fb3a");
            }
            crate::user::User::Person(person) => {
                assert_eq!(person.object, "user");
                assert_eq!(person.id, "21d48f75-3f61-4c36-8b24-7447ca72fb3a");
            }
        }

        assert!(!block.has_children);
        assert!(!block.archived);
        assert!(!block.in_trash);

        match block.details {
            Block::Bookmark { bookmark } => {
                assert_eq!(bookmark.url, "https://example.com");

                let rich_text = bookmark.caption.first().unwrap();

                assert_eq!(rich_text.plain_text, "example domain");
                assert_eq!(rich_text.href, None);
            }
            _ => panic!("Unexpected block type!"),
        }
    }

    #[test]
    fn deserialize_block_file() {
        let json_data = r#"
        {
            "object": "block",
            "id": "9d6fe494-cbf5-4971-bb5f-f7934fadec49",
            "parent": {
                "type": "page_id",
                "page_id": "6669fc5c-8483-4560-9810-06de1873c7cb"
            },
            "created_time": "2024-08-17T05:45:00.000Z",
            "last_edited_time": "2024-08-17T05:46:00.000Z",
            "created_by": {
                "object": "user",
                "id": "f388d469-3ad9-4127-86c7-48c8972af3a6"
            },
            "last_edited_by": {
                "object": "user",
                "id": "f388d469-3ad9-4127-86c7-48c8972af3a6"
            },
            "has_children": false,
            "archived": false,
            "in_trash": false,
            "type": "file",
            "file": {
                "caption": [],
                "type": "file",
                "file": {
                    "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                    "expiry_time": "2024-08-17T06:46:12.698Z"
                },
                "name": "2024-07-18 202106.png"
            }
        }
        "#;

        let block = serde_json::from_str::<BlockResponse>(json_data).unwrap();

        match block.details {
            Block::File(file) => match file.file {
                crate::others::file::File::Uploaded(uploaded_file) => {
                    assert_eq!(
                        uploaded_file.name,
                        Some("2024-07-18 202106.png".to_string())
                    )
                }
                crate::others::file::File::External(_) => panic!("Unexpected!"),
            },
            _ => panic!("Unexpected!"),
        }
    }

    #[test]
    fn deserialize_block_image() {
        let json_data = r#"
        {
            "object": "block",
            "id": "8ff6e7d3-ea3c-4c21-b3e4-db53df0530e9",
            "parent": {
                "type": "page_id",
                "page_id": "12eb8c08-9d6d-4b3f-9a8f-80de613de2ed"
            },
            "created_time": "2024-08-17T06:01:00.000Z",
            "last_edited_time": "2024-08-17T06:01:00.000Z",
            "created_by": {
                "object": "user",
                "id": "bb7ef91a-d553-447a-90de-00f56c06caa5"
            },
            "last_edited_by": {
                "object": "user",
                "id": "bb7ef91a-d553-447a-90de-00f56c06caa5"
            },
            "has_children": false,
            "archived": false,
            "in_trash": false,
            "type": "image",
            "image": {
                "caption": [],
                "type": "file",
                "file": {
                    "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                    "expiry_time": "2024-08-17T07:01:49.743Z"
                }
            }
        }
        "#;

        let block = serde_json::from_str::<BlockResponse>(json_data).unwrap();

        println!("{:?}", block);

        match block.details {
            Block::Image(image) => match image.image {
                crate::others::file::File::Uploaded(uploaded_file) => {
                    assert_eq!(
                        uploaded_file.file.url,
                        "https://prod-files-secure.s3.us-west-2.amazonaws.com/"
                    )
                }
                crate::others::file::File::External(_) => panic!("Unexpected!"),
            },
            _ => panic!("Unexpected!"),
        }
    }
}
