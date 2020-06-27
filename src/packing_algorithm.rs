use crate::bin::Bin;
use crate::error::{Error, Result};
use crate::item::Item;

/**
While loop to pack items into a bin, using a First Fit Descending approach.

When you pack an item into a bin, find the best fit, which will change the dimensions available
to pack items into. While there are still items to pack and dimensions large enough to hold at
least one of the items, it will continue to pack the same bin. If there is no remaining space in
the bin large enough for an item, a new dimension will be added to the available blocks. After
there are no more items needing to be packed, returns a list of lists of the items in their
bins. (first bin is first nested list, second is the second, etc.)

```rust
  use bin_packer_3d::bin::Bin;
  use bin_packer_3d::item::Item;
  use bin_packer_3d::packing_algorithm::packing_algorithm;

  let deck = Item::new("deck", [2.0, 8.0, 12.0]);
  let die = Item::new("die", [8.0, 8.0, 8.0]);
  let items = vec![deck.clone(), deck.clone(), die, deck.clone(), deck];

  let packed_items = packing_algorithm(Bin::new([8.0, 8.0, 12.0]), &items);
  assert_eq!(packed_items, Ok(vec![vec!["deck", "deck", "deck", "deck"], vec!["die"]]));
```
**/

pub fn packing_algorithm<'a>(bin: Bin, items: &'a Vec<Item<'_>>) -> Result<Vec<Vec<&'a str>>> {
    if !items.iter().all(|item| bin.does_item_fit(item)) {
        return Err(Error::ItemsNoFit(format!(
            "All items must fit within the bin dimensions."
        )));
    }

    // remaining_bins is a list of Bins, representing the available space into which items can be
    // added.

    let mut remaining_bins = Vec::<Bin>::new();

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
            .collect::<Vec<Bin>>();
    }

    Ok(packed_items)
}
