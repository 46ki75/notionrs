use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#equation>
///
/// Equation block objects are represented as children of paragraph blocks.
/// They are nested within a rich text object and contain
/// the following information within the equation property:
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct EquationBlock {
    /// A KaTeX compatible string.
    pub expression: String,
}

impl EquationBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Equation { equation: self }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn expression<T>(mut self, url: T) -> Self
    where
        T: AsRef<str>,
    {
        self.expression = url.as_ref().to_string();
        self
    }
}

impl<T> From<T> for EquationBlock
where
    T: AsRef<str>,
{
    fn from(expression: T) -> Self {
        Self::new().expression(expression)
    }
}

// # --------------------------------------------------------------------------------
//
// unit test
//
// # --------------------------------------------------------------------------------

#[cfg(test)]
mod unit_tests {

    use super::EquationBlock;

    #[test]
    fn deserialize_block_equation() {
        let json_data = r#"
        {
            "expression": "W_q = \\frac{\\lambda}{\\mu (\\mu - \\lambda)}"
        }
        "#;

        let child_page = serde_json::from_str::<EquationBlock>(json_data).unwrap();

        assert_eq!(
            child_page.expression,
            r"W_q = \frac{\lambda}{\mu (\mu - \lambda)}"
        )
    }
}
