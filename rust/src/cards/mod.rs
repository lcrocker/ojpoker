//@ cards/mod.rs

//! # cards | [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Libraries) | Non-game-specific card handling.

pub use crate::errors::*;

pub mod suit;
pub use suit::Suit;

pub mod rank;
pub use rank::Rank;

pub mod card;
pub use card::{ Card, cards_from_text };

pub mod utils;
pub use utils::*;

pub mod hashes;
pub use hashes::*;

pub mod master_deck;
pub use master_deck::MasterDeck;

pub mod stack;
pub use stack::*;

pub mod deck_hand;
pub use deck_hand::*;

pub use card::{ WHITE_JOKER, BLACK_JOKER, JOKER,
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
