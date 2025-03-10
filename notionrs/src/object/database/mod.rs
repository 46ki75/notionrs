use serde::{Deserialize, Serialize};

pub mod button;
pub mod checkbox;
pub mod created_by;
pub mod created_time;
pub mod date;
pub mod email;
pub mod files;
pub mod formula;
pub mod last_edited_by;
pub mod last_edited_time;
pub mod multi_select;
pub mod number;
pub mod people;
pub mod phone_number;
pub mod relation;
pub mod rich_text;
pub mod rollup;
pub mod select;
pub mod status;
pub mod title;
pub mod unique_id;
pub mod url;
pub mod verification;

pub use {
    button::DatabaseButtonProperty, checkbox::DatabaseCheckboxProperty,
    created_by::DatabaseCreatedByProperty, created_time::DatabaseCreatedTimeProperty,
    date::DatabaseDateProperty, email::DatabaseEmailProperty, files::DatabaseFilesProperty,
    formula::DatabaseFormulaProperty, last_edited_by::DatabaseLastEditedByProperty,
    last_edited_time::DatabaseLastEditedTimeProperty, multi_select::DatabaseMultiSelectProperty,
    number::DatabaseNumberProperty, people::DatabasePeopleProperty,
    phone_number::DatabasePhoneNumberProperty, relation::DatabaseRelationProperty,
    rich_text::DatabaseRichTextProperty, rollup::*, select::DatabaseSelectProperty,
    status::DatabaseStatusProperty, title::DatabaseTitleProperty,
    unique_id::DatabaseUniqueIdProperty, url::DatabaseUrlProperty,
    verification::DatabaseVerificationProperty,
};

#[derive(Debug, Deserialize, Serialize, Clone, notionrs_macro::Setter)]
pub struct DatabaseResponse {
    pub id: String,

    pub created_time: String,

    pub last_edited_time: String,

    pub created_by: crate::object::user::User,

    pub last_edited_by: crate::object::user::User,

    pub cover: Option<crate::object::file::File>,

    pub icon: Option<crate::object::icon::Icon>,

    pub url: String,

    pub public_url: Option<String>,

    pub archived: bool,

    pub in_trash: bool,

    pub is_inline: bool,

    pub title: Vec<crate::object::rich_text::RichText>,

    pub description: Vec<crate::object::rich_text::RichText>,

    pub properties: std::collections::HashMap<String, DatabaseProperty>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default, notionrs_macro::Setter)]
pub struct Sort {
    /// Specify the name of the property
    pub property: String,

    /// Specify the sort order
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

impl Sort {
    /// Helper function to create an ascending order filter
    /// - property_name: The name of the database property to sort by
    pub fn asc<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            property: property_name.as_ref().to_string(),
            direction: SortDirection::Ascending,
        }
    }

    /// Helper function to create a descending order filter
    /// - property_name: The name of the database property to sort by
    pub fn desc<T>(property_name: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            property: property_name.as_ref().to_string(),
            direction: SortDirection::Descending,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DatabaseProperty {
    Button(button::DatabaseButtonProperty),
    Checkbox(checkbox::DatabaseCheckboxProperty),
    CreatedBy(created_by::DatabaseCreatedByProperty),
    CreatedTime(created_time::DatabaseCreatedTimeProperty),
    Date(date::DatabaseDateProperty),
    Email(email::DatabaseEmailProperty),
    Files(files::DatabaseFilesProperty),
    Formula(formula::DatabaseFormulaProperty),
    LastEditedBy(last_edited_by::DatabaseLastEditedByProperty),
    LastEditedTime(last_edited_time::DatabaseLastEditedTimeProperty),
    MultiSelect(multi_select::DatabaseMultiSelectProperty),
    Number(number::DatabaseNumberProperty),
    People(people::DatabasePeopleProperty),
    PhoneNumber(phone_number::DatabasePhoneNumberProperty),
    Relation(relation::DatabaseRelationProperty),
    RichText(rich_text::DatabaseRichTextProperty),
    Rollup(rollup::DatabaseRollupProperty),
    Select(select::DatabaseSelectProperty),
    Status(status::DatabaseStatusProperty),
    Title(title::DatabaseTitleProperty),
    UniqueId(unique_id::DatabaseUniqueIdProperty),
    Url(url::DatabaseUrlProperty),
    Verification(verification::DatabaseVerificationProperty),
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
    fn serialize_database_sort() {
        let sort = Sort::asc("My Property");

        // Expected JSON output
        let expected_json = r#"{
            "property": "My Property",
            "direction": "ascending"
        }"#;

        // Serialize the Sort struct to JSON
        let serialized = serde_json::to_string(&sort).expect("Failed to serialize Sort");

        // Parse the expected JSON to ensure formatting matches
        let expected: serde_json::Value =
            serde_json::from_str(expected_json).expect("Failed to parse expected JSON");
        let serialized_value: serde_json::Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized JSON");

        // Compare the serialized value with the expected value
        assert_eq!(
            serialized_value, expected,
            "Serialized JSON does not match the expected JSON"
        );
    }

    #[test]
    pub fn aa() {
        let json_data = r#"
        {
            "id": "~B%7BT",
            "name": "Number",
            "description": "",
            "type": "number",
            "number": {
                "format": "number"
            }
        }
        "#;

        let number = serde_json::from_str::<DatabaseProperty>(json_data).unwrap();

        match number {
            DatabaseProperty::Number(num) => {
                assert_eq!(num.id, Some("~B%7BT".to_string()));
                assert_eq!(num.name, "Number");
                assert_eq!(
                    num.number.format,
                    crate::object::database::number::NumberFormat::Number
                );
            }
            _ => {
                panic!(
                    "A different variant was detected, although a DatabaseProperty::Number variant was expected."
                )
            }
        }
    }
}
