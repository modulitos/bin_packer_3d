use crate::block::{Block, Dimension};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;

/// The item's id, represented as a string slice.
///
/// Although it's not enforced, it's highly recommended that each item has a unique ItemId.
///
pub type ItemId = String;

/// Represents an item that a user will insert into a bin.
/// ```rust
///   use bin_packer_3d::item::Item;
///   let item = Item::new("deck", [2.0, 8.0, 12.0]);
/// ```
#[derive(Clone, Debug)]
pub struct Item {
    /// a string slice of the id
    pub id: ItemId,
    /// a Block
    pub block: Block,
}

impl Item {
    /// Create an item given it's id and dimensions.
    pub fn new<F: Into<Dimension> + Copy>(id: &str, dims: [F; 3]) -> Self {
        Self {
            id: id.into(),
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }

    fn get_largest_dim(&self) -> Dimension {
        self.block.dims[2]
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_largest_dim()
            .partial_cmp(&other.get_largest_dim())
            .unwrap_or(Equal)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}
