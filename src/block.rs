use crate::block::BestFitKind::{DoubledFit, ExactFit, GreaterThanFit};
use std::cmp::Ordering::Equal;

// TODO: explore using a fixed-decimal type. (eg: u16 for the integer, and u8 for the two decmial
// places)

pub type Dimension = f64;
type Volume = f64;

/// Represents the kinds of fits we support in the best-fit section of our algorithm.
/// usize contains the index of the dim where the best-fit has been matched.

enum BestFitKind {
    /// When the side of the container is more than twice the length of the item's matching side.
    DoubledFit(usize),

    /// When the item's side fits perfects across the length of our container.
    ExactFit(usize),

    /// When the side of the container is longer than the length of the item's matching side.
    GreaterThanFit(usize),
}

/// Represents a 3-dimensional cuboid.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Block {
    /// The dimensions, sorted in ascending order.
    pub dims: [Dimension; 3],
}

impl Block {
    pub fn new<F: Into<Dimension>>(d1: F, d2: F, d3: F) -> Self {
        if 100 > i32::MAX {
            println!("ok!");
        }
        // TODO: fail on negative values
        let mut dims = [d1.into(), d2.into(), d3.into()];
        dims.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        Self { dims }
    }

    pub fn volume(&self) -> Volume {
        self.dims.iter().product()
    }

    /// Returns a boolean regarding whether or not an item will fit into the block.

    pub fn does_it_fit(&self, other: &Block) -> bool {
        self.dims
            .iter()
            .zip(other.dims.iter())
            .all(|(d, other_d)| d >= other_d)
    }

    /**
    Finds the shortest length of the container that will fit the longest length of the item.

    Uses best fit to maximize for the volume of the remaining blocks in the container. The item
    and the remaining blocks are rotated to optimize for the largest possible volume in the
    remaining blocks.

    Returns a vec of the remaining blocks in the container

    If an item doesn't fit, we return None.

    /// ```rust
    ///   let item = Block::new(1, 1, 1);
    ///   let container = Block::new(1, 2, 2);
    ///   assert_eq!(
    ///       container.best_fit(&item),
    ///       Some(vec![
    ///           Block::new(1, 1, 1),
    ///           Block::new(1, 1, 2)
    ///       ])
    ///   );
    /// ```
    **/
    pub fn best_fit(mut self, item: &Block) -> Option<Vec<Block>> {
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
            .filter(|block| block.dims[0] > 0 as Dimension)
            .collect::<Vec<Block>>();
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

    fn _get_side_2_side_3(&self, item: &Block, side_1: usize) -> (usize, usize) {
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

    fn _get_best_fit(&self, item: &Block) -> BestFitKind {
        let doubled_fit_side = self.dims.iter().enumerate().find_map(|(i, side)| {
            if side >= &(item.dims[2] * 2_f64) {
                Some(i)
            } else {
                None
            }
        });
        let exact_fit_side = self.dims.iter().enumerate().find_map(|(i, dim)| {

            // consider comparing these within some error: `(dim - &item.dims[2]).abs() < error`

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
