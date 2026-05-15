use serde::{Deserialize, Serialize};

/// <https://developers.notion.com/reference/rich-text#equation>
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, notionrs_macro::Setter)]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    ///
    /// e.g., `E = mc^2`
    pub expression: String,
}

crate::impl_display_from_string_field!(Equation, expression);
crate::impl_from_as_ref!(Equation, expression);

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn equation_display_and_from() {
        let eq: Equation = "E = mc^2".into();
        assert_eq!(eq.expression, "E = mc^2");
        assert_eq!(eq.to_string(), "E = mc^2");

        let eq2 = Equation::default().expression("x = y");
        assert_eq!(eq2.expression, "x = y");
    }
}
