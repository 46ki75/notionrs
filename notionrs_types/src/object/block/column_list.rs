use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct ColumnListBlock {
    /// Only `column` can be specified.
    /// It can only be specified when making a block creation request.
    /// If you need to retrieve the child blocks, you will have to send a request to this block again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<super::Block>>,
}

impl ColumnListBlock {
    /// Only `column` can be specified.
    pub fn children(mut self, children: Vec<super::Block>) -> Self {
        self.children = Some(children);
        self
    }
}

impl std::fmt::Display for ColumnListBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn column_list_block() {
        let cl = ColumnListBlock::default();
        let with_children = cl.children(vec![]);
        assert!(with_children.children.as_ref().unwrap().is_empty());
        assert_eq!(with_children.to_string(), "");

        let json = serde_json::to_string(&ColumnListBlock::default()).unwrap();
        let de: ColumnListBlock = serde_json::from_str(&json).unwrap();
        assert!(de.children.is_none());
    }
}
