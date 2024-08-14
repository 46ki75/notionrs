use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Button(PageButtonProperty),
    Checkbox(PageCheckboxProperty),
    CreatedBy(PageCreatedByProperty),
    CreatedTime(PageCreatedTimeProperty),
    Date(PageDateProperty),
    Email(PageEmailProperty),
    Files(PageFilesProperty),
    Formula(PageFormulaProperty),
    LastEditedBy(PageLastEditedByProperty),
    LastEditedTime(PageLastEditedTimeProperty),
    MultiSelect(PageMultiSelectProperty),
    Number(PageNumberProperty),
    People(PagePeopleProperty),
    PhoneNumber(PagePhoneNumberProperty),
    Relation(PageRelationProperty),
    RichText(PageRichTextProperty),
    Rollup(PageRollupProperty),
    Select(PageSelectProperty),
    Status(PageStatusProperty),
    Title(PageTitleProperty),
    UniqueId(PageUniqueIdProperty),
    Url(PageUrlProperty),
}

// # --------------------------------------------------------------------------------
//
// Button
//
// # --------------------------------------------------------------------------------

/// Example checkbox page property value
/// ```json
/// {
///     "Button": {
///         "id": "c%60qZ",
///         "type": "button",
///         "button": {}
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageButtonProperty {
    pub id: String,
    pub button: std::collections::HashMap<String, String>,
}

// # --------------------------------------------------------------------------------
//
// Checkbox
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#checkbox>
///
/// Example checkbox page property value
/// ```json
/// {
///     "Task completed": {
///       "id": "ZI%40W",
///       "type": "checkbox",
///       "checkbox": true
///     }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCheckboxProperty {
    pub id: String,
    pub checkbox: bool,
}

// # --------------------------------------------------------------------------------
//
// CreatedBy
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#created-by>
///
/// Example created_by page property value
///
/// ```json
/// {
///   "Created by": {
///     "id": "fR4s",
///     "type": "created_by",
///     "created_by": {
///       "object": "user",
///       "id": "cb497a8c-1c30-4c22-87af-f8b0c1ee7389",
///       "name": "Sam",
///       "avatar_url": null,
///       "type": "person",
///       "person": {
///         "email": "info@example.com"
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCreatedByProperty {
    pub id: String,
    pub created_by: crate::user::User,
}

// # --------------------------------------------------------------------------------
//
// CreatedTime
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#created-time>
///
/// Example created_time page property value
///
/// ```json
/// {
///   "Created time": {
///     "id": "sv%3Fi",
///     "type": "created_time",
///     "created_time": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageCreatedTimeProperty {
    pub id: String,
    pub created_time: String,
}

