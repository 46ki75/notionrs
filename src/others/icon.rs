use serde::{Deserialize, Serialize};

use super::{emoji::Emoji, file::File};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}

impl Default for Icon {
    fn default() -> Self {
        Icon::Emoji(Emoji::default())
    }
}
