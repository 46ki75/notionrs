use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseResponse {
    pub id: String,
    pub title: Vec<crate::object::rich_text::RichText>,
    pub description: Vec<crate::object::rich_text::RichText>,

    pub created_time: String,
    pub last_edited_time: String,

    pub cover: Option<crate::object::file::File>,
    pub icon: Option<crate::object::icon::Icon>,

    pub url: String,
    pub public_url: Option<String>,

    pub in_trash: bool,
    pub is_inline: bool,
    pub is_locked: bool,

    pub data_sources: Vec<DatabaseResponseDataSourceRef>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseResponseDataSourceRef {
    pub id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_database_response() -> Result<(), Box<dyn std::error::Error>> {
        let json_data = br#"
{
    "object": "database",
    "id": "20434608-d5c9-8048-ac7f-d7fe5f300cd3",
    "title": [
        {
            "type": "text",
            "text": {
                "content": "Relation Test 1 and New child data source",
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
            "plain_text": "Relation Test 1 and New child data source",
            "href": null
        }
    ],
    "description": [],
    "parent": {
        "type": "page_id",
        "page_id": "20434608-d5c9-8062-a995-e4505bd3ac14"
    },
    "is_inline": true,
    "in_trash": false,
    "is_locked": false,
    "created_time": "2025-05-31T02:22:10.989+00:00",
    "last_edited_time": "2025-08-31T16:29:10.588+00:00",
    "data_sources": [
        {
            "id": "20434608-d5c9-8083-af14-000bd5abd7d4",
            "name": "Relation Test 1"
        },
        {
            "id": "26c34608-d5c9-811e-931b-000b5bc01985",
            "name": "New child data source"
        }
    ],
    "icon": null,
    "cover": null,
    "url": "https://www.notion.so/20434608d5c98048ac7fd7fe5f300cd3",
    "public_url": null,
    "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
    "request_id": "6cd78e70-89ba-4c01-9433-9fa02e9b847c"
}
        "#;

        let _ = serde_json::from_slice::<DatabaseResponse>(json_data).unwrap();

        Ok(())
    }
}
