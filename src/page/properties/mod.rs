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

pub use {
    button::PageButtonProperty, checkbox::PageCheckboxProperty, created_by::PageCreatedByProperty,
    created_time::PageCreatedTimeProperty, date::PageDateProperty, email::PageEmailProperty,
    files::PageFilesProperty, formula::PageFormulaProperty,
    last_edited_by::PageLastEditedByProperty, last_edited_time::PageLastEditedTimeProperty,
    multi_select::PageMultiSelectProperty, number::PageNumberProperty, people::PagePeopleProperty,
    phone_number::PagePhoneNumberProperty, relation::PageRelationProperty,
    rich_text::PageRichTextProperty, rollup::PageRollupProperty, select::PageSelectProperty,
    status::PageStatusProperty, title::PageTitleProperty, unique_id::PageUniqueIdProperty,
    url::PageUrlProperty, verification::PageVerificationProperty,
};

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
