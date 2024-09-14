use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseStatusProperty {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: DatabaseSelectOptionProperty,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseSelectOptionProperty {
    options: Vec<crate::others::select::Select>,
    groups: Vec<crate::others::select::SelectGroup>,
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
    fn deserialize_database_status_property() {
        let json_data = r#"
        {
            "id": "biOx",
            "name": "Status",
            "type": "status",
            "status": {
                "options": [
                    {
                        "id": "034ece9a-384d-4d1f-97f7-7f685b29ae9b",
                        "name": "Not started",
                        "color": "default"
                    },
                    {
                        "id": "330aeafb-598c-4e1c-bc13-1148aa5963d3",
                        "name": "In progress",
                        "color": "blue"
                    },
                    {
                        "id": "497e64fb-01e2-41ef-ae2d-8a87a3bb51da",
                        "name": "Done",
                        "color": "green"
                    }
                ],
                "groups": [
                    {
                        "id": "b9d42483-e576-4858-a26f-ed940a5f678f",
                        "name": "To-do",
                        "color": "gray",
                        "option_ids": [
                        "034ece9a-384d-4d1f-97f7-7f685b29ae9b"
                        ]
                    },
                    {
                        "id": "cf4952eb-1265-46ec-86ab-4bded4fa2e3b",
                        "name": "In progress",
                        "color": "blue",
                        "option_ids": [
                        "330aeafb-598c-4e1c-bc13-1148aa5963d3"
                        ]
                    },
                    {
                        "id": "4fa7348e-ae74-46d9-9585-e773caca6f40",
                        "name": "Complete",
                        "color": "green",
                        "option_ids": [
                        "497e64fb-01e2-41ef-ae2d-8a87a3bb51da"
                        ]
                    }
                ]
            }
        }
        "#;

        let status = serde_json::from_str::<DatabaseStatusProperty>(json_data).unwrap();

        assert_eq!(status.id, "biOx");
        assert_eq!(status.name, "Status");

        let options = &status.status.options;
        assert_eq!(options.len(), 3);

        assert_eq!(options[0].id, "034ece9a-384d-4d1f-97f7-7f685b29ae9b");
        assert_eq!(options[0].name, "Not started");
        assert_eq!(options[0].color, crate::others::color::ColorFG::Default);

        assert_eq!(options[1].id, "330aeafb-598c-4e1c-bc13-1148aa5963d3");
        assert_eq!(options[1].name, "In progress");
        assert_eq!(options[1].color, crate::others::color::ColorFG::Blue);

        assert_eq!(options[2].id, "497e64fb-01e2-41ef-ae2d-8a87a3bb51da");
        assert_eq!(options[2].name, "Done");
        assert_eq!(options[2].color, crate::others::color::ColorFG::Green);

        let groups = &status.status.groups;
        assert_eq!(groups.len(), 3);

        assert_eq!(groups[0].id, "b9d42483-e576-4858-a26f-ed940a5f678f");
        assert_eq!(groups[0].name, "To-do");
        assert_eq!(groups[0].color, crate::others::color::ColorFG::Gray);
        assert_eq!(
            groups[0].option_ids,
            vec!["034ece9a-384d-4d1f-97f7-7f685b29ae9b"]
        );

        assert_eq!(groups[1].id, "cf4952eb-1265-46ec-86ab-4bded4fa2e3b");
        assert_eq!(groups[1].name, "In progress");
        assert_eq!(groups[1].color, crate::others::color::ColorFG::Blue);
        assert_eq!(
            groups[1].option_ids,
            vec!["330aeafb-598c-4e1c-bc13-1148aa5963d3"]
        );

        assert_eq!(groups[2].id, "4fa7348e-ae74-46d9-9585-e773caca6f40");
        assert_eq!(groups[2].name, "Complete");
        assert_eq!(groups[2].color, crate::others::color::ColorFG::Green);
        assert_eq!(
            groups[2].option_ids,
            vec!["497e64fb-01e2-41ef-ae2d-8a87a3bb51da"]
        );
    }
}