use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub struct ListResponse<T> {
    /// always "list"
    pub object: String,
    pub results: Vec<T>,

    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum SearchResultItem {
    Page(crate::object::page::PageResponse),
    DataSource(crate::object::data_source::DataSourceResponse),
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn deserialize_search_result() {
        let json_data = r#"
{
    "object": "list",
    "results": [
        {
            "object": "page",
            "id": "26034608-d5c9-8042-a87a-ca6b04cf02c7",
            "created_time": "2025-08-31T16:29:00.000Z",
            "last_edited_time": "2025-08-31T16:30:00.000Z",
            "created_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "last_edited_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "cover": null,
            "icon": null,
            "parent": {
                "type": "database_id",
                "database_id": "20434608-d5c9-8048-ac7f-d7fe5f300cd3"
            },
            "archived": false,
            "in_trash": false,
            "is_locked": false,
            "properties": {
                "Parent item": {
                    "id": "JW%7BY",
                    "type": "relation",
                    "relation": [
                        {
                            "id": "20434608-d5c9-8067-8394-fbda69fef1aa"
                        }
                    ],
                    "has_more": false
                },
                "Relation": {
                    "id": "MZlh",
                    "type": "relation",
                    "relation": [],
                    "has_more": false
                },
                "Sub-item": {
                    "id": "%5BhIv",
                    "type": "relation",
                    "relation": [],
                    "has_more": false
                },
                "Tags": {
                    "id": "x%3BAi",
                    "type": "multi_select",
                    "multi_select": [
                        {
                            "id": "3bbd15dd-e836-4f7c-b93a-f3c6a6e42407",
                            "name": "Test",
                            "color": "gray"
                        }
                    ]
                },
                "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "MY Sub Item",
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
                            "plain_text": "MY Sub Item",
                            "href": null
                        }
                    ]
                },
                "Verification": {
                    "id": "verification",
                    "type": "verification",
                    "verification": {
                        "state": "unverified",
                        "verified_by": null,
                        "date": null
                    }
                },
                "Owner": {
                    "id": "verification_owner",
                    "type": "people",
                    "people": [
                        {
                            "object": "user",
                            "id": "d7592761-a145-4788-ba2c-d67bbfaed77f",
                            "name": "Shirayuki",
                            "avatar_url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/",
                            "type": "person",
                            "person": {
                                "email": "hi@example.com"
                            }
                        }
                    ]
                }
            },
            "url": "https://www.notion.so/MY-Sub-Item-26034608d5c98042a87aca6b04cf02c7",
            "public_url": null
        },
        {
            "object": "page",
            "id": "20434608-d5c9-80f1-b2c2-c84d92ca6346",
            "created_time": "2025-05-31T02:22:00.000Z",
            "last_edited_time": "2025-05-31T02:23:00.000Z",
            "created_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "last_edited_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "cover": null,
            "icon": null,
            "parent": {
                "type": "database_id",
                "database_id": "20434608-d5c9-8048-ac7f-d7fe5f300cd3"
            },
            "archived": false,
            "in_trash": false,
            "is_locked": false,
            "properties": {
                "Parent item": {
                    "id": "JW%7BY",
                    "type": "relation",
                    "relation": [],
                    "has_more": false
                },
                "Relation": {
                    "id": "MZlh",
                    "type": "relation",
                    "relation": [
                        {
                            "id": "20434608-d5c9-80b4-bc4a-d1a1b445fe1e"
                        }
                    ],
                    "has_more": false
                },
                "Sub-item": {
                    "id": "%5BhIv",
                    "type": "relation",
                    "relation": [],
                    "has_more": false
                },
                "Tags": {
                    "id": "x%3BAi",
                    "type": "multi_select",
                    "multi_select": [
                        {
                            "id": "3bbd15dd-e836-4f7c-b93a-f3c6a6e42407",
                            "name": "Test",
                            "color": "gray"
                        },
                        {
                            "id": "973d26cb-844e-425d-8ea1-71ce535f8703",
                            "name": "Camp",
                            "color": "orange"
                        }
                    ]
                },
                "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Greet",
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
                            "plain_text": "Greet",
                            "href": null
                        }
                    ]
                },
                "Verification": {
                    "id": "verification",
                    "type": "verification",
                    "verification": {
                        "state": "unverified",
                        "verified_by": null,
                        "date": null
                    }
                },
                "Owner": {
                    "id": "verification_owner",
                    "type": "people",
                    "people": [
                        {
                            "object": "user",
                            "id": "d7592761-a145-4788-ba2c-d67bbfaed77f",
                            "name": "Shirayuki",
                            "avatar_url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/",
                            "type": "person",
                            "person": {
                                "email": "hi@example.com"
                            }
                        }
                    ]
                }
            },
            "url": "https://www.notion.so/Greet-20434608d5c980f1b2c2c84d92ca6346",
            "public_url": null
        },
        {
            "object": "page",
            "id": "20434608-d5c9-8067-8394-fbda69fef1aa",
            "created_time": "2025-05-31T02:22:00.000Z",
            "last_edited_time": "2025-08-31T16:29:00.000Z",
            "created_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "last_edited_by": {
                "object": "user",
                "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
            },
            "cover": null,
            "icon": {
                "type": "custom_emoji",
                "custom_emoji": {
                    "id": "1b134608-d5c9-80da-9f7e-007a84fecebd",
                    "name": "vscode",
                    "url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/a4022676-2463-47b5-bbee-2bc7fd3f7994/Visual_Studio_Code_1.35_icon.svg.png"
                }
            },
            "parent": {
                "type": "database_id",
                "database_id": "20434608-d5c9-8048-ac7f-d7fe5f300cd3"
            },
            "archived": false,
            "in_trash": false,
            "is_locked": false,
            "properties": {
                "Parent item": {
                    "id": "JW%7BY",
                    "type": "relation",
                    "relation": [],
                    "has_more": false
                },
                "Relation": {
                    "id": "MZlh",
                    "type": "relation",
                    "relation": [
                        {
                            "id": "20434608-d5c9-80b4-bc4a-d1a1b445fe1e"
                        }
                    ],
                    "has_more": false
                },
                "Sub-item": {
                    "id": "%5BhIv",
                    "type": "relation",
                    "relation": [
                        {
                            "id": "26034608-d5c9-8042-a87a-ca6b04cf02c7"
                        }
                    ],
                    "has_more": false
                },
                "Tags": {
                    "id": "x%3BAi",
                    "type": "multi_select",
                    "multi_select": [
                        {
                            "id": "3bbd15dd-e836-4f7c-b93a-f3c6a6e42407",
                            "name": "Test",
                            "color": "gray"
                        }
                    ]
                },
                "Name": {
                    "id": "title",
                    "type": "title",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Hello",
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
                            "plain_text": "Hello",
                            "href": null
                        }
                    ]
                },
                "Verification": {
                    "id": "verification",
                    "type": "verification",
                    "verification": {
                        "state": "unverified",
                        "verified_by": null,
                        "date": null
                    }
                },
                "Owner": {
                    "id": "verification_owner",
                    "type": "people",
                    "people": [
                        {
                            "object": "user",
                            "id": "d7592761-a145-4788-ba2c-d67bbfaed77f",
                            "name": "Shirayuki",
                            "avatar_url": "https://s3-us-west-2.amazonaws.com/public.notion-static.com/",
                            "type": "person",
                            "person": {
                                "email": "hi@example.com"
                            }
                        }
                    ]
                }
            },
            "url": "https://www.notion.so/Hello-20434608d5c980678394fbda69fef1aa",
            "public_url": null
        }
    ],
    "next_cursor": null,
    "has_more": false,
    "type": "page_or_database",
    "page_or_database": {},
    "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
    "request_id": "24cc2720-c148-4e21-abf0-ae81794c0eb4"
}
        "#;

        let _: ListResponse<SearchResultItem> = serde_json::from_str(json_data).unwrap();
    }

    #[test]
    fn deserialize_comment_result() {
        let json_data = r#"
        {
            "object": "list",
            "results": [
                {
                    "object": "comment",
                    "id": "1e834608-d5c9-8021-894d-001df60d9340",
                    "parent": {
                        "type": "block_id",
                        "block_id": "1e334608-d5c9-80a4-a8a3-e524a536c43f"
                    },
                    "discussion_id": "1e834608-d5c9-80a2-ab7a-001c2c516cfd",
                    "created_time": "2025-05-03T11:45:00.000Z",
                    "last_edited_time": "2025-05-03T11:53:00.000Z",
                    "created_by": {
                        "object": "user",
                        "id": "426d05f1-ce5c-4077-b206-10fd26daa2a8"
                    },
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
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
                            "plain_text": "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                            "href": null
                        }
                    ]
                },
                {
                    "object": "comment",
                    "id": "1e834608-d5c9-8066-bb14-001df6afc208",
                    "parent": {
                        "type": "block_id",
                        "block_id": "1e334608-d5c9-80a4-a8a3-e524a536c43f"
                    },
                    "discussion_id": "1e834608-d5c9-80a2-ab7a-001c2c516cfd",
                    "created_time": "2025-05-03T11:45:00.000Z",
                    "last_edited_time": "2025-05-03T11:54:00.000Z",
                    "created_by": {
                        "object": "user",
                        "id": "426d05f1-ce5c-4077-b206-10fd26daa2a8"
                    },
                    "rich_text": [
                        {
                            "type": "text",
                            "text": {
                                "content": "Etiam ut lectus non odio lacinia tincidunt.",
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
                            "plain_text": "Etiam ut lectus non odio lacinia tincidunt.",
                            "href": null
                        }
                    ]
                }
            ],
            "next_cursor": null,
            "has_more": false,
            "type": "comment",
            "comment": {},
            "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
            "request_id": "ee3aa7b3-4d66-4775-9afa-54c283306f98"
        }
        "#;

        let result: ListResponse<crate::object::comment::Comment> =
            serde_json::from_str(json_data).unwrap();

        assert_eq!(result.object, "list");

        for result in result.results {
            assert_eq!(result.object, "comment");
        }
    }
}
