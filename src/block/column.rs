use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ColumnBlock {
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::BlockType>>,
}

impl ColumnBlock {
    pub fn build(self) -> super::BlockType {
        super::BlockType::Column { column: self }
    }

    pub fn new() -> Self {
        ColumnBlock {
            children: Some(vec![]),
        }
    }

    pub fn children(mut self, children: Vec<super::BlockType>) -> Self {
        self.children = Some(children);
        self
    }
}
