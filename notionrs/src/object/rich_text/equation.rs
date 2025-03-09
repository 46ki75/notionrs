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
