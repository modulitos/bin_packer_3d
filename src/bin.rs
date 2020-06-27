use crate::block::{Block, Dimension};
use crate::item::Item;
use num_traits::{Num, FromPrimitive};
use std::fmt::Debug;
use std::iter::Product;
use std::ops::Sub;

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Bin<T>
where
    // T: Num + PartialOrd + Product + Sub + Debug,
    T: Num +  FromPrimitive + Product + Debug,
{
    /// Represents the cuboid of this Bin.
    block: Block<T>,
}

impl<T> Bin<T>
where
    // T: Copy + Num + PartialOrd + Product + Sub + Debug,
    T: Copy + Num +  FromPrimitive + Product + Debug,
{
    /// Creates a new Bin from it's dimensions.
    pub fn new(dims: [Dimension<T>; 3]) -> Self {
        Self {
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }

    /// Returns whether or not the Bin's dimensions can emcompass or match the Bin.
    pub fn does_item_fit(&self, item: &Item<'_, T>) -> bool {
        self.block.does_it_fit(&item.block)
    }

    /// Returns the remaining bins after the item has been added to the current bin.
    /// Returns None if the item is too big to fit into the bin.
    pub fn best_fit(self, item: &Item<'_, T>) -> Option<Vec<Bin<T>>> {
        self.block
            .best_fit(&item.block)
            .map(|blocks| blocks.into_iter().map(|block| Bin::from(block)).collect())
    }
}

impl<T> From<Block<T>> for Bin<T>
where
    // T: Copy + Num + PartialOrd + Product + Sub + Debug,
    T: Copy + Num +  FromPrimitive + Product + Debug,
{
    fn from(block: Block<T>) -> Self {
        Bin::new([block.dims[0], block.dims[1], block.dims[2]])
    }
}
