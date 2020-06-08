use crate::block::{Block, Dimension};
use crate::item::Item;

// TODO: update docs:
/// While loop to pack items into a bin, using a First Fit Descending approach.
///
/// When you pack an item into a bin, find the best fit, which will change the dimensions available
/// to pack items into. While there are still items to pack and dimensions large enough to hold at
/// least one of the items, it will continue to pack the same bin. If there is no remaining space in
/// the bin large enough for an item, a new dimension will be added to the available blocks. After
/// there are no more items needing to be packed, returns a list of lists of the items in their
/// bins. (first bin is first nested list, second is the second, etc.)
///
/// Dims = [ int|float, int|float, int|float] - this is a list of 3 ints or floats, representing a
/// cuboid block for either our bins or items.
///
/// returns:
///   * [[String]]: list of lists representing all items in their bins. Length
///     of the outer list is the number of bins we'll use, and length of each
///     subarray is the number of items in each bin. The value of each item in
///     the subarray corresponds to the item's id.
///
/// >>> pack_bins([5,5,10], [[5,5,10], [5,5,6], [5,5,4]]) [ [[5,5,10]],
///     [[5,5,6], [5,5,4]] ]
pub fn packing_algorithm(bin_dimensions: [Dimension; 3], items: &Vec<Item>) -> Vec<Vec<String>> {
    // remaining_blocks is a list of Block objects, representing the available
    // space into which items can be added.

    let mut remaining_blocks = Vec::<Block>::new();

    let mut items_to_pack = items.clone();

    let mut packed_items: Vec<Vec<String>> = Vec::<Vec<String>>::new();

    while !items_to_pack.is_empty() {
        if remaining_blocks.is_empty() {
            remaining_blocks.push(Block::new(
                bin_dimensions[0],
                bin_dimensions[1],
                bin_dimensions[2],
            ));
            packed_items.push(vec![]);
        }
        remaining_blocks = remaining_blocks
            .into_iter()
            .flat_map(|block| {

                // iterator using find then manual map:

                if let Some((i, item)) = items_to_pack
                    .iter()
                    // TODO: is there a better way to structure this logic, to avoid having to clone
                    // here?
                    .cloned()
                    .enumerate()
                    .find(|(i, item)| block.does_it_fit(&item.block))
                {
                    // Add the id to our packed_items:
                    packed_items
                        .last_mut()
                        .expect("packed_items must not be empty!")
                        .push(String::from(&item.id));
                    items_to_pack.remove(i);
                    block.best_fit(&item.block).unwrap_or(vec![])
                } else {
                    vec![]
                }
            })
            .collect::<Vec<Block>>();
    }

    packed_items
}
