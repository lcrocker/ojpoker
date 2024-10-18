#![doc = include_str!("../../doc/poker_module.md")]

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Value) | Poker hand values base class
pub mod hand_value;
pub use hand_value::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/High_Hand) | Traditional "high" poker hands
pub mod high_hand;
pub use high_hand::{HandValueHigh, HandEvaluatorHigh};
