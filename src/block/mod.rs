use serde::{Deserialize, Serialize};

pub mod audio;
pub mod bookmark;
pub mod bulleted_list_item;
pub mod callout;
pub mod child_database;
pub mod child_page;
pub mod code;
pub mod embed;
pub mod equation;
pub mod file;
pub mod heading;
pub mod image;
pub mod link_preview;
pub mod numbered_list_tem;
pub mod paragraph;
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
pub struct Block {
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
    pub details: BlockType,
}

/// <https://developers.notion.com/reference/block#block-type-objects>
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockType {
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
        column_list: std::collections::HashMap<(), ()>,
    },
    Column {
        column: std::collections::HashMap<(), ()>,
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
        numbered_list_item: numbered_list_tem::NumberedListItemBlock,
    },
    Paragraph {
        paragraph: paragraph::ParagraphBlock,
    },
    Pdf {
        pdf: crate::others::file::File,
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
    Video(video::VideoBlock),
    Unknown(serde_json::Value),
}

impl BlockType {
    // pub fn audio<T>(url: T) -> Self
    // where
    //     T: AsRef<str>,
    // {
    //     BlockType::Audio {
    //         audio: crate::others::file::File::new(url.as_ref()),
    //     }
    // }

    pub fn bookmark<T>(url: T) -> bookmark::BookmarkBlock
    where
        T: AsRef<str>,
    {
        bookmark::BookmarkBlock {
            caption: vec![],
            url: url.as_ref().to_string(),
        }
    }

    pub fn file<T>(url: T) -> file::FileBlock
    where
        T: AsRef<str>,
    {
        file::FileBlock {
            file: crate::others::file::File::new(url),
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

        let block = serde_json::from_str::<Block>(json_data).unwrap();

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
            BlockType::Bookmark { bookmark } => {
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

        let block = serde_json::from_str::<Block>(json_data).unwrap();

        match block.details {
            BlockType::File(file) => match file.file {
                crate::others::file::File::File(uploaded_file) => {
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

        let block = serde_json::from_str::<Block>(json_data).unwrap();

        println!("{:?}", block);

        match block.details {
            BlockType::Image(image) => match image.image {
                crate::others::file::File::File(uploaded_file) => {
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
