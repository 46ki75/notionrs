use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ColumnBlock {
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl ColumnBlock {
    pub fn new() -> Self {
        ColumnBlock {
            children: Some(vec![]),
        }
    }

    pub fn children(mut self, children: Vec<super::Block>) -> Self {
        self.children = Some(children);
        self
    }
}

impl std::fmt::Display for ColumnBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}
