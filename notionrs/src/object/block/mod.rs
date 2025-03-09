use serde::{Deserialize, Serialize};

pub use self::bookmark::BookmarkBlock;
pub use self::bulleted_list_item::BulletedListItemBlock;
pub use self::callout::CalloutBlock;
pub use self::child_database::ChildDatabaseBlock;
pub use self::child_page::ChildPageBlock;
pub use self::code::CodeBlock;
pub use self::column::ColumnBlock;
pub use self::column_list::ColumnListBlock;
pub use self::embed::EmbedBlock;
pub use self::equation::EquationBlock;
pub use self::heading::HeadingBlock;
pub use self::link_preview::LinkPreviewBlock;
pub use self::numbered_list_item::NumberedListItemBlock;
pub use self::paragraph::ParagraphBlock;
pub use self::quote::QuoteBlock;
pub use self::synced_block::SyncedBlock;
pub use self::table::TableBlock;
pub use self::table_row::TableRowBlock;
pub use self::template::TemplateBlock;
pub use self::to_do::ToDoBlock;
pub use self::toggle::ToggleBlock;

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
pub mod heading;
pub mod link_preview;
pub mod numbered_list_item;
pub mod paragraph;
pub mod quote;
pub mod synced_block;
pub mod table;
pub mod table_row;
pub mod template;
pub mod to_do;
pub mod toggle;

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

    pub parent: crate::object::parent::Parent,

    /// This value is provided in ISO 8601 format.
    /// To convert it back to the original string,
    /// use the `.to_rfc3339()` method from `chrono`.
    pub created_time: chrono::DateTime<chrono::FixedOffset>,

    /// This value is provided in ISO 8601 format.
    /// To convert it back to the original string,
    /// use the `.to_rfc3339()` method from `chrono`.
    pub last_edited_time: chrono::DateTime<chrono::FixedOffset>,

    pub created_by: crate::object::user::User,

    pub last_edited_by: crate::object::user::User,

    pub has_children: bool,

    pub archived: bool,

    pub in_trash: bool,

    #[serde(flatten)]
    pub block: Block,
}

/// <https://developers.notion.com/reference/block#block-type-objects>
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Audio {
        audio: crate::object::file::File,
    },
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
    File {
        file: crate::object::file::File,
    },
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
    Image {
        image: crate::object::file::File,
    },
    LinkPreview {
        link_preview: link_preview::LinkPreviewBlock,
    },
    NumberedListItem {
        numbered_list_item: numbered_list_item::NumberedListItemBlock,
    },
    Paragraph {
        paragraph: paragraph::ParagraphBlock,
    },
    Pdf {
        pdf: crate::object::file::File,
    },
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
    Video {
        video: crate::object::file::File,
    },
    Unknown(serde_json::Value),
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Block::Audio { audio } => write!(f, "{}", audio),
            Block::Bookmark { bookmark } => write!(f, "{}", bookmark),
            Block::Breadcrumb { breadcrumb } => write!(f, "{:?}", breadcrumb),
            Block::BulletedListItem { bulleted_list_item } => write!(f, "{}", bulleted_list_item),
            Block::Callout { callout } => write!(f, "{}", callout),
            Block::ChildDatabase { child_database } => write!(f, "{}", child_database),
            Block::ChildPage { child_page } => write!(f, "{}", child_page),
            Block::Code { code } => write!(f, "{}", code),
            Block::ColumnList { column_list } => write!(f, "{}", column_list),
            Block::Column { column } => write!(f, "{}", column),
            Block::Divider { divider } => write!(f, "{:?}", divider),
            Block::Embed { embed } => write!(f, "{}", embed),
            Block::Equation { equation } => write!(f, "{}", equation),
            Block::File { file } => write!(f, "{}", file),
            Block::Heading1 { heading_1 } => write!(f, "{}", heading_1),
            Block::Heading2 { heading_2 } => write!(f, "{}", heading_2),
            Block::Heading3 { heading_3 } => write!(f, "{}", heading_3),
            Block::Image { image } => write!(f, "{}", image),
            Block::LinkPreview { link_preview } => write!(f, "{}", link_preview),
            Block::NumberedListItem { numbered_list_item } => write!(f, "{}", numbered_list_item),
            Block::Paragraph { paragraph } => write!(f, "{}", paragraph),
            Block::Pdf { pdf } => write!(f, "{}", pdf),
            Block::Quote { quote } => write!(f, "{}", quote),
            Block::SyncedBlock { synced_block } => write!(f, "{}", synced_block),
            Block::Table { table } => write!(f, "{}", table),
            Block::TableRow { table_row } => write!(f, "{}", table_row),
            Block::Template { template } => write!(f, "{}", template),
            Block::ToDo { to_do } => write!(f, "{}", to_do),
            Block::Toggle { toggle } => write!(f, "{}", toggle),
            Block::Video { video } => write!(f, "{}", video),
            Block::Unknown(value) => write!(f, "{:?}", value),
        }
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {
    use chrono::TimeZone;

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
            crate::object::parent::Parent::PageParent(parent) => {
                assert_eq!(parent.r#type, "page_id");
                assert_eq!(parent.page_id, "8a67eed6-3a1b-4e8c-90cc-ea87539ef9bc");
            }
            _ => panic!(),
        }

        let expected_created_time = chrono::Utc.with_ymd_and_hms(2024, 8, 17, 2, 50, 0).unwrap();
        assert_eq!(block.created_time, expected_created_time);

        let expected_last_edited_time =
            chrono::Utc.with_ymd_and_hms(2024, 8, 17, 2, 50, 0).unwrap();
        assert_eq!(block.last_edited_time, expected_last_edited_time);

        match block.created_by {
            crate::object::user::User::Bot(bot) => {
                assert_eq!(bot.object, "user");
                assert_eq!(bot.id, "21d48f75-3f61-4c36-8b24-7447ca72fb3a");
            }
            crate::object::user::User::Person(person) => {
                assert_eq!(person.object, "user");
                assert_eq!(person.id, "21d48f75-3f61-4c36-8b24-7447ca72fb3a");
            }
        }

        assert!(!block.has_children);
        assert!(!block.archived);
        assert!(!block.in_trash);

        match block.block {
            Block::Bookmark { bookmark } => {
                assert_eq!(bookmark.url, "https://example.com");

                let rich_text = bookmark.caption.first().unwrap();

                match rich_text {
                    crate::object::rich_text::RichText::Text {
                        plain_text, href, ..
                    } => {
                        assert_eq!(plain_text, "example domain");
                        assert_eq!(*href, None);
                    }
                    _ => panic!(),
                }
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

        match block.block {
            Block::File { file } => match file {
                crate::object::file::File::Uploaded(uploaded_file) => {
                    assert_eq!(
                        uploaded_file.name,
                        Some("2024-07-18 202106.png".to_string())
                    )
                }
                crate::object::file::File::External(_) => panic!("Unexpected!"),
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

        match block.block {
            Block::Image { image } => match image {
                crate::object::file::File::Uploaded(uploaded_file) => {
                    assert_eq!(
                        uploaded_file.file.url,
                        "https://prod-files-secure.s3.us-west-2.amazonaws.com/"
                    )
                }
                crate::object::file::File::External(_) => panic!("Unexpected!"),
            },
            _ => panic!("Unexpected!"),
        }
    }
}
