use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BulletedListItemBlockRequest {
    r#type: String,
    bulleted_list_item: BulletedListItemBlockRequestParams,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct BulletedListItemBlockRequestParams {
    /// The rich text in the bulleted_list_item block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The color of the block.
    pub color: crate::others::color::Color,
}

macro_rules! color_setters {
        ($($color:ident),*) => {
            $(
                pub fn $color(mut self) -> Self {
                    self.bulleted_list_item.color = crate::others::color::Color::$color();
                    self
                }
            )*
        }
    }

impl BulletedListItemBlockRequest {
    pub fn build(self) -> super::BlockRequest {
        super::BlockRequest::BulletedListItem(self)
    }

    pub fn new() -> Self {
        BulletedListItemBlockRequest {
            r#type: "bulleted_list_item".to_string(),
            bulleted_list_item: BulletedListItemBlockRequestParams {
                rich_text: vec![],
                color: crate::others::color::Color::default(),
            },
        }
    }

    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.bulleted_list_item.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: crate::others::color::Color) -> Self {
        self.bulleted_list_item.color = color;
        self
    }

    color_setters! {
        blue,
        brown,
        gray,
        green,
        orange,
        pink,
        purple,
        red,
        yellow,
        blue_background,
        brown_background,
        gray_background,
        green_background,
        orange_background,
        pink_background,
        purple_background,
        red_background,
        yellow_background
    }
}
