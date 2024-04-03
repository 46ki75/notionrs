use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Color {
    FG(ColorFG),
    BG(ColorBG),
}
