use serde::{Deserialize, Serialize};

use self::{
    button::PageButton, checkbox::PageCheckbox, created_by::PageCreatedBy,
    created_time::PageCreatedTime, date::PageDate, email::PageEmail, title::PageTitle,
};

pub mod button;
pub mod checkbox;
pub mod created_by;
pub mod created_time;
pub mod date;
pub mod email;
pub mod title;

#[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Button(PageButton),
    Checkbox(PageCheckbox),
    CreatedBy(PageCreatedBy),
    CreatedTime(PageCreatedTime),
    Date(PageDate),
    Email(PageEmail),
    Title(PageTitle),
}
