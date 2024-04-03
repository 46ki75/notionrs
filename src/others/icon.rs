use serde::{Deserialize, Serialize};

use super::{emoji::Emoji, file::File};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}
