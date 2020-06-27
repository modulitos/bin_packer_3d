use crate::block::{Block, Dimension};
use num_traits::{FromPrimitive, Num};
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::Debug;
use std::iter::Product;
use std::ops::Sub;

/// Represents an item that a user will insert into a bin.
#[derive(Clone, Debug)]
pub struct Item<'a, T>
where
    // T: Num + PartialOrd + Product + Sub + Debug,
    T: Num + FromPrimitive + Product + Debug,
{
    /// a string slice of the id
    pub id: &'a str,
    /// a Block
    pub block: Block<T>,
}

impl<'a, T> Item<'a, T>
where
    // T: Copy + Num + PartialOrd + Product + Sub + Debug,
    T: Copy + Num + FromPrimitive + Product + Debug,
{
    /// Create an item given it's id and dimensions.
    pub fn new(id: &'a str, dims: [Dimension<T>; 3]) -> Self {
        Self {
            id,
            block: Block::new(dims[0], dims[1], dims[2]),
        }
    }

    fn get_largest_dim(&self) -> Dimension<T> {
        self.block.dims[2]
    }
}

impl<T> Ord for Item<'_, T>
where
    // T: Num + PartialOrd + Product + Sub + Debug,
    T: Num + FromPrimitive + Product + Debug,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_largest_dim()
            .partial_cmp(&other.get_largest_dim())
            .unwrap_or(Equal)
    }
}

impl<T> PartialOrd for Item<'_, T>
where
    // T: Num + PartialOrd + Product + Sub + Debug,
    T: Num + FromPrimitive + Product + Debug,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<T> PartialEq for Item<'_, T>
where
    // T: Num + PartialOrd + Product + Sub + Debug,
    T: Num + FromPrimitive + Product + Debug,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Item<'_, T> where
    // T: Num + PartialOrd + Product + Sub + Debug
    T: Num + FromPrimitive + Product + Debug
{
}
