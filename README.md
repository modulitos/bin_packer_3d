[<img alt="build status" src="https://img.shields.io/github/workflow/status/modulitos/bin_packer_3d/CI/master?style=for-the-badge" height="20">](https://github.com/modulitos/bin_packer_3d/actions?query=branch%3Amaster)

# bin_packer_3d

<!-- "short sentence explaining what it is"] -->

This crate solves the problem of "fitting smaller boxes inside of a larger box" using a three
dimensional fitting algorithm.

<!-- [more detailed explanation] -->

The algorithm orthogonally packs the all the items into a minimum number of bins by leveraging a [First Fit
Decreasing](https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD)) greedy
strategy, along with rotational optimizations.

<!-- [at least one code example that users can copy/paste to try it] -->

# Usage:

```rust
    use bin_packer_3d::bin::Bin;
    use bin_packer_3d::item::Item;
    use bin_packer_3d::packing_algorithm::packing_algorithm;

    let deck = Item::new("deck", [2, 8, 12]);
    let die = Item::new("die", [8, 8, 8]);
    let items = vec![deck, deck, die, deck, deck];

    let packed_items = packing_algorithm(Bin::new([8, 8, 12]), &items);
    assert_eq!(packed_items, Ok(vec![vec!["deck", "deck", "deck", "deck"], vec!["die"]]));
```

<!-- # /// [more detailed explanation] -->

# Limitations:

This algorithm solves a constrained version of the 3D bin packing problem. As such, we have the
following limitations:

 * The items we are packing, and the bins that we are packing them into, are limited to cuboid
   shapes.

 * The items we are packing can be rotated in any direction, with the limitation that each edge must
   be parallel to the corresponding bin edge.

 * As an NP-Hard problem, this algorithm does not attempt to find the optimal solution, but instead
   uses an approximation that runs with a time complexity of *O(n^2)*

# Acknowledgements:

The algorithm leverages a rotational optimization when packing items which are less than half the
length of a bin's side, as proposed in the paper titled "The Three-Dimensional Bin Packing Problem"
(Martello, 1997), page 257:
[https://www.jstor.org/stable/pdf/223143.pdf](https://www.jstor.org/stable/pdf/223143.pdf)

Inspired by this implementation by Shotput:
https://github.com/shotput/BoxPackingAPI/commit/48cfbd9c7b82c6f7640386523627d7911ff9089b
https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4
https://medium.com/the-chain/efficiency-of-the-shotput-packing-algorithm-a690e914d49c
