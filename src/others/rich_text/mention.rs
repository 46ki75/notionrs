use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mention {
    Database(DatabaseMention),
    Date(crate::page::date::PageDatePropertyParameter),
    LinkPreview(LinkPreviewMention),
    LinkMention(LinkMentionMention),
    TemplateMention(TemplateMention),
    Page(PageMention),
    User(UserMention),
}

impl crate::ToPlainText for Mention {
    /// Convert Mention to a plain string
    fn to_plain_text(&self) -> String {
        match self {
            Mention::Database(db) => db.to_plain_text(),
            Mention::Date(date) => date.to_plain_text(),
            Mention::LinkPreview(lp) => lp.to_plain_text(),
            Mention::LinkMention(lm) => lm.to_plain_text(),
            Mention::TemplateMention(tm) => tm.to_plain_text(),
            Mention::Page(p) => p.to_plain_text(),
            Mention::User(user) => user.user.to_plain_text(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatabaseMention {
    pub id: String,
}

impl crate::ToPlainText for DatabaseMention {
    /// Convert DatabaseMention to a plain string
    fn to_plain_text(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewMention {
    pub url: String,
}

impl crate::ToPlainText for LinkPreviewMention {
    /// Convert LinkPreviewMention to a plain string
    fn to_plain_text(&self) -> String {
        self.url.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkMentionMention {
    pub description: Option<String>,
    pub href: Option<String>,
    pub icon_url: Option<String>,
    pub iframe_url: Option<String>,
    pub link_author: Option<String>,
    pub padding: Option<u32>,
    pub thumbnail_url: Option<String>,
    pub title: String,
}

impl crate::ToPlainText for LinkMentionMention {
    /// Convert LinkMentionMention to a plain string
    fn to_plain_text(&self) -> String {
        self.title.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PageMention {
    pub id: String,
}

impl crate::ToPlainText for PageMention {
    /// Convert PageMention to a plain string
    fn to_plain_text(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TemplateMention {
    TemplateMentionDate(TemplateMentionDate),
    TemplateMentionUser(TemplateMentionUser),
}

impl std::fmt::Display for TemplateMention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateMention::TemplateMentionDate(date) => {
                write!(f, "template_mention_date: {}", date)
            }
            TemplateMention::TemplateMentionUser(user) => {
                write!(f, "template_mention_user: {}", user)
            }
        }
    }
}

impl crate::ToPlainText for TemplateMention {
    /// Convert TemplateMention to a plain string
    fn to_plain_text(&self) -> String {
        self.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMentionDate {
    Today,
    Now,
}

impl std::fmt::Display for TemplateMentionDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_str = match self {
            TemplateMentionDate::Today => "today",
            TemplateMentionDate::Now => "now",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMentionUser {
    Me,
}

impl std::fmt::Display for TemplateMentionUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_str = match self {
            TemplateMentionUser::Me => "me",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct UserMention {
    pub user: crate::User,
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

mod unit_tests {

    #[test]
    fn deserialize_mention() {
        let json_data = r#"
        {
            "type": "user",
            "user": {
                "object": "user",
                "id": "151144c2-b503-4c01-b12f-a48dad7cb8da",
                "name": "x",
                "avatar_url": "https://lh3.googleusercontent.com/",
                "type": "person",
                "person": {
                    "email": "46ki75@example.com"
                }
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        if let crate::others::rich_text::mention::Mention::User(user) = mention {
            if let crate::User::Person(person) = user.user {
                assert_eq!(
                    person.id,
                    "151144c2-b503-4c01-b12f-a48dad7cb8da".to_string()
                )
            }
        }
    }
}
