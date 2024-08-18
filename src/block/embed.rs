use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#embed>
///
/// Embed block objects include information about another website displayed within the Notion UI.
/// The embed property contains the following information:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EmbedBlock {
    /// The link to the website that the embed block displays.
    pub url: String,
}

impl EmbedBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Embed { embed: self }
    }

    pub fn new() -> Self {
        Self::default()
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::EmbedBlock;

    #[test]
    fn deserialize_block_embed() {
        let json_data = r#"
        {
            "url": "example.com"
        }
        "#;

        let embed: EmbedBlock = serde_json::from_str::<EmbedBlock>(json_data).unwrap();

        assert_eq!(embed.url, "example.com")
    }
}
