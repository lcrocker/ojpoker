#![warn(missing_docs)]
#![allow(clippy::manual_range_contains)]
#![doc = include_str!("../doc/onejoker_crate.md")]

pub mod prelude;
pub mod error;
pub mod utils;

#[macro_use]
pub mod cards;
pub mod poker;
