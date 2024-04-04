use serde::{Deserialize, Serialize};

use self::{
    button::PageButtonProperty, checkbox::PageCheckboxProperty, created_by::PageCreatedByProperty,
    created_time::PageCreatedTimeProperty, date::PageDateProperty, email::PageEmailProperty,
    files::PageFilesProperty, formula::PageFormulaProperty,
    last_edited_by::PageLastEditedByProperty, last_edited_time::PageLastEditedTimeProperty,
    multi_select::PageMultiSelectProperty, number::PageNumberProperty, people::PagePeopleProperty,
    phone_number::PagePhoneNumberProperty, title::PageTitleProperty,
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
pub mod title;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Button(PageButtonProperty),
    Checkbox(PageCheckboxProperty),
    CreatedBy(PageCreatedByProperty),
    CreatedTime(PageCreatedTimeProperty),
    Date(PageDateProperty),
    Email(PageEmailProperty),
    Files(PageFilesProperty),
    Formula(PageFormulaProperty),
    LastEditedBy(PageLastEditedByProperty),
    LastEditedTime(PageLastEditedTimeProperty),
    MultiSelect(PageMultiSelectProperty),
    Number(PageNumberProperty),
    People(PagePeopleProperty),
    PhoneNumber(PagePhoneNumberProperty),
    Title(PageTitleProperty),
}
