use std::result;

#[derive(Debug, PartialEq)]
/// There are the errors which can be raised when using the bin packing algorithm.
pub enum Error {

    /// It is an invariant that each item must be able to fit within the bin's dimensions. If one or
    /// more items don't fit into the bin, then this error will be raised.
    ///
    AllItemsMustFit(String),
}

/// This Result type is a convenience type that uses the BinPacker's Error type as a default.
pub type Result<T, E = Error> = result::Result<T, E>;
