use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseRelationProperty {
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

    /// An empty object (`{}`)
    pub relation: DatabaseRelationDetail,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseRelationDetail {
    /// The database that the relation property refers to.
    /// The corresponding linked page values must belong to the database in order to be valid.
    pub database_id: String,

    /// Used when creating a one-way relation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_property: Option<std::collections::HashMap<(), ()>>,

    /// Used when creating a two-way relation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dual_property: Option<DatabaseRelationDualProperty>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseRelationDualProperty {
    /// The ID of the property for creating a two-way relation.
    pub synced_property_id: String,

    /// The database column name of the property for creating a two-way relation.
    pub synced_property_name: String,
}

impl DatabaseRelationProperty {
    /// Modify the value of this field when updating the column name of the property.
    pub fn name<T>(mut self, name: T) -> Self
    where
        T: AsRef<str>,
    {
        self.name = name.as_ref().to_string();
        self
    }

    pub fn create_one_way_relation<T>(database_id: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            relation: DatabaseRelationDetail {
                database_id: database_id.as_ref().to_string(),
                single_property: Some(std::collections::HashMap::new()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn create_tow_way_relation<S, T, U>(
        database_id: S,
        synced_property_id: T,
        synced_property_name: U,
    ) -> Self
    where
        S: AsRef<str>,
        T: AsRef<str>,
        U: AsRef<str>,
    {
        Self {
            relation: DatabaseRelationDetail {
                database_id: database_id.as_ref().to_string(),
                dual_property: Some(DatabaseRelationDualProperty {
                    synced_property_id: synced_property_id.as_ref().to_string(),
                    synced_property_name: synced_property_name.as_ref().to_string(),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
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
    fn deserialize_database_relation_property() {
        let json_data = r#"
        {
            "id": "VGw%7B",
            "name": "Relation",
            "type": "relation",
            "relation": {
                "database_id": "12aa03d7-9b26-8158-b34b-d0e7ceec0d15",
                "type": "dual_property",
                "dual_property": {
                    "synced_property_name": "Related to Untitled Database (Relation)",
                    "synced_property_id": "csu%5B"
                }
            }
        }
        "#;

        let relation = serde_json::from_str::<DatabaseRelationProperty>(json_data).unwrap();

        assert_eq!(relation.id, Some("VGw%7B".to_string()));
        assert_eq!(relation.name, "Relation");
        assert_eq!(
            relation.relation.database_id,
            "12aa03d7-9b26-8158-b34b-d0e7ceec0d15"
        );
        assert_eq!(
            relation
                .relation
                .dual_property
                .clone()
                .unwrap()
                .synced_property_name,
            "Related to Untitled Database (Relation)"
        );
        assert_eq!(
            relation.relation.dual_property.unwrap().synced_property_id,
            "csu%5B"
        );
    }
}
