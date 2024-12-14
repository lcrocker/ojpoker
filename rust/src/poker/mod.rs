#![doc = include_str!("../../doc/poker_module.md")]

pub mod hand_scale;
pub use hand_scale::*;

pub mod hand_value;
pub use hand_value::*;

pub mod reference_evaluators;
pub use reference_evaluators::*;

pub mod bug;
pub use bug::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Game) | Poker games
pub mod games;
pub use games::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Tables) | Lookup tables
pub mod tables;
#[cfg(any(feature = "high-hand-tables",
    feature = "ace-to-five-tables",
    feature = "deuce-to-seven-tables",
    feature = "badugi-tables"))]
pub use tables::*;
