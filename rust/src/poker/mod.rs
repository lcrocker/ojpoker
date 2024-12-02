#![doc = include_str!("../../doc/poker_module.md")]

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Scale) | Poker hand types
pub mod hand_scale;
pub use hand_scale::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Value) | Evaluated hand data
pub mod hand_value;
pub use hand_value::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Reference_Evaluators) | Hand evaluation code
pub mod reference_evaluators;
pub use reference_evaluators::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Bug) | Bug scan and replacement
pub mod bug;
pub use bug::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Game) | Poker games
pub mod games;
pub use games::*;
