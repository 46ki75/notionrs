use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mention {
    Database {
        database: DatabaseMention,
    },
    Date {
        date: crate::page::date::PageDatePropertyParameter,
    },
    LinkPreview {
        link_preview: LinkPreviewMention,
    },
    LinkMention {
        link_mention: LinkMentionMention,
    },
    TemplateMention {
        template_mention: TemplateMention,
    },
    Page {
        page: PageMention,
    },
    User {
        user: crate::User,
    },
    CustomEmoji {
        custom_emoji: crate::others::custom_emoji::CustomEmojiContent,
    },
}

impl std::fmt::Display for Mention {
    /// Display the mention.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mention::Database { database } => write!(f, "{}", database),
            Mention::Date { date } => write!(f, "{}", date),
            Mention::LinkPreview { link_preview } => write!(f, "{}", link_preview),
            Mention::LinkMention { link_mention } => write!(f, "{}", link_mention),
            Mention::TemplateMention { template_mention } => write!(f, "{}", template_mention),
            Mention::Page { page } => write!(f, "{}", page),
            Mention::User { user } => write!(f, "{}", user),
            Mention::CustomEmoji { custom_emoji } => write!(f, "{}", custom_emoji),
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
    /// Display the page_id.
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

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

mod unit_tests {

    #[test]
    fn mention_user() {
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

        assert!(matches!(mention, super::Mention::User { .. }));
    }

    #[test]
    fn mention_date() {
        let json_data = r#"
        {
            "type": "date",
            "date": {
                "start": "2025-02-22",
                "end": null,
                "time_zone": null
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::Date { .. }));
    }

    #[test]
    fn mention_link_preview() {
        let json_data = r#"
        {
            "type": "link_preview",
            "link_preview": {
                "url": "https://www.rust-lang.org/"
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::LinkPreview { .. }));
    }

    #[test]
    fn mention_link_mention() {
        let json_data = r#"
        {
            "type": "link_mention",
            "link_mention": {
                "title": "Rust Programming Language",
                "description": "A language empowering everyone to build reliable and efficient software.",
                "url": "https://www.rust-lang.org/"
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::LinkMention { .. }));
    }

    #[test]
    fn mention_page() {
        let json_data = r#"
        {
            "type": "page",
            "page": {
                "id": "1a12bef9-0b67-80ed-92c1-d75e2d67c690"
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::Page { .. }));
    }

    #[test]
    fn mention_database() {
        let json_data = r#"
        {
            "type": "database",
            "database": {
                "id": "1a12bef9-0b67-80ed-92c1-d75e2d67c690"
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::Database { .. }));
    }

    #[test]
    fn mention_custom_emoji() {
        let json_data = r#"
        {
            "type": "custom_emoji",
            "custom_emoji": {
                "id": "1a22bef9-0b67-80fc-af7a-007aa14b8136",
                "name": "rust",
                "url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/"
            }
        }
        "#;

        let mention = serde_json::from_str::<super::Mention>(json_data).unwrap();

        assert!(matches!(mention, super::Mention::CustomEmoji { .. }));
    }
}
