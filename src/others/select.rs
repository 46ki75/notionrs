use serde::{Deserialize, Serialize};

use super::color::ColorFG;

#[derive(Debug, Deserialize, Serialize)]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: ColorFG,
}
