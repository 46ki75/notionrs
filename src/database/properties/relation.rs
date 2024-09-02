use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseRelationProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub relation: DatabaseRelationDetail,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseRelationDetail {
    /// The database that the relation property refers to.
    /// The corresponding linked page values must belong to the database in order to be valid.
    pub database_id: String,

    /// The id of the corresponding property that is updated in the related database when this property is changed.
    pub synced_property_id: String,

    /// The name of the corresponding property that is updated in the related database when this property is changed.
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
            "id": "~pex",
            "name": "Projects",
            "type": "relation",
            "relation": {
                "database_id": "6c4240a9-a3ce-413e-9fd0-8a51a4d0a49b",
                "synced_property_name": "Tasks",
                "synced_property_id": "JU]K"
            }
        }
        "#;

        let relation = serde_json::from_str::<DatabaseRelationProperty>(json_data).unwrap();

        assert_eq!(relation.id, "~pex");
        assert_eq!(relation.name, "Projects");
        assert_eq!(
            relation.relation.database_id,
            "6c4240a9-a3ce-413e-9fd0-8a51a4d0a49b"
        );
        assert_eq!(relation.relation.synced_property_name, "Tasks");
        assert_eq!(relation.relation.synced_property_id, "JU]K");
    }
}
