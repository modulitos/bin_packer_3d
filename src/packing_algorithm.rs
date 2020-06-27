use crate::bin::Bin;
use crate::error::{Error, Result};
use crate::item::Item;
use num_traits::{Num, FromPrimitive};
use std::iter::Product;
use std::ops::Sub;
use std::fmt::Debug;

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
pub fn packing_algorithm<'a, T: Num + PartialOrd + FromPrimitive + Product + Debug >(
    bin: Bin<T>,
    items: &'a Vec<Item<'_, T>>,
) -> Result<Vec<Vec<&'a str>>> {
    if !items.iter().all(|item| bin.does_item_fit(item)) {
        return Err(Error::ItemsNoFit(format!(
            "All items must fit within the bin dimensions."
        )));
    }

    // remaining_bins is a list of Bins, representing the available space into which items can be
    // added.

    let mut remaining_bins = Vec::<Bin<T>>::new();

    let mut items_to_pack = items.clone();

    // Sort the items in descending order, where order is based on the longest dimension:

    items_to_pack.sort_by(|a, b| b.cmp(&a));

    let mut packed_items: Vec<Vec<&str>> = Vec::<Vec<&str>>::new();

    while !items_to_pack.is_empty() {
        if remaining_bins.is_empty() {
            remaining_bins.push(bin.clone());
            packed_items.push(vec![]);
        }
        remaining_bins = remaining_bins
            .into_iter()
            .flat_map(|bin| {
                items_to_pack
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find_map(|(i, item)| {
                        let remaining_bins = bin.clone().best_fit(&item)?;
                        // Add the id to our packed_items:
                        packed_items
                            .last_mut()
                            .expect("packed_items must not be empty!")
                            .push(item.id);
                        items_to_pack.remove(i);
                        Some(remaining_bins)
                    })
                    // If no items can be fitted into the bin, then skip the bin:
                    .unwrap_or(vec![])
            })
            .collect::<Vec<Bin<T>>>();
    }

    Ok(packed_items)
}