// # --------------------------------------------------------------------------------
//
// Date
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#date>
///
/// Example date page property value
///
/// ```json
/// {
///   "Date": {
///     "id": "w%5E%7DO",
///     "type": "date",
///     "date": {
///       "start": "2024-04-04T00:00:00.000+02:00",
///       "end": null,
///       "time_zone": null
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageDateProperty {
    pub id: String,
    pub date: Option<PageDatePropertyParameter>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageDatePropertyParameter {
    start: Option<String>,
    end: Option<String>,
    time_zone: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// Email
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#email>
///
/// Example email page property value
///
/// ```json
/// {
///   "Email": {
///     "id": "rXuf",
///     "type": "email",
///     "email": "hi@example.com"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageEmailProperty {
    pub id: String,
    pub email: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// Files
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#files>
///
/// Example filea page property value
///
/// ```json
/// {
///   "Files & media": {
///     "id": "Q%7Dn%3E",
///     "type": "files",
///     "files": [
///       {
///         "name": "Project Alpha blueprint",
///         "type": "file",
///         "file": {
///           "url": "https://prod-files-secure.s3.us-west-2.amazonaws.com/daa95f86-2d56-4e18-be3b-16d81b31dc0d",
///           "expiry_time": "2024-04-04T10:45:54.308Z"
///         }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageFilesProperty {
    pub id: String,
    pub files: Vec<crate::others::file::File>,
}

// # --------------------------------------------------------------------------------
//
// Formula
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#formula>
///
/// Example formula page property value
///
/// ```json
/// {
///   "Formula": {
///     "type": "formula",
///     "id": "W~%5BW",
///     "formula": { "type": "string", "string": "My Title" }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageFormulaProperty {
    pub id: String,
    pub formula: Formula,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Formula {
    Boolean(FormulaBoolean),
    Date(FormulaDate),
    Number(FormulaNumber),
    String(FormulaString),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaBoolean {
    pub boolean: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaDate {
    pub date: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaNumber {
    pub number: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormulaString {
    pub string: String,
}

// # --------------------------------------------------------------------------------
//
// LastEditedBy
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#last-edited-by>
///
/// Example last_edited_by page property value
///
/// ```json
/// {
///   "CLast edited by": {
///     "id": "fR4s",
///     "type": "last_edited_by",
///     "created_by": {
///       "object": "user",
///       "id": "cb497a8c-1c30-4c22-87af-f8b0c1ee7389",
///       "name": "Sam",
///       "avatar_url": null,
///       "type": "person",
///       "person": {
///         "email": "info@example.com"
///       }
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedByProperty {
    pub id: String,
    pub last_edited_by: crate::user::User,
}

// # --------------------------------------------------------------------------------
//
// LastEditedTime
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#last-edited-time>
///
/// Example last_edited_by page property value
///
/// ```json
/// {
///   "Last edited time": {
///     "id": "sv%3Fi",
///     "type": "last_edited_by",
///     "last_edited_by": "2024-04-03T10:55:00.000Z"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageLastEditedTimeProperty {
    pub id: String,
    pub last_edited_time: String,
}

// # --------------------------------------------------------------------------------
//
// MultiSelect
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#multi-select>
///
/// Example multi_select page property value
///
/// ```json
/// {
///   "Multi-select": {
///     "id": "_bnY",
///     "type": "multi_select",
///     "multi_select": [
///       {
///         "id": "959ba3e3-5a64-4ee6-864b-9e94ddc024d5",
///         "name": "HTML",
///         "color": "orange"
///       },
///       {
///         "id": "f22b05c9-0225-4dee-b25b-db7e63a47e0b",
///         "name": "CSS",
///         "color": "blue"
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageMultiSelectProperty {
    pub id: String,
    pub multi_select: Vec<crate::others::select::Select>,
}

// # --------------------------------------------------------------------------------
//
// Number
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#number>
///
/// Example number page property value
///
/// ```json
/// { "Number": { "type": "number", "id": "%5Chme", "number": 20.0 } }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageNumberProperty {
    pub id: String,
    pub number: Option<f64>,
}

// # --------------------------------------------------------------------------------
//
// People
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#people>
///
/// Example people page property value
///
/// ```json
/// {
///   "Person": {
///     "type": "people",
///     "id": "pAoV",
///     "people": [
///       {
///         "object": "user",
///         "id": "4050d499-9586-4352-85a2-d4cb55a68200",
///         "name": "46ki75",
///         "avatar_url": null,
///         "type": "person",
///         "person": { "email": "46ki75@example.com" }
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PagePeopleProperty {
    pub id: String,
    pub people: Vec<crate::user::User>,
}

// # --------------------------------------------------------------------------------
//
// PhoneNumber
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#phone-number>
///
/// Example phone_number page property value
///
/// ```json
/// {
///   "Checkbox": {
///     "type": "phone_number",
///     "id": "Se%3Dp",
///     "phone_number": "080"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PagePhoneNumberProperty {
    pub id: String,
    pub phone_number: Option<String>,
}

// # --------------------------------------------------------------------------------
//
// Relation
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#relation>
///
/// Example relation page property value
///
/// ```json
/// {
///   "Related": {
///     "id": "b%7D%3Ek",
///     "type": "relation",
///     "relation": [
///       {
///         "id": "669ffc58-9c20-4264-956b-f7f917c58400"
///       }
///     ],
///     "has_more": false
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationProperty {
    pub id: String,
    pub relation: Vec<PageRelationPropertyParameter>,

    /// If a relation has more than 25 references,
    /// then the has_more value for the relation in the response object is true.
    /// If a relation doesn’t exceed the limit, then has_more is false.
    pub has_more: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageRelationPropertyParameter {
    pub id: String,
}

// # --------------------------------------------------------------------------------
//
// RichText
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#rich-text>
///
/// Example rich_text page property value
///
/// ```json
/// {
///   "Text": {
///     "id": "mM%3BV",
///     "type": "rich_text",
///     "rich_text": [
///       {
///         "type": "text",
///         "text": {
///           "content": "My Description",
///           "link": null
///         },
///         "annotations": {
///           "bold": false,
///           "italic": false,
///           "strikethrough": false,
///           "underline": false,
///           "code": false,
///           "color": "default"
///         },
///         "plain_text": "My Description",
///         "href": null
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRichTextProperty {
    pub id: String,
    pub rich_text: Vec<crate::others::rich_text::RichText>,
}

// # --------------------------------------------------------------------------------
//
// Rollup
//
// # --------------------------------------------------------------------------------

// TODO: Implement the Rollup object.
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRollupProperty {
    pub id: String,
}

// # --------------------------------------------------------------------------------
//
// Select
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#select>
///
/// Example select page property value
///
/// ```json
/// {
///   "Select": {
///     "type": "select",
///     "id": "chOy",
///     "select": {
///       "id": "eede87ce-52db-4b16-9931-2bc40687d697",
///       "name": "TODO",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageSelectProperty {
    pub id: String,
    pub select: Option<crate::others::select::Select>,
}

// # --------------------------------------------------------------------------------
//
// Status
//
// # --------------------------------------------------------------------------------
/// <https://developers.notion.com/reference/page-property-values#status>
///
/// Example status page property value
///
/// ```json
/// {
///   "Status": {
///     "type": "status",
///     "id": "xx%7Cd",
///     "status": {
///       "id": "4a1accbf-6716-4cf2-9034-5877581fc5f6",
///       "name": "Not started",
///       "color": "default"
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageStatusProperty {
    pub id: String,
    pub status: crate::others::select::Select,
}

// # --------------------------------------------------------------------------------
//
// Title
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#title>
///
/// Example title page property value
///
/// ```json
/// {
///   "title": {
///     "id": "title",
///     "type": "title",
///     "title": [
///       {
///         "type": "text",
///         "text": {
///           "content": "My Title",
///           "link": null
///         },
///         "annotations": {
///           "bold": false,
///           "italic": false,
///           "strikethrough": false,
///           "underline": false,
///           "code": false,
///           "color": "default"
///         },
///         "plain_text": "My Title",
///         "href": null
///       }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageTitleProperty {
    pub id: String,
    pub title: Vec<crate::others::rich_text::RichText>,
}

// # --------------------------------------------------------------------------------
//
// UniqueId
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#unique_id>
///
/// Example unique_id page property value
///
/// ```json
/// {
///   "ID": {
///     "id": "mBKy",
///     "type": "unique_id",
///     "unique_id": {
///       "prefix": "TES",
///       "number": 434
///     }
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdProperty {
    pub id: String,
    pub unique_id: PageUniqueIdPropertyParameter,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PageUniqueIdPropertyParameter {
    pub prefix: Option<String>,
    pub number: u64,
}

// # --------------------------------------------------------------------------------
//
// Url
//
// # --------------------------------------------------------------------------------

/// <https://developers.notion.com/reference/page-property-values#url>
///
/// Example url page property value
///
/// ```json
/// {
///   "URL": {
///     "type": "url",
///     "id": "h_AH",
///     "url": "https://developers.notion.com/reference/page-property-values#url"
///   }
/// }
/// ```
#[derive(Debug, Deserialize, Serialize)]
pub struct PageUrlProperty {
    pub id: String,
    pub url: Option<String>,
}
