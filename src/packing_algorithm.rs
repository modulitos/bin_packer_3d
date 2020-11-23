use crate::bin::Bin;
use crate::error::{Error, Result};
use crate::item::{Item, ItemId};

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
  # use bin_packer_3d::error::Result;
  # fn main() -> Result<()> {

  let deck = Item::new("deck", [2.0, 8.0, 12.0]);
  let die = Item::new("die", [8.0, 8.0, 8.0]);
  let items = vec![deck.clone(), deck.clone(), die, deck.clone(), deck];

  let packed_items = packing_algorithm(Bin::new([8.0, 8.0, 12.0]), &items).unwrap();
  assert_eq!(packed_items, vec![vec!["deck", "deck", "deck", "deck"], vec!["die"]]);
  # Ok(())
  # }
```
**/

pub fn packing_algorithm(
    bin: Bin,
    items: &[Item],
) -> Result<Vec<Vec<ItemId>>> {
    if !items.iter().all(|item| bin.fits(item)) {
        return Err(Error::AllItemsMustFit(format!(
            "All items must fit within the bin dimensions."
        )));
    }

    let mut items_to_pack = items.to_owned();

    // Sort the items in descending order, where order is based on the longest dimension:

    items_to_pack.sort_by(|a, b| b.cmp(&a));

    let mut packed_bins: Vec<Bin> = Vec::new();
    let mut bin_currently_packing = bin.clone_as_empty_bin();

    loop {
        match (
            items_to_pack.is_empty(),
            bin_currently_packing.items.is_empty(),
        ) {
            (true, true) => break,
            (true, false) => {
                // no more items to pack:
                packed_bins.push(bin_currently_packing);
                break;
            }
            (false, _) => {
                if let Some(packed_item_index) = items_to_pack
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find_map(|(item_index, item)| {
                        bin_currently_packing.try_packing(item).map(|_| item_index)
                    })
                {
                    items_to_pack.remove(packed_item_index);
                } else {
                    // We can't fit any more items into the current bin - add it to our packed_bins, and
                    // open up a new bin to pack.

                    let packed_bin =
                        std::mem::replace(&mut bin_currently_packing, bin.clone_as_empty_bin());
                    packed_bins.push(packed_bin);
                }
            }
        }
    }

    // map the bins back into their Vec<ItemId> representations:

    Ok(packed_bins
        .into_iter()
        .map(|bin| bin.items.into_iter().map(|item| item.id).collect())
        .collect())
}
