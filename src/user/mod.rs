pub mod bot;
pub mod person;

use serde::{Deserialize, Serialize};

use self::{bot::Bot, person::Person};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum User {
    Bot(Bot),
    Person(Person),
}
