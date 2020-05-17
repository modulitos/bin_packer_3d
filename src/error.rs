use std::error;
use std::result;

pub type Error = Box<dyn error::Error>;

// pub type Result<T, E = Error> = result::Result<T, E>;
pub type Result<T, E = Error> = result::Result<T, E>;
