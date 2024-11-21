//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Error) | Library-related error type

use crate::cards::Card;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Error) | Library-related error type
#[derive(Debug, Clone)]
pub enum Error {
    /// Function not implemented
    NotImplemented(String),
    /// Test failure
    TestFailure(String),
    /// Internal
    Internal(String),
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
    /// Badly formed description
    BadDescription(String),
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Result) | Library-related result type
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotImplemented(s)
                => write!(f, "function '{}' not implemented", s),
            Error::TestFailure(s)
                => write!(f, "test failure: {}", s),
            Error::Internal(s)
                => write!(f, "internal error: {}", s),
            Error::IO(s)
                => write!(f, "IO error: {}", s),
            Error::NotRank(r)
                => write!(f, "'{}' is not a card rank", r),
            Error::NotSuit(s)
                => write!(f, "'{}' is not a card suit", s),
            Error::NotCard(s)
                => write!(f, "'{}' is not a card", s),
            Error::ParseEmpty(s)
                => write!(f, "empty input expecting {}", s),
            Error::ParseOther(msg)
                => write!(f, "parse error: {}", msg),
            Error::InvalidCard(c, d)
                => write!(f, "invalid card {} for {} deck", c, d),
            Error::DuplicateCard(c)
                => write!(f, "duplicate card {}", c),
            Error::EmptyDeck(have, need)
                => write!(f, "empty deck: {}/{}", have, need),
            Error::EmptyHand(have, need)
                => write!(f, "empty hand: {}/{}", have, need),
            Error::CardNotFound(c)
                => write!(f, "{} not found", c),
            Error::HashDomain(s)
                => write!(f, "hand {} is not in hash function domain", s),
            Error::BadHand(s)
                => write!(f, "bad hand: {}", s),
            Error::BadDescription(s)
                => write!(f, "bad description: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl From<serde_json5::Error> for Error {
    fn from(e: serde_json5::Error) -> Self {
        Error::ParseOther(e.to_string())
    }
}
