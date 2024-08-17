use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BulletedListItemRequest {
    pub r#type: String,

    pub bulleted_list_item: crate::block::bulleted_list_item::BulletedListItemBlock,
}

impl BulletedListItemRequest {
    pub fn new() -> Self {
        BulletedListItemRequest {
            r#type: "bulleted_list_item".to_string(),
            bulleted_list_item: crate::block::bulleted_list_item::BulletedListItemBlock {
                rich_text: vec![],
                color: crate::others::color::Color::FG(crate::others::color::ColorFG::Default),
            },
        }
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.bulleted_list_item.rich_text = rich_text;
        self
    }

    pub fn default_color(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Default);
        self
    }

    pub fn blue(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Blue);
        self
    }

    pub fn brown(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Brown);
        self
    }

    pub fn gray(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Gray);
        self
    }

    pub fn green(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Green);
        self
    }

    pub fn orange(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Orange);
        self
    }

    pub fn pink(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Pink);
        self
    }

    pub fn purple(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Purple);
        self
    }

    pub fn red(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Red);
        self
    }

    pub fn yellow(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::FG(crate::others::color::ColorFG::Yellow);
        self
    }

    pub fn blue_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::BlueBackground);
        self
    }

    pub fn brown_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::BrownBackground);
        self
    }

    pub fn gray_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::GrayBackground);
        self
    }

    pub fn green_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::GreenBackground);
        self
    }

    pub fn orange_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::OrangeBackground);
        self
    }

    pub fn pink_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::PinkBackground);
        self
    }

    pub fn purple_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::PurpleBackground);
        self
    }

    pub fn red_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::RedBackground);
        self
    }

    pub fn yellow_background(mut self) -> Self {
        self.bulleted_list_item.color =
            crate::others::color::Color::BG(crate::others::color::ColorBG::YellowBackground);
        self
    }
}

impl Default for BulletedListItemRequest {
    fn default() -> Self {
        Self::new()
    }
}
