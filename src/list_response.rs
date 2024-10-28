use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ListResponse<T> {
    /// always "list"
    pub object: String,
    pub results: Vec<T>,

    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum SearchResultItem {
    Page(crate::page::PageResponse),
    Database(crate::database::DatabaseResponse),
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
                    "id": "535be925-aa1e-48ed-b9b5-449065699b91",
                    "created_time": "2024-10-28T10:42:00.000Z",
                    "last_edited_time": "2024-10-28T13:33:00.000Z",
                    "created_by": {
                        "object": "user",
                        "id": "a1d8eb4a-45d3-49a0-97ba-fd96b83d82f1"
                    },
                    "last_edited_by": {
                        "object": "user",
                        "id": "a1d8eb4a-45d3-49a0-97ba-fd96b83d82f1"
                    },
                    "cover": null,
                    "icon": null,
                    "parent": {
                        "type": "database_id",
                        "database_id": "12da03d7-9b26-8075-b21e-c54373a7d875"
                    },
                    "archived": false,
                    "in_trash": false,
                    "properties": {
                        "Date": {
                            "id": "qjHv",
                            "type": "date",
                            "date": {
                                "start": "2024-10-28",
                                "end": "2024-10-30",
                                "time_zone": null
                            }
                        },
                        "Last edited by": {
                            "id": "ruuJ",
                            "type": "last_edited_by",
                            "last_edited_by": {
                                "object": "user",
                                "id": "a1d8eb4a-45d3-49a0-97ba-fd96b83d82f1",
                                "name": "shirayuki",
                                "avatar_url": "https://lh3.googleusercontent.com/",
                                "type": "person",
                                "person": {
                                    "email": "hi@example.com"
                                }
                            }
                        },
                        "Select": {
                            "id": "t%5Cy%3F",
                            "type": "select",
                            "select": {
                                "id": "7bfa67ff-4055-4368-a615-464a39d169d4",
                                "name": "My Selection",
                                "color": "gray"
                            }
                        },
                        "Files & media": {
                            "id": "xeVg",
                            "type": "files",
                            "files": [
                                {
                                    "name": "0244.jpg",
                                    "type": "file",
                                    "file": {
                                        "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                                        "expiry_time": "2024-10-28T14:42:22.849Z"
                                    }
                                },
                                {
                                    "name": "0294.png",
                                    "type": "file",
                                    "file": {
                                        "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/",
                                        "expiry_time": "2024-10-28T14:42:22.953Z"
                                    }
                                }
                            ]
                        },
                        "Related Untitled Database": {
                            "id": "y%40IV",
                            "type": "relation",
                            "relation": [
                                {
                                    "id": "535be925-aa1e-48ed-b9b5-449065699b91"
                                }
                            ],
                            "has_more": false
                        },
                        "URL": {
                            "id": "%7B%3F%3AM",
                            "type": "url",
                            "url": "https://github.com/46ki75/notionrs"
                        },
                        "Person": {
                            "id": "%7BaDy",
                            "type": "people",
                            "people": [
                                {
                                    "object": "user",
                                    "id": "a1d8eb4a-45d3-49a0-97ba-fd96b83d82f1",
                                    "name": "shirayuki",
                                    "avatar_url": "https://lh3.googleusercontent.com/",
                                    "type": "person",
                                    "person": {
                                        "email": "hi@example.com"
                                    }
                                }
                            ]
                        },
                        "Formula": {
                            "id": "%7BrzW",
                            "type": "formula",
                            "formula": {
                                "type": "string",
                                "string": null
                            }
                        },
                        "Name": {
                            "id": "title",
                            "type": "title",
                            "title": [
                                {
                                    "type": "text",
                                    "text": {
                                        "content": "aaa",
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
                                    "plain_text": "aaa",
                                    "href": null
                                }
                            ]
                        }
                    },
                    "url": "https://www.notion.so/aaa-535be925aa1e48edb9b5449065699b91",
                    "public_url": null
                },
                {
                    "object": "database",
                    "id": "3d9f2e45-df78-442d-bb73-e79823018866",
                    "cover": null,
                    "icon": null,
                    "created_time": "2024-07-09T18:33:00.000Z",
                    "created_by": {
                        "object": "user",
                        "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
                    },
                    "last_edited_by": {
                        "object": "user",
                        "id": "d7592761-a145-4788-ba2c-d67bbfaed77f"
                    },
                    "last_edited_time": "2024-10-24T19:25:00.000Z",
                    "title": [
                        {
                            "type": "text",
                            "text": {
                                "content": "query_databse",
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
                            "plain_text": "query_databse",
                            "href": null
                        }
                    ],
                    "description": [],
                    "is_inline": false,
                    "properties": {
                        "Files & media": {
                            "id": "%3AlnV",
                            "name": "Files & media",
                            "description": "",
                            "type": "files",
                            "files": {}
                        },
                        "User": {
                            "id": "%40SAb",
                            "name": "User",
                            "description": "k",
                            "type": "people",
                            "people": {}
                        },
                        "email": {
                            "id": "JW%5ED",
                            "name": "email",
                            "type": "email",
                            "email": {}
                        },
                        "Date": {
                            "id": "NS%3F%5E",
                            "name": "Date",
                            "description": "",
                            "type": "date",
                            "date": {}
                        },
                        "Checkbox": {
                            "id": "XjE%60",
                            "name": "Checkbox",
                            "type": "checkbox",
                            "checkbox": {}
                        },
                        "URL": {
                            "id": "%5E%3D%7C%7D",
                            "name": "URL",
                            "type": "url",
                            "url": {}
                        },
                        "Phone Number": {
                            "id": "bZy%3C",
                            "name": "Phone Number",
                            "type": "phone_number",
                            "phone_number": {}
                        },
                        "Text": {
                            "id": "dDGD",
                            "name": "Text",
                            "type": "rich_text",
                            "rich_text": {}
                        },
                        "Select": {
                            "id": "djaK",
                            "name": "Select",
                            "type": "select",
                            "select": {
                                "options": [
                                    {
                                        "id": "30246975-11fc-4479-96fb-36fdf2af96f4",
                                        "name": "My Select",
                                        "color": "pink",
                                        "description": null
                                    }
                                ]
                            }
                        },
                        "Created time": {
                            "id": "gSYD",
                            "name": "Created time",
                            "type": "created_time",
                            "created_time": {}
                        },
                        "Button": {
                            "id": "jWe~",
                            "name": "Button",
                            "type": "button",
                            "button": {}
                        },
                        "Relation": {
                            "id": "jYnf",
                            "name": "Relation",
                            "type": "relation",
                            "relation": {
                                "database_id": "3d9f2e45-df78-442d-bb73-e79823018866",
                                "type": "single_property",
                                "single_property": {}
                            }
                        },
                        "LastUpdatedBy": {
                            "id": "k%3CJ%5B",
                            "name": "LastUpdatedBy",
                            "type": "last_edited_by",
                            "last_edited_by": {}
                        },
                        "ID": {
                            "id": "lLae",
                            "name": "ID",
                            "type": "unique_id",
                            "unique_id": {
                                "prefix": null
                            }
                        },
                        "Multi-select": {
                            "id": "oydx",
                            "name": "Multi-select",
                            "type": "multi_select",
                            "multi_select": {
                                "options": [
                                    {
                                        "id": "8785d616-fa83-4bd1-9f7b-19ef4afb91be",
                                        "name": "Test",
                                        "color": "default",
                                        "description": null
                                    },
                                    {
                                        "id": "ec2067dd-4544-4945-aa5c-53b54d97eea7",
                                        "name": "My Tag",
                                        "color": "red",
                                        "description": null
                                    },
                                    {
                                        "id": "b649c486-0e4b-4790-8799-8d99e990c2bb",
                                        "name": "基本設計",
                                        "color": "purple",
                                        "description": null
                                    }
                                ]
                            }
                        },
                        "LastUpdatedAt": {
                            "id": "uhCI",
                            "name": "LastUpdatedAt",
                            "type": "last_edited_time",
                            "last_edited_time": {}
                        },
                        "formula": {
                            "id": "%7CR%5BK",
                            "name": "formula",
                            "type": "formula",
                            "formula": {
                                "expression": "{{notion:block_property:~B%7BT:00000000-0000-0000-0000-000000000000:906620e1-4b96-4427-bba0-af8b28119cd7}}/2"
                            }
                        },
                        "CreatedBy": {
                            "id": "%7Cy~C",
                            "name": "CreatedBy",
                            "description": "",
                            "type": "created_by",
                            "created_by": {}
                        },
                        "Status": {
                            "id": "%7D%3DAB",
                            "name": "Status",
                            "type": "status",
                            "status": {
                                "options": [
                                    {
                                        "id": "66e54a14-9da3-4e64-8e99-3e4d2d77702e",
                                        "name": "Not",
                                        "color": "default",
                                        "description": null
                                    },
                                    {
                                        "id": "8c690548-69f6-49b2-9edb-41129a4e52fc",
                                        "name": "In Progress",
                                        "color": "blue",
                                        "description": null
                                    },
                                    {
                                        "id": "73daa93e-c863-464b-873d-dbd203855b8b",
                                        "name": "Done",
                                        "color": "green",
                                        "description": null
                                    }
                                ],
                                "groups": [
                                    {
                                        "id": "e6fa4c2f-2e05-4ba2-a2a0-1765f5cdb1a1",
                                        "name": "To-do",
                                        "color": "gray",
                                        "option_ids": [
                                            "66e54a14-9da3-4e64-8e99-3e4d2d77702e"
                                        ]
                                    },
                                    {
                                        "id": "88bc4303-8d8b-49f2-80cb-097098b1ea01",
                                        "name": "In progress",
                                        "color": "blue",
                                        "option_ids": [
                                            "8c690548-69f6-49b2-9edb-41129a4e52fc"
                                        ]
                                    },
                                    {
                                        "id": "931e4f1b-a99c-485d-8802-918df637ed10",
                                        "name": "Complete",
                                        "color": "green",
                                        "option_ids": [
                                            "73daa93e-c863-464b-873d-dbd203855b8b"
                                        ]
                                    }
                                ]
                            }
                        },
                        "Rollup": {
                            "id": "%7Dqcp",
                            "name": "Rollup",
                            "type": "rollup",
                            "rollup": {
                                "rollup_property_name": "Title",
                                "relation_property_name": "Relation",
                                "rollup_property_id": "title",
                                "relation_property_id": "jYnf",
                                "function": "show_original"
                            }
                        },
                        "Number": {
                            "id": "~B%7BT",
                            "name": "Number",
                            "description": "",
                            "type": "number",
                            "number": {
                                "format": "number"
                            }
                        },
                        "Title": {
                            "id": "title",
                            "name": "Title",
                            "type": "title",
                            "title": {}
                        }
                    },
                    "parent": {
                        "type": "page_id",
                        "page_id": "03bbcc29-2cea-49f7-bd58-4dc7d7fa679f"
                    },
                    "url": "https://www.notion.so/3d9f2e45df78442dbb73e79823018866",
                    "public_url": null,
                    "archived": false,
                    "in_trash": false
                }
            ],
            "next_cursor": null,
            "has_more": false,
            "type": "page_or_database",
            "page_or_database": {},
            "developer_survey": "https://notionup.typeform.com/to/bllBsoI4?utm_source=postman",
            "request_id": "acbffb71-9090-4ed5-b666-35e287bdee0c"
        }
        "#;

        let result: ListResponse<SearchResultItem> = serde_json::from_str(json_data).unwrap();
        assert_eq!(result.object, "list");
        assert!(matches!(result.results[0], SearchResultItem::Page(_)));
        assert!(matches!(result.results[1], SearchResultItem::Database(_)));
    }
}
