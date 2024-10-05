use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ColumnListBlock {
    /// Only `column` can be specified.
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl ColumnListBlock {
    pub fn new() -> Self {
        ColumnListBlock {
            children: Some(vec![]),
        }
    }

    /// Only `column` can be specified.
    pub fn children(mut self, children: Vec<super::Block>) -> Self {
        self.children = Some(children);
        self
    }
}
