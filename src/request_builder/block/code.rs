use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CodeBlockRequest {
    r#type: String,
    code: CodeBlockRequestParams,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CodeBlockRequestParams {
    /// The rich text in the caption of the code block.
    pub caption: Vec<crate::others::rich_text::RichText>,

    /// The rich text in the code block.
    pub rich_text: Vec<crate::others::rich_text::RichText>,

    /// The language of the code contained in the code block.
    pub language: crate::others::language::Language,
}

impl CodeBlockRequest {
    pub fn build(self) -> super::BlockRequest {
        super::BlockRequest::Code(self)
    }

    pub fn new() -> Self {
        CodeBlockRequest {
            r#type: "code".to_string(),
            code: CodeBlockRequestParams::default(),
        }
    }

    pub fn caption(mut self, caption: Vec<crate::others::rich_text::RichText>) -> Self {
        self.code.rich_text = caption;
        self
    }

    /// The contents of the code.
    pub fn rich_text(mut self, rich_text: Vec<crate::others::rich_text::RichText>) -> Self {
        self.code.rich_text = rich_text;
        self
    }

    pub fn language(mut self, language: crate::others::language::Language) -> Self {
        self.code.language = language;
        self
    }
}
