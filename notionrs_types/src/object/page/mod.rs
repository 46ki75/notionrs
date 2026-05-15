use serde::{Deserialize, Serialize};

use crate::{
    object::user::User,
    object::{emoji_and_icon::EmojiAndIcon, file::File, parent::Parent},
};

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
pub mod place;
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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Button(button::PageButtonProperty),
    Checkbox(checkbox::PageCheckboxProperty),
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
    Place(place::PagePlaceProperty),
    Relation(relation::PageRelationProperty),
    RichText(rich_text::PageRichTextProperty),
    Rollup(rollup::PageRollupProperty),
    Select(select::PageSelectProperty),
    Status(status::PageStatusProperty),
    Title(title::PageTitleProperty),
    UniqueId(unique_id::PageUniqueIdProperty),
    Url(url::PageUrlProperty),
    Verification(verification::PageVerificationProperty),
}

impl std::fmt::Display for PageProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageProperty::Button(button) => write!(f, "{}", button),
            PageProperty::Checkbox(checkbox) => write!(f, "{}", checkbox),
            PageProperty::CreatedBy(created_by) => write!(f, "{}", created_by),
            PageProperty::CreatedTime(created_time) => write!(f, "{}", created_time),
            PageProperty::Date(date) => write!(f, "{}", date),
            PageProperty::Email(email) => write!(f, "{}", email),
            PageProperty::Files(files) => write!(f, "{}", files),
            PageProperty::Formula(formula) => write!(f, "{}", formula),
            PageProperty::LastEditedBy(last_edited_by) => write!(f, "{}", last_edited_by),
            PageProperty::LastEditedTime(last_edited_time) => write!(f, "{}", last_edited_time),
            PageProperty::MultiSelect(multi_select) => write!(f, "{}", multi_select),
            PageProperty::Number(number) => write!(f, "{}", number),
            PageProperty::People(people) => write!(f, "{}", people),
            PageProperty::PhoneNumber(phone_number) => write!(f, "{}", phone_number),
            PageProperty::Place(place) => write!(f, "{}", place),
            PageProperty::Relation(relation) => write!(f, "{}", relation),
            PageProperty::RichText(rich_text) => write!(f, "{}", rich_text),
            PageProperty::Rollup(rollup) => write!(f, "{}", rollup),
            PageProperty::Select(select) => write!(f, "{}", select),
            PageProperty::Status(status) => write!(f, "{}", status),
            PageProperty::Title(title) => write!(f, "{}", title),
            PageProperty::UniqueId(unique_id) => write!(f, "{}", unique_id),
            PageProperty::Url(url) => write!(f, "{}", url),
            PageProperty::Verification(verification) => write!(f, "{}", verification),
        }
    }
}

/// <https://developers.notion.com/reference/page>
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageResponse<T = std::collections::HashMap<String, crate::object::page::PageProperty>>
where
    T: Clone + Send + 'static,
{
    pub id: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_time: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub last_edited_time: time::OffsetDateTime,
    pub created_by: User,
    pub last_edited_by: User,
    pub cover: Option<File>,
    pub icon: Option<EmojiAndIcon>,
    pub parent: Parent,
    #[deprecated(note = "Use `in_trash` instead. Deprecated in 2026-03-11 API version.")]
    #[serde(default)]
    pub archived: bool,
    pub properties: T,
    pub url: String,
    pub public_url: Option<String>,
    pub developer_survey: Option<String>,
    pub request_id: Option<String>,
    pub in_trash: bool,
    pub is_locked: bool,
    /// Whether the page has been archived.
    #[serde(default)]
    pub is_archived: bool,
}

