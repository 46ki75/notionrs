use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/block#equation>
///
/// Equation block objects are represented as children of paragraph blocks.
/// They are nested within a rich text object and contain
/// the following information within the equation property:
#[derive(Deserialize, Serialize, Debug)]
pub struct EquationBlock {
    /// A KaTeX compatible string.
    pub expression: String,
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
