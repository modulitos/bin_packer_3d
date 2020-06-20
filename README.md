# bin_packing_3d
<!-- "short sentence explaining what it is"] -->

This crate solves the problem of "fitting smaller boxes inside of a larger box" using a three
dimensional fitting algorithm.

<!-- [more detailed explanation] -->

The algorithm leverages a [First Fit
Decreasing](https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD)) greedy
strategy, which some rotational optimizations.

<!-- [at least one code example that users can copy/paste to try it] -->

# Usage:

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

<!-- # /// [more detailed explanation] -->

# Limitations:

This algorithm solves a constrained version of the 3D bin packing problem. As such, we have the
following limitations:

 * The items we are packing, and the bins that we are packing them into, are limited to cuboid shapes

 * As an NP-Hard problem, this algorithm does not attempt to find the optimal solution

# Acknowledgements:

The algorithm leverages a rotational optimization when packing items which are less than half the
length of a bin's side, as proposed in the paper titled "The Three-Dimensional Bin Packing Problem"
(Martello, 1997):
[https://www.jstor.org/stable/pdf/223143.pdf](https://www.jstor.org/stable/pdf/223143.pdf), page 257