// # --------------------------------------------------------------------------------
//
// unit_tests
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_wiki_page() {
        let json_data = include_str!("./seeds/page_wiki.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn deserialize_page() {
        let json_data = include_str!("./seeds/page.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn deserialize_page_custom_struct() {
        let json_data = include_str!("./seeds/page.json");

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        struct CustomProperty {
            #[serde(rename = "Files & media")]
            files: files::PageFilesProperty,

            #[serde(rename = "User")]
            user: people::PagePeopleProperty,

            email: email::PageEmailProperty,

            #[serde(rename = "Date")]
            date: date::PageDateProperty,

            #[serde(rename = "Checkbox")]
            checkbox: checkbox::PageCheckboxProperty,

            #[serde(rename = "URL")]
            url: url::PageUrlProperty,

            #[serde(rename = "Phone Number")]
            phone_number: phone_number::PagePhoneNumberProperty,

            #[serde(rename = "Text")]
            text: rich_text::PageRichTextProperty,

            #[serde(rename = "Select")]
            select: select::PageSelectProperty,

            #[serde(rename = "Button")]
            button: button::PageButtonProperty,

            #[serde(rename = "Relation")]
            relation: relation::PageRelationProperty,

            #[serde(rename = "LastUpdatedBy")]
            last_edited_by: last_edited_by::PageLastEditedByProperty,

            #[serde(rename = "ID")]
            unique_id: unique_id::PageUniqueIdProperty,

            #[serde(rename = "Multi-select")]
            multi_select: multi_select::PageMultiSelectProperty,

            #[serde(rename = "LastUpdatedAt")]
            last_edited_time: last_edited_time::PageLastEditedTimeProperty,

            formula: formula::PageFormulaProperty,

            #[serde(rename = "CreatedBy")]
            created_by: created_by::PageCreatedByProperty,

            #[serde(rename = "Status")]
            status: status::PageStatusProperty,

            #[serde(rename = "Rollup")]
            rollup: rollup::PageRollupProperty,

            #[serde(rename = "Title")]
            title: title::PageTitleProperty,
        }

        let _page =
            serde_json::from_str::<crate::object::page::PageResponse<CustomProperty>>(json_data)
                .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn deserialize_page_icon_emoji() {
        let json_data = include_str!("./seeds/page_icon_emoji.json");

        let _page = serde_json::from_str::<crate::object::page::PageResponse>(json_data)
            .expect("An error occurred while deserializing the page");
    }

    #[test]
    fn page_property_display_all_variants() {
        use crate::object::page::{
            button::PageButtonProperty,
            checkbox::PageCheckboxProperty,
            created_by::PageCreatedByProperty,
            created_time::PageCreatedTimeProperty,
            date::PageDateProperty,
            email::PageEmailProperty,
            files::PageFilesProperty,
            formula::{Formula, FormulaString, PageFormulaProperty},
            last_edited_by::PageLastEditedByProperty,
            last_edited_time::PageLastEditedTimeProperty,
            multi_select::PageMultiSelectProperty,
            number::PageNumberProperty,
            people::PagePeopleProperty,
            phone_number::PagePhoneNumberProperty,
            place::PagePlaceProperty,
            relation::PageRelationProperty,
            rich_text::PageRichTextProperty,
            rollup::PageRollupProperty,
            select::PageSelectProperty,
            status::PageStatusProperty,
            title::PageTitleProperty,
            unique_id::{PageUniqueIdProperty, PageUniqueIdPropertyParameter},
            url::PageUrlProperty,
            verification::PageVerificationProperty,
        };
        use crate::object::select::Select;
        use crate::object::user::User;

        let now = time::OffsetDateTime::now_utc();
        let variants = vec![
            PageProperty::Button(PageButtonProperty::default()),
            PageProperty::Checkbox(PageCheckboxProperty::from(true)),
            PageProperty::CreatedBy(PageCreatedByProperty::default().created_by(User::default())),
            PageProperty::CreatedTime(PageCreatedTimeProperty::default().created_time(now)),
            PageProperty::Date(PageDateProperty::default()),
            PageProperty::Email(PageEmailProperty::from("e@x.com")),
            PageProperty::Files(PageFilesProperty::default()),
            PageProperty::Formula(PageFormulaProperty {
                id: None,
                formula: Formula::String(FormulaString {
                    string: Some("v".into()),
                }),
            }),
            PageProperty::LastEditedBy(PageLastEditedByProperty {
                id: None,
                last_edited_by: User::default(),
            }),
            PageProperty::LastEditedTime(PageLastEditedTimeProperty {
                id: None,
                last_edited_time: now,
            }),
            PageProperty::MultiSelect(PageMultiSelectProperty::default()),
            PageProperty::Number(PageNumberProperty::from(1.0)),
            PageProperty::People(PagePeopleProperty::default()),
            PageProperty::PhoneNumber(PagePhoneNumberProperty::from("555")),
            PageProperty::Place(PagePlaceProperty::default()),
            PageProperty::Relation(PageRelationProperty::default()),
            PageProperty::RichText(PageRichTextProperty::from("text")),
            PageProperty::Rollup(PageRollupProperty { id: None }),
            PageProperty::Select(PageSelectProperty::default()),
            PageProperty::Status(PageStatusProperty::default().status(Select::from("S"))),
            PageProperty::Title(PageTitleProperty::from("Title")),
            PageProperty::UniqueId(PageUniqueIdProperty {
                id: None,
                unique_id: PageUniqueIdPropertyParameter::default(),
            }),
            PageProperty::Url(PageUrlProperty::from("https://u")),
            PageProperty::Verification(PageVerificationProperty::default()),
        ];

        for v in &variants {
            let _ = v.to_string();
        }
    }
}
