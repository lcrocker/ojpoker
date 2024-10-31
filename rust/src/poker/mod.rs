#![doc = include_str!("../../doc/poker_module.md")]

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Value) | Poker hand values base class
pub mod hand_value;
pub use hand_value::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Evaluator_State) | State machine for evaluating poker hands
pub mod eval_state;
pub use eval_state::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/High_Hand) | Traditional "high" poker hands
pub mod high_hand;
pub use high_hand::{HandValueHigh, HandEvaluatorHigh};

pub mod ace_to_five;
pub use ace_to_five::{HandValueAceToFive, HandEvaluatorAceToFive};

pub mod deuce_to_seven;
pub use deuce_to_seven::{HandValueDeuceToSeven, HandEvaluatorDeuceToSeven};

pub mod ace_to_six;
pub use ace_to_six::{HandValueAceToSix, HandEvaluatorAceToSix};

pub mod badugi;
pub use badugi::{HandValueBadugi, HandEvaluatorBadugi};
