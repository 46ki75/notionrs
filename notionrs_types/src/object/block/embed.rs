use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#embed>
///
/// Embed block objects include information about another website displayed within the Notion UI.
/// The embed property contains the following information:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct EmbedBlock {
    /// The link to the website that the embed block displays.
    pub url: String,
}

impl EmbedBlock {
    pub fn url<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.url = url.as_ref().to_string();
        self
    }
}

impl<T> From<T> for EmbedBlock
where
    T: AsRef<str>,
{
    fn from(url: T) -> Self {
        Self::default().url(url)
    }
}

impl std::fmt::Display for EmbedBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.url)
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
