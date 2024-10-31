#![warn(missing_docs)]
#![allow(clippy::manual_range_contains)]
#![doc = include_str!("../doc/onejoker_crate.md")]

pub mod errors;
pub use errors::OjError;

pub mod utils;
pub use utils::{
    oj_rand_next32, oj_rand_range, oj_shuffle, oj_sort,
    oj_next_combination, oj_binomial,
};

pub mod cards;
pub use cards::suit::Suit;
pub use cards::rank::Rank;
pub use cards::card::{
    Ordinal, Card,
    WHITE_JOKER, BLACK_JOKER, JOKER,
    LOW_ACE_OF_CLUBS, LOW_ACE_OF_DIAMONDS, LOW_ACE_OF_HEARTS, LOW_ACE_OF_SPADES,
    DEUCE_OF_CLUBS, DEUCE_OF_DIAMONDS, DEUCE_OF_HEARTS, DEUCE_OF_SPADES,
    TREY_OF_CLUBS, TREY_OF_DIAMONDS, TREY_OF_HEARTS, TREY_OF_SPADES,
    FOUR_OF_CLUBS, FOUR_OF_DIAMONDS, FOUR_OF_HEARTS, FOUR_OF_SPADES,
    FIVE_OF_CLUBS, FIVE_OF_DIAMONDS, FIVE_OF_HEARTS, FIVE_OF_SPADES,
    SIX_OF_CLUBS, SIX_OF_DIAMONDS, SIX_OF_HEARTS, SIX_OF_SPADES,
    SEVEN_OF_CLUBS, SEVEN_OF_DIAMONDS, SEVEN_OF_HEARTS, SEVEN_OF_SPADES,
    EIGHT_OF_CLUBS, EIGHT_OF_DIAMONDS, EIGHT_OF_HEARTS, EIGHT_OF_SPADES,
    NINE_OF_CLUBS, NINE_OF_DIAMONDS, NINE_OF_HEARTS, NINE_OF_SPADES,
    TEN_OF_CLUBS, TEN_OF_DIAMONDS, TEN_OF_HEARTS, TEN_OF_SPADES,
    JACK_OF_CLUBS, JACK_OF_DIAMONDS, JACK_OF_HEARTS, JACK_OF_SPADES,
    KNIGHT_OF_CLUBS, KNIGHT_OF_DIAMONDS, KNIGHT_OF_HEARTS, KNIGHT_OF_SPADES,
    QUEEN_OF_CLUBS, QUEEN_OF_DIAMONDS, QUEEN_OF_HEARTS, QUEEN_OF_SPADES,
    KING_OF_CLUBS, KING_OF_DIAMONDS, KING_OF_HEARTS, KING_OF_SPADES,
    ACE_OF_CLUBS, ACE_OF_DIAMONDS, ACE_OF_HEARTS, ACE_OF_SPADES,
};
pub use cards::card_parse::parse_cards;

pub use cards::hashes::{
    ojh_fnv_32, ojh_fnv_64, ojh_positional_32c, ojh_positional_32cs,
    ojh_positional_32cr, ojh_positional_64c, ojh_positional_64cs,
    ojh_positional_64cr, ojh_bitfield_64co, ojh_prime_32cor,
    ojh_prime_64co, ojh_prime_64cor, ojh_mp5_english,
};

pub use cards::hand::Hand;
pub use cards::deck::Deck;

pub mod poker;
pub use poker::hand_value::{
    HandLevelTrait,
    HandLevelHigh, HandLevelPaiGow, HandLevelStripped, HandLevelManilla,
    HandLevelMexican, HandLevelAceToFive, HandLevelDeuceToSeven,
    HandLevelAceToSix, HandLevelBadugi, HandLevelActionRazz,
    HandValueTrait, HandValue, HandEvaluatorTrait,
};
pub use poker::high_hand::{ HandValueHigh, HandEvaluatorHigh };
pub use poker::ace_to_five::{ HandValueAceToFive, HandEvaluatorAceToFive };
pub use poker::deuce_to_seven::{ HandValueDeuceToSeven, HandEvaluatorDeuceToSeven };
pub use poker::ace_to_six::{ HandValueAceToSix, HandEvaluatorAceToSix };
pub use poker::badugi::{ HandValueBadugi, HandEvaluatorBadugi };
