use crate::block::BestFitKind::{DoubledFit, ExactFit, GreaterThanFit};
extern crate num_traits;
use self::num_traits::{zero, FromPrimitive, Num, Zero};
use std::cmp::Ordering::Equal;
use std::fmt::Debug;
use std::iter::Product;
use std::ops::{Add, Mul, Sub};

// TODO: explore using a fixed-decimal type. (eg: u16 for the integer, and u8 for the two decmial
// places)

// pub type Dimension = f32;
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Dimension<N: Num>(N);

// impl Debug for Num::FromStrRadixError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
//         f.debug_struct("Dimension")
//             .field("x", &self)
//             .finish()
//     }
// }

impl<T> Sub for Dimension<T>
where
    T: Num,
{
    type Output = Self;

    fn sub(self, other: Dimension<T>) -> Self::Output {
        self - other
    }
}

impl<T> Mul for Dimension<T>
where
    T: Num + Mul,
{
    type Output = Self;

    fn mul(self, other: Dimension<T>) -> Self::Output {
        self * other
    }
}
impl<T> Add for Dimension<T>
where
    T: Num,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        self + other
    }
}

// impl<T> Mul

impl<T> Zero for Dimension<T>
where
    T: Num + Add,
{
    fn zero() -> Self {
        zero()
    }
    fn is_zero(&self) -> bool {
        self == &zero()
    }
}

// type Volume = f32;
// pub struct Dimension<N: Num>(N);

/// Represents the kinds of fits we support in the best-fit section of our algorithm.

enum BestFitKind {
    // usize contains the index of the dim that matches the best fit
    DoubledFit(usize),
    ExactFit(usize),
    GreaterThanFit(usize),
}

trait DimensionTrait: Num + PartialOrd + Product + Sub + Debug {}
impl<T: Num + PartialOrd + Product + Sub + Debug> DimensionTrait for T {}

/// Represents a 3-dimensional cuboid.

#[derive(Debug, PartialEq, Clone)]
pub struct Block<T>
where
    // T: Num + PartialOrd + Product + Sub + Debug + num_traits::Zero + FromPrimitive,
    T: Num +  FromPrimitive + Product + Debug,
    // T: DimensionTrait,
{
    // The dimensions, sorted in ascending order.
    pub dims: [Dimension<T>; 3],
}

