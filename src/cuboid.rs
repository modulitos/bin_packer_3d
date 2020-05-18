use crate::cuboid::FirstFitKind::{DoubledFit, ExactFit, GreaterThanFit};
use crate::error::Result;

type Dimension = u8;
type Volume = u32;

enum FirstFitKind {
    // usize contains the index of the dim that matches the first fit
    DoubledFit(usize),
    ExactFit(usize),
    GreaterThanFit(usize),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Box {
    dims: [Dimension; 3],
}

impl Box {
    fn new(d1: Dimension, d2: Dimension, d3: Dimension) -> Self {
        let mut dims = [d1, d2, d3];
        dims.sort();
        Self { dims }
    }

    fn volume(&self) -> Volume {
        self.dims.iter().map(|&dim| Volume::from(dim)).product()
    }

    // Returns a boolean regarding whether or not an item will fit into the box.

    fn does_it_fit(&self, other: &Box) -> bool {
        self.dims
            .iter()
            .zip(other.dims.iter())
            .all(|(d, other_d)| d >= other_d)
    }

    // This is a rotation method to rotate the item by first checking if the item
    // MUST be rotated in a specific direction based on size constraints, then
    // rotates it so it leaves the largest bulk volume left in the box

    fn _get_side_2_side_3(&self, item: &Box, side_1: usize) -> (usize, usize) {
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

    fn _get_first_fit(&self, item: &Box) -> FirstFitKind {
        let doubled_fit_side = self.dims.iter().enumerate().find_map(|(i, side)| {
            if side >= &(item.dims[2] * 2) {
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

    pub fn best_fit(mut self, item: &Box) -> Option<Vec<Box>> {
        if !self.does_it_fit(&item) {
            return None;
        }

        let mut blocks = vec![];

        let side_1 = match self._get_first_fit(item) {
            DoubledFit(i) => {
                // choose the shortest side of the box we can stack the item twice on its longest
                // side based on theory of if b_dim / 2 >= s_dim, don't open a new bin (or don't
                // rotate the box).

                let block_1 = Box::new(
                    self.dims[i] - item.dims[2],
                    self.dims[(i + 2) % 3],
                    self.dims[(i + 1) % 3],
                );

                // reset the box dimensions to being the height of the item:

                self.dims[i] = item.dims[2];

                blocks.push(block_1);
                // (i, Some(block_1))
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

                blocks.push(Box::new(
                    self.dims[i] - item.dims[2],
                    item.dims[0],
                    item.dims[1],
                ));
                i
            }
        };

        let (side_2, side_3) = self._get_side_2_side_3(item, side_1);

        // option one for remaining blocks
        let block_2a = Box::new(
            self.dims[side_1],
            self.dims[side_2],
            self.dims[side_3] - item.dims[0],
        );
        let block_3a = Box::new(
            self.dims[side_1],
            self.dims[side_2] - item.dims[1],
            item.dims[0],
        );

        // option two for remaining blocks
        let block_2b = Box::new(
            self.dims[side_1],
            self.dims[side_2] - item.dims[1],
            self.dims[side_3],
        );
        let block_3b = Box::new(
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
            .filter(|block| block.dims[0] != 0)
            .collect::<Vec<Box>>();
        res.sort_by_key(|block| block.volume());
        Some(res)
    }
}

#[test]
fn test_box_creation() -> Result<()> {
    Box::new(1, 2, 3);
    Ok(())
}

#[test]
fn test_box_creation_sorts() -> Result<()> {
    let b = Box::new(2, 1, 3);
    assert_eq!(b.dims, [1, 2, 3]);
    Ok(())
}

#[test]
fn test_box_volume() -> Result<()> {
    let b = Box::new(3, 4, 5);
    assert_eq!(b.volume(), 60);
    Ok(())
}

#[test]
fn test_box_volume_large_values() -> Result<()> {
    let b = Box::new(200, 100, 200);
    assert_eq!(b.volume(), 4_000_000);
    Ok(())
}

#[test]
fn test_box_does_it_fit() -> Result<()> {
    // test that when an item fits, it returns true
    let item = Box::new(4, 12, 14);
    let container = Box::new(4, 22, 14);
    assert!(container.does_it_fit(&item));
    Ok(())
}

#[test]
fn test_box_does_it_fit_false() -> Result<()> {
    // test that when a item does not fit, it returns false
    let item = Box::new(4, 12, 14);
    let container = Box::new(3, 14, 14);
    assert!(!container.does_it_fit(&item));
    Ok(())
}

#[test]
fn test_best_fit_nil() -> Result<()> {
    // assert that if a item does not fit in the container,
    // we get None returned
    let item = Box::new(4, 12, 14);
    let container = Box::new(3, 14, 14);
    assert_eq!(container.best_fit(&item), None);
    Ok(())
}

#[test]
fn test_best_fit_exact_size() -> Result<()> {
    // assert that if a item is the same size as the container, the remaining_dimensions comes back
    // empty

    let item = Box::new(13, 13, 31);
    let container = Box::new(13, 13, 31);
    assert_eq!(container.best_fit(&item), Some(vec![]));
    Ok(())
}

#[test]
fn test_best_fit_half_size() -> Result<()> {
    // Assert that if a item is smaller than the container, but has two dimensions the same, it will
    // return the empty space

    let item = Box::new(13, 13, 31);
    let container = Box::new(13, 26, 31);
    assert_eq!(container.best_fit(&item), Some(vec![Box::new(13, 13, 31)]));
    Ok(())
}

// TODO: extend our Box work for floats as well.
// #[test]
// fn test_best_fit_first_fit_greater_than() -> Result<()> {
// # test that the "greater than" match clause of the first fit returns the
// # correct remaining space.
// item = Box.new(1.25, 7, 10)
// container = Box.new(3.5, 9.5, 12.5)
//     assert_eq!(container.best_fit(&item), Some(vec![Box::new(13, 13, 31)]));
//     Ok(())
// }

#[test]
fn test_best_fit_multiple_spaces_1_2_2() -> Result<()> {
    // test to ensure that our 2x theorum is working
    let item = Box::new(1, 1, 1);
    let container = Box::new(1, 2, 2);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![Box::new(1, 1, 1), Box::new(1, 1, 2)])
    );
    Ok(())
}

#[test]
fn test_best_fit_multiple_spaces() -> Result<()> {
    // assert that if a item is smaller than the container, but has two dimensions
    // the same, it will return the empty space
    let item = Box::new(13, 13, 31);
    let container = Box::new(20, 20, 31);
    assert_eq!(
        container.best_fit(&item),
        Some(vec![Box::new(7, 13, 31), Box::new(7, 20, 31)])
    );
    Ok(())
}
