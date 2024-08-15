use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorFG {
    Default,
    Blue,
    Brown,
    Gray,
    Green,
    Orange,
    Pink,
    Purple,
    Red,
    Yellow,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ColorBG {
    BlueBackground,
    BrownBackground,
    GrayBackground,
    GreenBackground,
    OrangeBackground,
    PinkBackground,
    PurpleBackground,
    RedBackground,
    YellowBackground,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(untagged)]
pub enum Color {
    FG(ColorFG),
    BG(ColorBG),
}
