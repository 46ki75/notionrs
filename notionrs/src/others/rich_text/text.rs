use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Text {
    pub content: String,
    pub link: Option<TextLink>,
}

impl Text {
    pub fn content<T>(mut self, content: T) -> Self
    where
        T: AsRef<str>,
    {
        self.content = content.as_ref().to_string();
        self
    }

    pub fn link<T>(mut self, link: T) -> Self
    where
        T: AsRef<str>,
    {
        self.link = Some(TextLink {
            url: link.as_ref().to_string(),
        });
        self
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<T> From<T> for Text
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::default().content(value)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TextLink {
    pub url: String,
}
