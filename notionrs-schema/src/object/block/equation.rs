use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#equation>
///
/// Equation block objects are represented as children of paragraph blocks.
/// They are nested within a rich text object and contain
/// the following information within the equation property:
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct EquationBlock {
    /// A KaTeX compatible string.
    pub expression: String,
}

impl EquationBlock {
    pub fn expression<T>(mut self, expression: T) -> Self
    where
        T: AsRef<str>,
    {
        self.expression = expression.as_ref().to_string();
        self
    }
}

impl<T> From<T> for EquationBlock
where
    T: AsRef<str>,
{
    fn from(expression: T) -> Self {
        Self::default().expression(expression)
    }
}

impl std::fmt::Display for EquationBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.expression)
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
