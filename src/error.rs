use std::error;
use std::result;

#[derive(Debug, PartialEq)]
pub enum Error {
    ItemsNoFit(String),
}

// pub type Result<T, E = Error> = result::Result<T, E>;
pub type Result<T, E = Error> = result::Result<T, E>;
