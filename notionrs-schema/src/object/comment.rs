use serde::{Deserialize, Serialize};

/// When creating a new comment, either the parent.page_id or discussion_id parameter must be provided — not both.
#[derive(Debug, Deserialize, Serialize, Clone, notionrs_macro::Setter)]
pub struct Comment {
    /// Always `"comment"`,
    #[serde(skip_serializing)]
    pub object: String,

    #[serde(skip_serializing)]
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<crate::object::parent::Parent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_id: Option<String>,

    #[serde(skip_serializing)]
    #[serde(with = "time::serde::rfc3339")]
    pub created_time: time::OffsetDateTime,

    #[serde(skip_serializing)]
    #[serde(with = "time::serde::rfc3339")]
    pub last_edited_time: time::OffsetDateTime,

    #[serde(skip_serializing)]
    pub created_by: Option<crate::object::user::User>,

    pub rich_text: Vec<crate::object::rich_text::RichText>,
}

impl Default for Comment {
    fn default() -> Self {
        Comment {
            object: "comment".to_string(),
            id: String::new(),
            parent: None,
            discussion_id: None,
            created_time: time::OffsetDateTime::now_utc(),
            last_edited_time: time::OffsetDateTime::now_utc(),
            created_by: None,
            rich_text: vec![],
        }
    }
}

impl<T> From<T> for Comment
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self {
            rich_text: vec![crate::object::rich_text::RichText::from(value)],
            ..Default::default()
        }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.rich_text
                .iter()
                .map(|s| s.to_string())
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn deserialize_url_property() {
        let json_data = r#"
        {
            "object": "comment",
            "id": "1e834608-d5c9-8021-894d-001df60d9340",
            "parent": {
                "type": "block_id",
                "block_id": "1e334608-d5c9-80a4-a8a3-e524a536c43f"
            },
            "discussion_id": "1e834608-d5c9-80a2-ab7a-001c2c516cfd",
            "created_time": "2025-05-03T11:45:00.000Z",
            "last_edited_time": "2025-05-03T11:45:00.000Z",
            "created_by": {
                "object": "user",
                "id": "79a215bc-a3f0-4977-bb62-1ef058f06556"
            },
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": "Workers of the world, unite!",
                        "link": null
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "Workers of the world, unite!",
                    "href": null
                }
            ]
        }
        "#;

        let comment = serde_json::from_str::<Comment>(json_data).unwrap();

        assert_eq!(
            comment.discussion_id,
            Some("1e834608-d5c9-80a2-ab7a-001c2c516cfd".to_string())
        );
    }
}
