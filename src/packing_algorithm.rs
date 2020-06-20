use crate::bin::Bin;
use crate::block::Block;
use crate::error::{Error, Result};
use crate::item::Item;

/// While loop to pack items into a bin, using a First Fit Descending approach.
///
/// When you pack an item into a bin, find the best fit, which will change the dimensions available
/// to pack items into. While there are still items to pack and dimensions large enough to hold at
/// least one of the items, it will continue to pack the same bin. If there is no remaining space in
/// the bin large enough for an item, a new dimension will be added to the available blocks. After
/// there are no more items needing to be packed, returns a list of lists of the items in their
/// bins. (first bin is first nested list, second is the second, etc.)
///
/// returns:
///   * [[&str]]: list of lists representing all items in their bins. Length
///     of the outer list is the number of bins we'll use, and length of each
///     subarray is the number of items in each bin. The value of each item in
///     the subarray corresponds to the item's id.
///
/// >>> pack_bins([5,5,10], [[5,5,10], [5,5,6], [5,5,4]]) [ [[5,5,10]],
///     [[5,5,6], [5,5,4]] ]
pub fn packing_algorithm<'a>(bin: Bin, items: &'a Vec<Item<'_>>) -> Result<Vec<Vec<&'a str>>> {
    // remaining_blocks is a list of Block objects, representing the available
    // space into which items can be added.

    if !items.iter().all(|item| bin.does_item_fit(item)) {
        return Err(Error::ItemsNoFit(format!(
            "All items must fit within the bin dimensions."
        )));
    }

    let mut remaining_blocks = Vec::<Block>::new();

    let mut items_to_pack = items.clone();

    // Sort the items in descending order, where order is based on the longest dimension:

    items_to_pack.sort_by(|a, b| b.cmp(&a));

    let mut packed_items: Vec<Vec<&str>> = Vec::<Vec<&str>>::new();

    while !items_to_pack.is_empty() {
        if remaining_blocks.is_empty() {
            remaining_blocks.push(bin.block.clone());
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
                    .find(|(_, item)| block.does_it_fit(&item.block))
                {
                    // Add the id to our packed_items:
                    packed_items
                        .last_mut()
                        .expect("packed_items must not be empty!")
                        .push(item.id);
                    items_to_pack.remove(i);
                    block.best_fit(&item.block).unwrap_or(vec![])
                } else {
                    vec![]
                }
            })
            .collect::<Vec<Block>>();
    }

    Ok(packed_items)
}
