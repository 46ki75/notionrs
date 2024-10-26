use serde::{Deserialize, Serialize};

use super::{emoji::Emoji, file::File};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}

impl Icon {
    pub fn new_file() -> Self {
        Icon::File(File::new())
    }

    pub fn new_emoji(emoji: char) -> Self {
        Icon::Emoji(Emoji::from(emoji))
    }
}

impl Default for Icon {
    fn default() -> Self {
        Icon::Emoji(Emoji::default())
    }
}
