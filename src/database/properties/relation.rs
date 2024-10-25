use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct DatabaseRelationProperty {
    #[serde(skip_serializing)]
    pub id: Option<String>,

    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing)]
    pub description: Option<String>,

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
