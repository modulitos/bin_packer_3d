use crate::error::Result;

type Dimension = u8;

pub struct Box {
    dims: [Dimension; 3],
}

impl Box {
    pub fn new(d1: Dimension, d2: Dimension, d3: Dimension) -> Self {
        let mut dims = [d1, d2, d3];
        dims.sort();
        Self { dims }
    }

    pub fn volume(&self) -> Dimension {
        self.dims.iter().product()
    }

    // Returns a boolean regarding whether or not an item will fit into the box.

    pub fn does_it_fit(&self, other: &Box) -> bool {
        self.dims
            .iter()
            .zip(other.dims.iter())
            .all(|(d, other_d)| d >= other_d)
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
