//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Errors) | Library-related error types.

use crate::cards::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Rust_Errors) | Library-related error types.
#[derive(Debug, Clone)]
pub enum OjError {
    /// Function not implemented
    NotImplemented(String),
    /// Test failure
    TestFailure(String),
    /// IO error
    IO(String),
    /// Number or text is not a valid card rank.
    NotRank(String),
    /// Number or text is not a valid card suit.
    NotSuit(String),
    /// Number or text is not a valid card ordinal.
    NotCard(String),
    /// Empty input where a value was expected.
    ParseEmpty(String),
    /// Some other parsing error.
    ParseOther(String),
    /// Card is not valid for the deck type.
    InvalidCard(Card, String),
    /// Duplicate card in deck or hand not allowing them.
    DuplicateCard(Card),
    /// Deck is empty when it should not be.
    EmptyDeck(usize, usize),
    /// Hand is empty when it should not be.
    EmptyHand(usize, usize),
    /// Card not found in deck or hand.
    CardNotFound(Card),
    /// Hash function doesn't apply to this hand.
    HashDomain(String),
    /// Badly formed poker hand
    BadHand(String),
}

impl std::fmt::Display for OjError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OjError::NotImplemented(s)  
                => write!(f, "function '{}' not implemented", s),
            OjError::TestFailure(s)
                => write!(f, "test failure: {}", s),
            OjError::IO(s)
                => write!(f, "IO error: {}", s),
            OjError::NotRank(r)
                => write!(f, "'{}' is not a card rank", r),
            OjError::NotSuit(s)
                => write!(f, "'{}' is not a card suit", s),
            OjError::NotCard(s)
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
            OjError::HashDomain(s)
                => write!(f, "hand {} is not in hash function domain", s),
            OjError::BadHand(s)
                => write!(f, "bad hand: {}", s),
        }
    }
}

impl std::error::Error for OjError {}

impl From<std::io::Error> for OjError {
    fn from(e: std::io::Error) -> Self {
        OjError::IO(e.to_string())
    }
}

impl From<json5::Error> for OjError {
    fn from(e: json5::Error) -> Self {
        OjError::ParseOther(e.to_string())
    }
}
