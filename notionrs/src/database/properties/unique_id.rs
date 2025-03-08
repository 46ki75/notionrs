use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseUniqueIdProperty {
    /// Property Identifier
    #[serde(skip_serializing)]
    pub id: Option<String>,

    /// Modify the value of this field when updating the column name of the property.
    #[serde(skip_serializing)]
    pub name: String,

    /// Although it is not explicitly stated in the official documentation,
    /// you can add a description to the property by specifying this.
    #[serde(skip_serializing)]
    pub description: Option<String>,

    pub unique_id: DatabaseUniqueIdPropertyItem,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseUniqueIdPropertyItem {
    pub prefix: Option<String>,
}
