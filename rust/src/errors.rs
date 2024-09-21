//! # errors | [wiki](https://github.com/lcrocker/tspoker/wiki/Errors) | Library-related error types.

use crate::cards::*;

#[derive(Debug, Clone)]
pub enum OjError {
    ParseNotRank(String),
    ParseNotSuit(String),
    ParseNotCard(String),
    ParseEmpty(String),
    ParseOther(String),
    InvalidCard(Card, String),
    DuplicateCard(Card),
    EmptyDeck(usize, usize),
    EmptyHand(usize, usize),
    CardNotFound(Card),
    NoAssociatedDeck,
}

impl std::fmt::Display for OjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OjError::ParseNotRank(r)
                => write!(f, "'{}' is not a card rank", r),
            OjError::ParseNotSuit(s)
                => write!(f, "'{}' is not a card suit", s),
            OjError::ParseNotCard(s)
                => write!(f, "'{}' is not a card", s),                
            OjError::ParseEmpty(s)
                => write!(f, "empty input expecting {}", s),
            OjError::ParseOther(msg)
                => write!(f, "parse error: {}", msg),
            OjError::InvalidCard(c, d)
                => write!(f, "invalid card {} for {} deck", c, d),
            OjError::DuplicateCard(c)
                => write!(f, "duplicate card {}", c),
            OjError::EmptyDeck(have, need)
                => write!(f, "empty deck: {}/{}", have, need),
            OjError::EmptyHand(have, need)
                => write!(f, "empty hand: {}/{}", have, need),
            OjError::CardNotFound(c)
                => write!(f, "{} not found", c),
            OjError::NoAssociatedDeck
                => write!(f, "hand has no deck to draw from"),
        }
    }
}
