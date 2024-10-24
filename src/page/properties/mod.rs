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

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Button {
        /// An underlying identifier for the property.
        /// `id` remains constant when the property name changes.
        id: String,

        /// Always `"button"`
        button: std::collections::HashMap<String, String>,
    },

    /// <https://developers.notion.com/reference/page-property-values#checkbox>
    Checkbox {
        /// An underlying identifier for the property.
        /// `id` remains constant when the property name changes.
        id: Option<String>,

        /// Whether the checkbox is checked (`true`) or unchecked (`false`).
        checkbox: bool,
    },
    CreatedBy(created_by::PageCreatedByProperty),
    CreatedTime(created_time::PageCreatedTimeProperty),
    Date(date::PageDateProperty),
    Email(email::PageEmailProperty),
    Files(files::PageFilesProperty),
    Formula(formula::PageFormulaProperty),
    LastEditedBy(last_edited_by::PageLastEditedByProperty),
    LastEditedTime(last_edited_time::PageLastEditedTimeProperty),
    MultiSelect(multi_select::PageMultiSelectProperty),
    Number(number::PageNumberProperty),
    People(people::PagePeopleProperty),
    PhoneNumber(phone_number::PagePhoneNumberProperty),
    Relation(relation::PageRelationProperty),
    RichText(rich_text::PageRichTextProperty),
    Rollup(rollup::PageRollupProperty),
    Select(select::PageSelectProperty),
    Status(status::PageStatusProperty),
    Title {
        /// An underlying identifier for the property.
        /// `id` remains constant when the property name changes.
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,

        /// An array of [rich text](https://developers.notion.com/reference/rich-text) objects.
        title: Vec<crate::others::rich_text::RichText>,
    },
    UniqueId(unique_id::PageUniqueIdProperty),
    Url(url::PageUrlProperty),
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

// #[cfg(test)]
// mod unit_tests {
//     use crate::prelude::ToJson;

//     use super::*;
//     use serde_json;

//     #[test]
//     fn deserialize_page_property() {
//         let json_data = r#"
//         {
//             "Button": {
//                 "id": "c%60qZ",
//                 "type": "button",
//                 "button": {}
//             }
//         }
//         "#;

//         let button_map =
//             serde_json::from_str::<std::collections::HashMap<String, PageProperty>>(json_data)
//                 .unwrap();

//         let button = button_map.get("Button").unwrap();

//         match button {
//             PageProperty::Button(button_property) => {
//                 assert_eq!(button_property.id, "c%60qZ".to_string());
//                 assert!(button_property.button.is_empty());
//             }
//             _ => panic!("Expected a Button variant"),
//         }

//         let button_string = button.to_json();

//         assert!(button_string.contains("\"type\":\"button\""));
//     }
// }
