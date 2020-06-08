use crate::block::{Block, Dimension};
use crate::item::Item;

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Bin {
    pub block: Block,
}

impl Bin {
    pub fn new(dims: [Dimension; 3]) -> Self {
        Self {
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }
    pub fn does_item_fit(&self, item: &Item) -> bool {
        self.block.does_it_fit(&item.block)
    }
}
