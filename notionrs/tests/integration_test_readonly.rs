mod readonly;

use notionrs_types::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IntegrationTestReadOnlyDataSourceSchema {
    #[serde(rename = "ID")]
    pub id: PageUniqueIdProperty,

    #[serde(rename = "My Title")]
    pub my_title: PageTitleProperty,

    #[serde(rename = "Text")]
    pub text: PageRichTextProperty,

    #[serde(rename = "URL")]
    pub url: PageUrlProperty,

    /// ## Options
    ///
    /// - `Page`
    /// - `Block`
    /// - `Comment`
    #[serde(rename = "API Type")]
    pub api_type: PageSelectProperty,

    /// ## Options
    ///
    /// - To-do
    ///   - `Not started`
    /// - In progress
    ///   - `In progress`
    /// - Completed
    ///   - `Done`
    #[serde(rename = "Status")]
    pub status: PageStatusProperty,

    /// ## Options
    ///
    /// - `Something`
    /// - `Another thing`
    /// - `Yet another thing`
    /// - `Last thing`
    #[serde(rename = "Multi-select")]
    pub multi_select: PageMultiSelectProperty,

    #[serde(rename = "Checkbox")]
    pub checkbox: PageCheckboxProperty,

    #[serde(rename = "My Date")]
    pub my_date: PageDateProperty,

    #[serde(rename = "User")]
    pub user: PagePeopleProperty,

    #[serde(rename = "Phone Number")]
    pub phone_number: PagePhoneNumberProperty,

    #[serde(rename = "Files & media")]
    pub files_and_media: PageFilesProperty,

    #[serde(rename = "Number")]
    pub number: PageNumberProperty,

    #[serde(rename = "Related Read-only: Integration Test")]
    pub related_read_only_integration_test: PageRelationProperty,

    #[serde(rename = "Related back to Read-only: Integration Test")]
    pub related_back_to_read_only_integration_test: PageRelationProperty,

    #[serde(rename = "Rollup")]
    pub rollup: PageRollupProperty,

    #[serde(rename = "Place")]
    pub place: PagePlaceProperty,

    #[serde(rename = "Created time")]
    pub created_time: PageCreatedTimeProperty,

    #[serde(rename = "Updated time")]
    pub updated_time: PageLastEditedTimeProperty,

    #[serde(rename = "Created by")]
    pub updated_by: PageCreatedByProperty,

    #[serde(rename = "Last edited by")]
    pub last_edited_by: PageLastEditedByProperty,
}
