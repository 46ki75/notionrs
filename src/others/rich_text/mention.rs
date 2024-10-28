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

impl std::fmt::Display for Mention {
    /// Display the mention.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mention::Database(db) => write!(f, "{}", db),
            Mention::Date(date) => write!(f, "{}", date),
            Mention::LinkPreview(lp) => write!(f, "{}", lp),
            Mention::LinkMention(lm) => write!(f, "{}", lm),
            Mention::TemplateMention(tm) => write!(f, "{}", tm),
            Mention::Page(p) => write!(f, "{}", p),
            Mention::User(user) => write!(f, "{}", user),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatabaseMention {
    pub id: String,
}

impl std::fmt::Display for DatabaseMention {
    /// Display the database_id.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreviewMention {
    pub url: String,
}

impl std::fmt::Display for LinkPreviewMention {
    /// Display the url.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
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

impl std::fmt::Display for LinkMentionMention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PageMention {
    pub id: String,
}

impl std::fmt::Display for PageMention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TemplateMention {
    TemplateMentionDate(TemplateMentionDate),
    TemplateMentionUser(TemplateMentionUser),
}

impl std::fmt::Display for TemplateMention {
    /// Display the date or user if it exists.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateMention::TemplateMentionDate(date) => {
                write!(f, "{}", date)
            }
            TemplateMention::TemplateMentionUser(user) => {
                write!(f, "{}", user)
            }
        }
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

impl std::fmt::Display for UserMention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user)
    }
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
