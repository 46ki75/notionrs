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

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Icon::File(file) => write!(f, "{}", file),
            Icon::Emoji(emoji) => write!(f, "{}", emoji),
        }
    }
}
