use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
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
