use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct TranscriptionBlock {
    pub title: Option<Vec<crate::object::rich_text::RichText>>,

    pub status: Option<TranscriptionStatus>,

    pub children: Option<TranscriptionChildren>,

    pub calendar_event: Option<TranscriptionCalendarEvent>,

    pub recording: Option<TranscriptionRecording>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct TranscriptionChildren {
    pub summary_block_id: Option<String>,

    pub notes_block_id: Option<String>,

    pub transcript_block_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionStatus {
    TranscriptionNotStarted,
    TranscriptionPaused,
    TranscriptionInProgress,
    SummaryInProgress,
    NotesReady,
}

#[derive(Deserialize, Serialize, Debug, Clone, notionrs_macro::Setter)]
pub struct TranscriptionCalendarEvent {
    #[serde(with = "time::serde::rfc3339")]
    pub start_time: time::OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub end_time: time::OffsetDateTime,

    pub attendees: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, notionrs_macro::Setter)]
pub struct TranscriptionRecording {
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,

    #[serde(default, with = "time::serde::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}

impl fmt::Display for TranscriptionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.title
                .as_deref()
                .map(|title| title.iter().map(|r| r.to_string()).collect::<String>())
                .unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_block_transcript() {
        let json_data = br#"
        {
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": "Salami Lid Issue",
                        "link": null
                    },
                    "annotations": {
                        "bold": true,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "Salami Lid Issue",
                    "href": null
                },
                {
                    "type": "text",
                    "text": {
                        "content": " ",
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
                    "plain_text": " ",
                    "href": null
                },
                {
                    "type": "mention",
                    "mention": {
                        "type": "date",
                        "date": {
                            "start": "2026-02-28",
                            "end": null,
                            "time_zone": null
                        }
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "2026-02-28",
                    "href": null
                }
            ],
            "status": "notes_ready",
            "children": {
                "summary_block_id": "31434608-d5c9-80b6-a815-fcb76dc6be32",
                "notes_block_id": "31434608-d5c9-80b3-bec5-c974fd3ebaf6",
                "transcript_block_id": "31434608-d5c9-805d-b5b3-feab2fa3cb2a"
            },
            "recording": {
                "start_time": "2026-02-27T12:43:00.000Z"
            }
        }
        "#;

        let transcription: TranscriptionBlock = serde_json::from_slice(json_data).unwrap();

        assert_eq!(
            transcription.children.unwrap().summary_block_id,
            Some("31434608-d5c9-80b6-a815-fcb76dc6be32".to_owned())
        );
    }
}
