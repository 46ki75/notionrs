use serde::{Deserialize, Serialize};

use self::title::PageTitle;

pub mod title;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PageProperty {
    PageTitle(PageTitle),
}