impl<T> Block<T>  where  T: Copy + Num +  FromPrimitive + Product + Debug + PartialOrd,
    // T: Copy + Num + PartialOrd + Product + Sub + Debug + Zero + Debug + FromPrimitive,
    // T: DimensionTrait,
{
    // pub fn new(d1: Dimension<T>, d2: Dimension<T>, d3: Dimension<T>) -> Self {
    pub fn new(d1: Dimension<T>, d2: Dimension<T>, d3: Dimension<T>) -> Self {
        let mut dims = [d1, d2, d3];
        dims.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        Self { dims }
    }

    pub fn volume(&self) -> Dimension<T> {
        // self.dims.iter().map(|&dim| Volume::from(dim)).product()
        self.dims.iter().product()
    }

    /// Returns a boolean regarding whether or not an item will fit into the block.

    pub fn does_it_fit(&self, other: &Block<T>) -> bool {
        self.dims
            .iter()
            .zip(other.dims.iter())
            .all(|(d, other_d)| d >= other_d)
    }

    /// Finds the shortest length of the container that will fit the longest length of the item.
    ///
    /// Uses best fit to maximize for the volume of the remaining blocks in the container. The item
    /// and the remaining blocks are rotated to optimize for the largest possible volume in the
    /// remaining blocks.
    ///
    /// Returns a vec of the remaining blocks in the container
    ///
    /// If an item doesn't fit, we return None.
    ///
    /// example:
    ///   >>> Block::new(10,10,10).best_fit(Block::new(5,5,5))
    ///       [ Block<5,5,5>, Block<5,5,10>, Block<5,10,10> ]

    pub fn best_fit(mut self, item: &Block<T>) -> Option<Vec<Block<T>>> {
        if !self.does_it_fit(&item) {
            return None;
        }

        let mut blocks = vec![];

        let side_1 = match self._get_best_fit(item) {
            DoubledFit(i) => {
                // choose the shortest side of the container we can stack the item twice on its
                // longest side based on theory of if b_dim / 2 >= s_dim, don't open a new block (or
                // don't rotate the item).

                let block_1 = Block::new(
                    self.dims[i] - item.dims[2],
                    self.dims[(i + 2) % 3],
                    self.dims[(i + 1) % 3],
                );

                // reset the container's dimensions to being the height of the item:

                self.dims[i] = item.dims[2];

                blocks.push(block_1);
                i
            }
            ExactFit(i) => {
                // If the item's longest side fits perfects across the length of our container, then
                // use that side instead:

                i
            }
            GreaterThanFit(i) => {
                // If we can't do either of the above, then choose the shortest side of the
                // container where we can stack the longest side of the item: i = sides.find {
                // |side| dims[side] >= item.dims[2] }

                blocks.push(Block::new(
                    self.dims[i] - item.dims[2],
                    item.dims[0],
                    item.dims[1],
                ));
                i
            }
        };

        let (side_2, side_3) = self._get_side_2_side_3(item, side_1);

        // option one for remaining blocks
        let block_2a = Block::new(
            self.dims[side_1],
            self.dims[side_2],
            self.dims[side_3] - item.dims[0],
        );
        let block_3a = Block::new(
            self.dims[side_1],
            self.dims[side_2] - item.dims[1],
            item.dims[0],
        );

        // option two for remaining blocks
        let block_2b = Block::new(
            self.dims[side_1],
            self.dims[side_2] - item.dims[1],
            self.dims[side_3],
        );
        let block_3b = Block::new(
            self.dims[side_1],
            self.dims[side_3] - item.dims[0],
            item.dims[1],
        );

        // select the option where block_2 and block_3 are closest in size
        //
        // this operator has been tested and is 5-15% more accurate than if
        // volume(block_2a) > volume(block_2b)

        if block_2a.volume() < block_2b.volume() {
            blocks.push(block_2a);
            blocks.push(block_3a);
        } else {
            blocks.push(block_2b);
            blocks.push(block_3b);
        }

        // if the block's smallest dimension is not 0, then it has volume, so the block should be
        // returned as part of our results

        let mut res = blocks
            .into_iter()
            // .filter(|block| block.dims[0] > 0)
            .filter(|block| block.dims[0] > num_traits::zero())
            // .filter(|block| block.dims[0] > 0 as Dimension)
            .collect::<Vec<Block<T>>>();
        res.sort_by(|block_a, block_b| {
            block_a
                .volume()
                .partial_cmp(&block_b.volume())
                .unwrap_or(Equal)
        });
        Some(res)
    }

    // This is a rotation method to rotate the item first checking if the item MUST be rotated in a
    // specific direction based on size constraints, then rotates it so it leaves the largest bulk
    // volume left in the container.

    fn _get_side_2_side_3(&self, item: &Block<T>, side_1: usize) -> (usize, usize) {
        if item.dims[1] > self.dims[(side_1 + 2) % 3] {
            ((side_1 + 1) % 3, (side_1 + 2) % 3)
        } else if item.dims[1] > self.dims[(side_1 + 1) % 3] {
            ((side_1 + 2) % 3, (side_1 + 1) % 3)
        } else {
            ((side_1 + 1) % 3, (side_1 + 2) % 3)
        }
    }

    // Find the best fit where the longest side of our item fits into the shortest side of our
    // container.

    fn _get_best_fit(&self, item: &Block<T>) -> BestFitKind {
        let doubled_fit_side = self.dims.iter().enumerate().find_map(|(i, side)| {
            if side >= &(item.dims[2] * Dimension(2)) {
                // if side >= &(item.dims[2] * Dimension(
                //     <T as Num>::from_str_radix("2", 10)
                //         .unwrap())) {
                Some(i)
            } else {
                None
            }
        });
        let exact_fit_side = self.dims.iter().enumerate().find_map(|(i, dim)| {
            if dim == &item.dims[2] {
                Some(i)
            } else {
                None
            }
        });

        match (doubled_fit_side, exact_fit_side) {
            (Some(i), None) => DoubledFit(i),
            (None, Some(i)) => ExactFit(i),
            (Some(doubled_i), Some(exact_i)) => {
                if doubled_i <= exact_i {
                    DoubledFit(doubled_i)
                } else {
                    ExactFit(exact_i)
                }
            }
            (None, None) => {
                let i = self
                    .dims
                    .iter()
                    .enumerate()
                    .find_map(|(i, dim)| if dim >= &item.dims[2] { Some(i) } else { None })
                    .expect("Invariant violated: item must fit within the container!");
                GreaterThanFit(i)
            }
        }
    }
}
