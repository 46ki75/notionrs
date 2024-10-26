use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Equation {
    pub expression: String,
}

impl Equation {
    pub fn expression<T>(mut self, expression: T) -> Self
    where
        T: AsRef<str>,
    {
        self.expression = expression.as_ref().to_string();
        self
    }
}

impl<T> From<T> for Equation
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        Self::default().expression(value)
    }
}
