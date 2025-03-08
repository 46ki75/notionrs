use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/rich-text#text>
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct Text {
    /// The actual text content of the `Text`.
    pub content: String,

    /// An object with information about any inline link in this text, if included.
    /// If the text contains an inline link, then the object key is url and the value is the URL’s string web address.
    /// If the text doesn’t have any inline links, then the value is null.
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

    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.link = Some(TextLink {
            url: url.as_ref().to_string(),
        });
        self
    }
}

crate::impl_display_from_string_field!(Text, content);
crate::impl_from_as_ref!(Text, content);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct TextLink {
    pub url: String,
}

crate::impl_display_from_string_field!(TextLink, url);
crate::impl_from_as_ref!(TextLink, url);
