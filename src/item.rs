use crate::block::{Block, Dimension};

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub block: Block,
}

impl Item {
    pub fn new(id: String, dims: [Dimension; 3]) -> Self {
        Self {
            id,
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }
}
