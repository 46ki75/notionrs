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

impl crate::ToPlainText for Text {
    /// Convert Text to a plain string
    fn to_plain_text(&self) -> String {
        self.content.clone()
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
