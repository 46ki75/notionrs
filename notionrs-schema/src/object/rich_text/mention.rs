use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Mention {
    User {
        /// <https://developers.notion.com/reference/rich-text#user-mention-type-object>
        user: crate::object::user::User,
    },
    Date {
        /// <https://developers.notion.com/reference/rich-text#date-mention-type-object>
        ///
        /// Date mentions contain a date property value object within the corresponding date field.
        date: crate::object::page::date::PageDatePropertyParameter,
    },
    LinkPreview {
        link_preview: LinkPreviewMention,
    },
    LinkMention {
        link_mention: LinkMention,
    },
    TemplateMention {
        template_mention: TemplateMention,
    },
    Page {
        page: PageMention,
    },
    Database {
        database: DatabaseMention,
    },
    CustomEmoji {
        custom_emoji: crate::object::emoji::CustomEmojiContent,
    },
}

impl std::fmt::Display for Mention {
    /// Display the mention.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mention::User { user } => write!(f, "{}", user),
            Mention::Date { date } => write!(f, "{}", date),
            Mention::LinkPreview { link_preview } => write!(f, "{}", link_preview),
            Mention::LinkMention { link_mention } => write!(f, "{}", link_mention),
            Mention::TemplateMention { template_mention } => write!(f, "{}", template_mention),
            Mention::Page { page } => write!(f, "{}", page),
            Mention::Database { database } => write!(f, "{}", database),
            Mention::CustomEmoji { custom_emoji } => write!(f, "{}", custom_emoji),
        }
    }
}

/// <https://developers.notion.com/reference/rich-text#database-mention-type-object>
///
/// Database mentions contain a database reference within the corresponding `database` field.
/// A database reference is an object with an `id` key and a string value (UUIDv4) corresponding to a database ID.
///
/// If an integration doesn’t have [access](https://developers.notion.com/reference/capabilities) to the mentioned database,
/// then the mention is returned with just the ID.
/// The `plain_text` value that would be a title appears as `"Untitled"`
/// and the annotation object’s values are defaults.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, notionrs_macro::Setter)]
pub struct DatabaseMention {
    /// database id.
    pub id: String,
}
crate::impl_display_from_string_field!(DatabaseMention, id);
crate::impl_from_as_ref!(DatabaseMention, id);

/// <https://developers.notion.com/reference/rich-text#link-preview-mention-type-object>
///
/// If a user opts to share a Link Preview as a mention,
/// then the API handles the Link Preview mention
/// as a rich text object with a `type` value of `link_preview`.
/// Link preview rich text mentions contain a corresponding
/// `link_preview` object that includes the `url`
/// that is used to create the Link Preview mention.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, notionrs_macro::Setter)]
pub struct LinkPreviewMention {
    /// The URL of the link.
    pub url: String,
}
crate::impl_display_from_string_field!(LinkPreviewMention, url);
crate::impl_from_as_ref!(LinkPreviewMention, url);

/// There is no documentation for this type in the official API reference.
///
/// The `href` field is a string that represents the URL of the link.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, notionrs_macro::Setter)]
pub struct LinkMention {
    pub href: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub link_author: Option<String>,
    pub link_provider: Option<String>,
    pub thumbnail_url: Option<String>,
    pub icon_url: Option<String>,
    pub iframe_url: Option<String>,
    pub height: Option<u32>,
    pub padding: Option<u32>,
    pub padding_top: Option<u32>,
}
crate::impl_display_from_string_field!(LinkMention, href);
crate::impl_from_as_ref!(LinkMention, href);

/// <https://developers.notion.com/reference/rich-text#page-mention-type-object>
///
/// Page mentions contain a `page` reference within the corresponding `page` field.
/// A page reference is an object with an `id` property and a string value (UUIDv4)
/// corresponding to a page ID.
///
/// If an integration doesn’t have access to the mentioned page,
/// then the mention is returned with just the ID. The `plain_text` value
/// that would be a title appears as `"Untitled"` and the annotation object’s values are defaults.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, notionrs_macro::Setter)]
pub struct PageMention {
    /// page id.
    pub id: String,
}
crate::impl_display_from_string_field!(PageMention, id);
crate::impl_from_as_ref!(PageMention, id);

/// <https://developers.notion.com/reference/rich-text#template-mention-type-object>
///
/// The content inside a [template button](https://www.notion.com/ja/help/buttons?cookie_sync_completed=true)
/// in the Notion UI can include placeholder date
/// and user mentions that populate when a template is duplicated.
/// Template mention type objects contain these populated values.
/// Template mention rich text objects contain a template_mention object
/// with a nested type key that is either `"template_mention_date"` or `"template_mention_user"`.
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
                "href": "https://www.rust-lang.org/"
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
