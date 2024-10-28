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
};

#[derive(Debug, Deserialize, Serialize)]
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
                    crate::database::properties::number::NumberFormat::Number
                );
            }
            _ => {
                panic!("A different variant was detected, although a DatabaseProperty::Number variant was expected.")
            }
        }
    }
}
