use crate::block::{Block, Dimension};
use crate::item::Item;

/// Represents an bin that a user can insert items into.
/// ```rust
/// use bin_packer_3d::bin::Bin;
/// let bin = Bin::new([1.0, 2.0, 3.0]);
/// ```

#[derive(Clone, Debug)]
pub struct Bin<'a> {
    /// Represents the cuboid of this bin.
    blocks: Vec<Block>,
    /// Represents the items that are currently packed inside this bin.
    pub items: Vec<Item<'a>>,
}

impl<'a> Bin<'a> {
    /// Creates a new Bin from it's dimensions.
    pub fn new<F: Into<Dimension> + Copy>(dims: [F; 3]) -> Self {
        Self {
            blocks: vec![Block::new(dims[0], dims[1], dims[2])],
            items: vec![],
        }
    }

    /**
    Returns whether or not the Bin's dimensions can emcompass or match the item.

    An item that fits into a bin:
    ```rust
        use bin_packer_3d::bin::Bin;
        use bin_packer_3d::item::Item;
        let item = Item::new("item1", [1.0, 2.0, 3.0]);
        let bin = Bin::new([1.0, 2.0, 3.0]);
        assert!(bin.fits(&item));
    ```
    An item that does not fit into a bin:
    ```rust
        use bin_packer_3d::bin::Bin;
        use bin_packer_3d::item::Item;
        let item = Item::new("item2", [4.0, 12.0, 14.0]);
        let bin = Bin::new([3.0, 12.0, 14.0]);
        assert!(!bin.fits(&item));
    ```
    **/
    pub fn fits(&self, item: &Item<'_>) -> bool {
        self.blocks
            .iter()
            .any(|block| block.does_it_fit(&item.block))
    }

    /**
     Add the item to the bin
     Returns None if the item cannot be packed into the bin.

    ```rust
        use bin_packer_3d::bin::Bin;
        use bin_packer_3d::item::{Item, ItemId};

        let item_1 = Item::new("item1", [24.0, 10.0, 2.0]);
        let item_2 = Item::new("item2", [24.0, 10.0, 2.0]);
        let item_3 = Item::new("item3", [24.0, 10.0, 2.0]);
        let mut bin = Bin::new([24.0, 10.0, 4.0]);
        assert!(bin.try_packing(item_1).is_some());
        assert!(bin.try_packing(item_2).is_some());
        assert_eq!(bin.try_packing(item_3), None);
        assert_eq!(
            bin.items
                .into_iter()
                .map(|item| item.id)
                .collect::<Vec<&ItemId>>(),
            vec!["item1", "item2"]
        );
    ```
    **/
    pub fn try_packing(&mut self, item: Item<'a>) -> Option<()> {
        let block_to_pack_index =
            self.blocks
                .iter()
                .enumerate()
                .find_map(|(block_index, block)| {
                    if block.does_it_fit(&item.block) {
                        Some(block_index)
                    } else {
                        None
                    }
                })?;
        let block_to_pack = self.blocks.remove(block_to_pack_index);
        self.blocks.append(
            &mut block_to_pack
                .best_fit(&item.block)
                .expect("Invalid state - the block doesn't fit the item."),
        );
        self.items.push(item);
        Some(())
    }
    /**

    Returns a new bin that is the same dimensions as the original bin, but without any items.

    ```rust
        use bin_packer_3d::bin::Bin;
        use bin_packer_3d::item::{Item, ItemId};
        let bin = Bin::new([24.0, 10.0, 4.0]);
        let new_bin = bin.clone_as_empty_bin();
    ```
    **/
    pub fn clone_as_empty_bin(&self) -> Self {
        Self {
            blocks: self.blocks.clone(),
            items: vec![],
        }
    }
}
