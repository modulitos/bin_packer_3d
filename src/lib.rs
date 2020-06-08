#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// #![warn(rust_2018_idioms)]

/*!

This module is used as a three dimensional fitting algorithm for packing items into bins. It solves
a constrained version of the 3D bin packing problem.

This algorithm takes a greedy approach, or more specifically, [First Fit Decreasing](
https://en.wikipedia.org/wiki/Bin_packing_problem#First_Fit_Decreasing_(FFD))

It leverages an optimization when packing items which are less than half the length of a bin's side,
as proposed in the paper titled "The Three-Dimensional Bin Packing Problem" (Martello, 1997):
[https://www.jstor.org/stable/pdf/223143.pdf](https://www.jstor.org/stable/pdf/223143.pdf), page 257

# Usage

<TODO>

# Example

<TODO>

*/


mod block;

/// A struct representing the items we'll be packing into the bin.
pub mod item;

/// A struct representing the bin where we'll be packing the items.
pub mod bin;

/// Our packing algorithm.
pub mod packing_algorithm;
pub mod error;

#[cfg(test)]
mod tests;

