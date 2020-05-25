use crate::block::FirstFitKind::{DoubledFit, ExactFit, GreaterThanFit};
use crate::error::Result;
use std::cmp::Ordering::Equal;

type Dimension = f32;
type Volume = f32;

enum FirstFitKind {
    // usize contains the index of the dim that matches the first fit
    DoubledFit(usize),
    ExactFit(usize),
    GreaterThanFit(usize),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    dims: [Dimension; 3],
}

impl Block {
    fn new(d1: Dimension, d2: Dimension, d3: Dimension) -> Self {
        let mut dims = [d1, d2, d3];
        dims.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        Self { dims }
    }

    fn volume(&self) -> Volume {
        self.dims.iter().map(|&dim| Volume::from(dim)).product()
    }

    // Returns a boolean regarding whether or not an item will fit into the box.

    fn does_it_fit(&self, other: &Block) -> bool {
        self.dims
            .iter()
            .zip(other.dims.iter())
            .all(|(d, other_d)| d >= other_d)
    }

    // This is a rotation method to rotate the item by first checking if the item
    // MUST be rotated in a specific direction based on size constraints, then
    // rotates it so it leaves the largest bulk volume left in the box

    fn _get_side_2_side_3(&self, item: &Block, side_1: usize) -> (usize, usize) {
        if item.dims[1] > self.dims[(side_1 + 2) % 3] {
            (side_1 - 2, side_1 - 1)
        } else if item.dims[1] > self.dims[(side_1 + 1) % 3] {
            (side_1 - 1, side_1 - 2)
        } else {
            ((side_1 + 1) % 3, (side_1 + 2) % 3)
        }
    }

    // Find the first fit where the longest side of our item fits into the
    // shortest side of our container.

    fn _get_first_fit(&self, item: &Block) -> FirstFitKind {
        let doubled_fit_side = self.dims.iter().enumerate().find_map(|(i, side)| {
            if side >= &(item.dims[2] * 2 as Dimension) {
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

    // Finds the shortest length of the container that will fit the longest
    // length of the item.
    //
    // Uses first fit, then rotates for remaining largest volume block(s)
    //
    // Returns a list of the remaining dimensions in the container, sorted by non-decreasing volume.
    //
    // example:
    //   >>> Box::new(10,10,10).best_fit(Box::new(5,5,5))
    //       [ Box<5,5,5>, Box<5,5,10>, Box<5,10,10> ]

    pub fn best_fit(mut self, item: &Block) -> Option<Vec<Block>> {
        if !self.does_it_fit(&item) {
            return None;
        }

        let mut blocks = vec![];

        let side_1 = match self._get_first_fit(item) {
            DoubledFit(i) => {
                // choose the shortest side of the box we can stack the item twice on its longest
                // side based on theory of if b_dim / 2 >= s_dim, don't open a new bin (or don't
                // rotate the box).

                let block_1 = Block::new(
                    self.dims[i] - item.dims[2],
                    self.dims[(i + 2) % 3],
                    self.dims[(i + 1) % 3],
                );

                // reset the box dimensions to being the height of the item:

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
                //  If we can't do either of the above, then chose the shortest side of the
                //  container where we can stack the longest side of the item:

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
}

#[test]
fn test_box_creation() -> Result<()> {
    Block::new(1 as Dimension, 2 as Dimension, 3 as Dimension);
    Ok(())
}

#[test]
fn test_box_creation_sorts() -> Result<()> {
    let b = Block::new(2 as Dimension, 1 as Dimension, 3 as Dimension);
    assert_eq!(b.dims, [1 as Dimension, 2 as Dimension, 3 as Dimension]);
    Ok(())
}

#[test]
fn test_box_volume() -> Result<()> {
    let b = Block::new(3 as Dimension, 4 as Dimension, 5 as Dimension);
    assert_eq!(b.volume(), 60 as Dimension);
    Ok(())
}

#[test]
fn test_box_volume_large_values() -> Result<()> {
    let b = Block::new(200 as Dimension, 100 as Dimension, 200 as Dimension);
    assert_eq!(b.volume(), 4_000_000 as Dimension);
    Ok(())
}

#[test]
fn test_box_does_it_fit() -> Result<()> {
    // test that when an item fits, it returns true
    let item = Block::new(3.5, 12.7, 14 as Dimension);
    let container = Block::new(4 as Dimension, 22 as Dimension, 14 as Dimension);
    assert!(container.does_it_fit(&item));
    Ok(())
}

#[test]
fn test_box_does_it_fit_false() -> Result<()> {
    // test that when a item does not fit, it returns false
    let item = Block::new(4 as Dimension, 12 as Dimension, 14 as Dimension);
    let container = Block::new(3 as Dimension, 14 as Dimension, 14 as Dimension);
    assert!(!container.does_it_fit(&item));
    Ok(())
}

#[test]
fn test_best_fit_nil() -> Result<()> {
    // assert that if a item does not fit in the container,
    // we get None returned
    let item = Block::new(4 as Dimension, 12 as Dimension, 14 as Dimension);
    let container = Block::new(3 as Dimension, 14 as Dimension, 14 as Dimension);
    assert_eq!(container.best_fit(&item), None);
    Ok(())
}

#[test]
fn test_best_fit_exact_size() -> Result<()> {
    // assert that if a item is the same size as the container, the remaining_dimensions comes back
    // empty

    let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
    let container = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
    assert_eq!(container.best_fit(&item), Some(vec![]));
    Ok(())
}

#[test]
fn test_best_fit_half_size() -> Result<()> {
    // Assert that if a item is smaller than the container, but has two dimensions the same, it will
    // return the empty space

    let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
    let container = Block::new(13 as Dimension, 26 as Dimension, 31 as Dimension);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![Block::new(
            13 as Dimension,
            13 as Dimension,
            31 as Dimension
        )])
    );
    Ok(())
}

#[test]
fn test_best_fit_first_fit_greater_than() -> Result<()> {
    // test that the "greater than" match clause of the first fit returns the
    // correct remaining space.
    let item = Block::new(1.25, 7 as Dimension, 10 as Dimension);
    let container = Block::new(3.5, 9.5, 12.5);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![
            Block::new(1.25, 2.5, 7 as Dimension),
            Block::new(2.5, 3.5, 12.5),
            Block::new(2.25, 7 as Dimension, 12.5)
        ])
    );
    Ok(())
}

#[test]
fn test_best_fit_multiple_spaces_1_2_2() -> Result<()> {
    // test to ensure that our 2x theorum is working
    let item = Block::new(1 as Dimension, 1 as Dimension, 1 as Dimension);
    let container = Block::new(1 as Dimension, 2 as Dimension, 2 as Dimension);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![
            Block::new(1 as Dimension, 1 as Dimension, 1 as Dimension),
            Block::new(1 as Dimension, 1 as Dimension, 2 as Dimension)
        ])
    );
    Ok(())
}

#[test]
fn test_best_fit_multiple_spaces() -> Result<()> {
    // assert that if a item is smaller than the container, but has two dimensions
    // the same, it will return the empty space
    let item = Block::new(13 as Dimension, 13 as Dimension, 31 as Dimension);
    let (x, y, z) = (20 as Dimension, 20 as Dimension, 31 as Dimension);
    let container = Block::new(x, y, z);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![
            Block::new(7 as Dimension, 13 as Dimension, 31 as Dimension),
            Block::new(7 as Dimension, 20 as Dimension, 31 as Dimension)
        ])
    );
    Ok(())
}
