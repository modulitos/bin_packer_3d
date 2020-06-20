use crate::block::{Block, Dimension};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

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

    fn get_largest_dim(&self) -> Dimension {
        self.block.dims[2]
    }
}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_largest_dim()
            .partial_cmp(&other.get_largest_dim())
            .unwrap_or(Equal)
    }
}

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for Item<'a> {}
