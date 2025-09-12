use serde::{Deserialize, Serialize};

pub mod button;
pub mod checkbox;
pub mod created_by;
pub mod created_time;
pub mod date;
pub mod email;
pub mod files;
pub mod formula;
pub mod last_edited_by;
pub mod last_edited_time;
pub mod multi_select;
pub mod number;
pub mod people;
pub mod phone_number;
pub mod relation;
pub mod rich_text;
pub mod rollup;
pub mod select;
pub mod status;
pub mod title;
pub mod unique_id;
pub mod url;
pub mod verification;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseResponse {
    pub id: String,

    pub created_time: String,

    pub last_edited_time: String,

    pub cover: Option<crate::object::file::File>,

    pub icon: Option<crate::object::icon::Icon>,

    pub url: String,

    pub public_url: Option<String>,

    pub in_trash: bool,

    pub is_inline: bool,

    pub title: Vec<crate::object::rich_text::RichText>,

    pub description: Vec<crate::object::rich_text::RichText>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataSourceResponse {
    pub id: String,

    pub created_time: String,

    pub last_edited_time: String,

    pub parent: crate::object::parent::DatabaseParent,

    pub properties: std::collections::HashMap<String, DataSourceProperty>,

    pub icon: Option<crate::object::icon::Icon>,

    pub cover: Option<crate::object::file::File>,

    pub url: String,

    pub title: Vec<crate::object::rich_text::RichText>,

    pub archived: bool,

    pub in_trash: bool,

    pub description: Vec<crate::object::rich_text::RichText>,

    pub public_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataSourceProperty {
    Button(button::DataSourceButtonProperty),
    Checkbox(checkbox::DataSourceCheckboxProperty),
    CreatedBy(created_by::DataSourceCreatedByProperty),
    CreatedTime(created_time::DataSourceCreatedTimeProperty),
    Date(date::DataSourceDateProperty),
    Email(email::DataSourceEmailProperty),
    Files(files::DataSourceFilesProperty),
    Formula(formula::DataSourceFormulaProperty),
    LastEditedBy(last_edited_by::DataSourceLastEditedByProperty),
    LastEditedTime(last_edited_time::DataSourceLastEditedTimeProperty),
    MultiSelect(multi_select::DataSourceMultiSelectProperty),
    Number(number::DataSourceNumberProperty),
    People(people::DataSourcePeopleProperty),
    PhoneNumber(phone_number::DataSourcePhoneNumberProperty),
    Relation(relation::DataSourceRelationProperty),
    RichText(rich_text::DataSourceRichTextProperty),
    Rollup(rollup::DataSourceRollupProperty),
    Select(select::DataSourceSelectProperty),
    Status(status::DatabaseStatusProperty),
    Title(title::DataSourceTitleProperty),
    UniqueId(unique_id::DataSourceUniqueIdProperty),
    Url(url::DataSourceUrlProperty),
    Verification(verification::DataSourceVerificationProperty),
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
    pub fn deserialize_data_source_response() {
        let json_data = r#"
{
    "object": "data_source",
    "id": "26c34608-d5c9-811e-931b-000b5bc01985",
    "cover": null,
    "icon": null,
    "created_time": "2025-05-31T02:22:00.000Z",
    "created_by": {
        "object": "user",
        "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
    },
    "last_edited_by": {
        "object": "user",
        "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
    },
    "last_edited_time": "2025-08-31T16:29:00.000Z",
    "title": [
        {
            "type": "text",
            "text": {
                "content": "New child data source",
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
            "plain_text": "New child data source",
            "href": null
        }
    ],
    "description": [],
    "is_inline": true,
    "properties": {
        "Count": {
            "id": "KBHY",
            "name": "Count",
            "description": null,
            "type": "number",
            "number": {
                "format": "number"
            }
        },
        "Title": {
            "id": "title",
            "name": "Title",
            "description": null,
            "type": "title",
            "title": {}
        }
    },
    "parent": {
        "type": "database_id",
        "database_id": "20434608-d5c9-8048-ac7f-d7fe5f300cd3"
    },
    "database_parent": {
        "type": "page_id",
        "page_id": "20434608-d5c9-8062-a995-e4505bd3ac14"
    },
    "url": "https://www.notion.so/20434608d5c98048ac7fd7fe5f300cd3",
    "public_url": null,
    "archived": false,
    "in_trash": false,
    "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
    "request_id": "aa6a89be-0b55-47e1-93e8-c0f5eedceae9"
}
        "#;

        let _ = serde_json::from_str::<DataSourceResponse>(json_data).unwrap();
    }
}
