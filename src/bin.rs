use crate::block::{Block, Dimension};
use crate::item::Item;

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Bin {

    /// Represents the cuboid of this Bin.
    pub block: Block,
}

impl Bin {
    /// Creates a new Bin from it's dimensions.
    pub fn new(dims: [Dimension; 3]) -> Self {
        Self {
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }
    /// Returns whether or not the Bin's dimensions can emcompass or match the Bin.
    pub fn does_item_fit(&self, item: & Item<'_>) -> bool {
        self.block.does_it_fit(&item.block)
    }
}
