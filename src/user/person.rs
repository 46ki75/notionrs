use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    pub object: String, // always "user"
    pub id: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,

    /// always "person"
    pub r#type: String,
    pub person: Option<PersonDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDetail {
    pub email: Option<String>,
}
