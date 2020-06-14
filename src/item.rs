use crate::block::{Block, Dimension};

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Item<'a> {
    /// a string slice of the id
    pub id: &'a str,
    /// a Block
    pub block: Block,
}

impl<'a> Item<'a> {
    /// Create an item given it's id and dimensions.
    pub fn new(id: &'a str, dims: [Dimension; 3]) -> Self {
        Self {
            id,
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }
}
