use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseRollupProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub rollup: DatabaseRollupDetail,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseRollupDetail {
    pub function: RollupFunction,

    /// The id of the related database property that is rolled up.
    pub relation_property_id: String,

    /// The name of the related database property that is rolled up.
    pub relation_property_name: String,

    /// The id of the rollup property.
    pub rollup_property_id: String,

    /// The name of the rollup property.
    pub rollup_property_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    CountPerGroup,
    Count,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    Unchecked,
    Unique,
    ShowOriginal,
    ShowUnique,
    Sum,
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
    fn deserialize_database_rollup_property() {
        let json_data = r#"
        {
            "id": "%5E%7Cy%3C",
            "name": "Estimated total project time",
            "type": "rollup",
            "rollup": {
                "rollup_property_name": "Days to complete",
                "relation_property_name": "Tasks",
                "rollup_property_id": "\\nyY",
                "relation_property_id": "Y]<y",
                "function": "sum"
            }
        }
        "#;

        let rollup = serde_json::from_str::<DatabaseRollupProperty>(json_data).unwrap();

        assert_eq!(rollup.id, "%5E%7Cy%3C");
        assert_eq!(rollup.name, "Estimated total project time");
        assert_eq!(rollup.rollup.rollup_property_name, "Days to complete");
        assert_eq!(rollup.rollup.relation_property_name, "Tasks");
        assert_eq!(rollup.rollup.rollup_property_id, "\\nyY");
        assert_eq!(rollup.rollup.relation_property_id, "Y]<y");
        assert_eq!(rollup.rollup.function, RollupFunction::Sum);
    }
}
