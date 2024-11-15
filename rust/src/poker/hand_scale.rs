//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandScale) | Poker hand evaluation info

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandScale) | Poker hand evaluation info
///
/// Enum representing hand "scales", or ways in which poker hands are
/// evaluated in different games.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HandScale {
    /// None / Invalid
    None = 0,
    /// Traditional "high" poker hands
    #[default]
    HighHand = 1,
    /// Low hands, aces low, no straights or flushes
    AceToFive = 2,
    /// Low hands, aces low ("Kansas City" low)
    DeuceToSeven = 3,
    /// Low hands, aces high ("London" low)
    AceToSix = 4,
    /// Four cards, aces low, no matching suits
    Badugi = 5,
    /// Badugi, aces high
    Badeucy = 6,
    /// High hands, except wheel beats K-high straight
    PaiGow = 7,
    /// Stripped deck: flush beats full house
    Stripped = 8,
    /// Ace-to-five low, face card needed to qualify
    ActionRazz = 9,
    /// High hands with single bug
    HighHandBug = 10,
    /// Ace-to-five low with single bug
    AceToFiveBug = 11,
    /// Spanish deck with single bug
    Mexican = 12,
}

const HANDSCALE_MAX: usize = HandScale::Mexican as usize;

fn scale_by_alias(alias: &str) -> HandScale {
    match &alias.to_lowercase()[..] {
        "high-hand" | "high" | "traditional" | "standard" | "poker"
            | "default" => HandScale::HighHand,

        "high-bug" | "bug" => HandScale::HighHandBug,
        "ace-to-five" | "razz" | "low" => HandScale::AceToFive,
        "ace-to-five-bug" | "low-bug" | "california-lowball" => HandScale::AceToFiveBug,
        "deuce-to-seven" | "kansas-city-lowball" => HandScale::DeuceToSeven,
        "ace-to-six" | "london-lowball" => HandScale::AceToSix,
        "badugi" => HandScale::Badugi,
        "badeucy" => HandScale::Badeucy,
        "paigow" | "pai-gow" => HandScale::PaiGow,
        "stripped" | "manila" => HandScale::Stripped,
        "mexican" => HandScale::Mexican,
        "action-razz" => HandScale::ActionRazz,

        _ => HandScale::HighHand,
    }
}

impl HandScale {
    /// Get a hand scale by index
    pub const fn from_u8(index: u8) -> HandScale {
        match index {
            1 => HandScale::HighHand,
            2 => HandScale::AceToFive,
            3 => HandScale::DeuceToSeven,
            4 => HandScale::AceToSix,
            5 => HandScale::Badugi,
            6 => HandScale::PaiGow,
            7 => HandScale::Stripped,
            8 => HandScale::ActionRazz,
            _ => HandScale::None,
        }
    }

    /// Get hand scale by name
    /// ```rust
    /// use onejoker::*;
    ///
    /// let scale = HandScale::by_name("ace-to-five");
    /// assert_eq!(scale, HandScale::AceToFive);
    /// ```
    pub fn by_name(sname: &str) -> HandScale {
        scale_by_alias(sname)
    }

    /// Canonical name of the game or hand type
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(HandScale::AceToFive.name(), "ace-to-five");
    /// ```
    pub fn name(&self) -> &'static str {
        SCALE_INFO_TABLE[*self as usize - 1].name
    }

    /// Mapping from generic hand level to numeric value
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(4, HandScale::AceToFive.value_from_level(HandLevel::Trips));
    /// ```
    pub fn value_from_level(&self, hl: HandLevel) -> u32 {
        (SCALE_INFO_TABLE[*self as usize - 1].value_from_level)(hl)
    }

    /// Mapping from numeric value to generic hand level
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(HandLevel::Trips, HandScale::AceToFive.level_from_value(4));
    /// ```
    pub fn level_from_value(&self, v: u32) -> HandLevel {
        (SCALE_INFO_TABLE[*self as usize - 1].level_from_value)(v)
    }

    /// Index of the preferred deck for this game
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(DeckType::Low, HandScale::AceToFive.deck_type());
    /// ```
    pub const fn deck_type(&self) -> DeckType {
        DeckType::from_u8(SCALE_INFO_TABLE[*self as usize - 1].deck_type)
    }

    /// Number of cards in a complete hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(5, HandScale::AceToFive.complete_hand());
    /// ```
    pub const fn complete_hand(&self) -> usize {
        SCALE_INFO_TABLE[*self as usize - 1].complete_hand as usize
    }

    /// Calculations and comparisons expect low aces
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(HandScale::AceToFive.low_aces());
    /// assert!(! HandScale::DeuceToSeven.low_aces());
    /// ```
    pub const fn low_aces(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_aces
    }

    /// Hand values are calculated for low hands
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(HandScale::AceToFive.low_hands());
    /// assert!(HandScale::DeuceToSeven.low_hands());
    /// assert!(! HandScale::HighHand.low_hands());
    /// ```
    pub const fn low_hands(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_hands
    }

    /// Does the game include straights and flushes?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(HandScale::HighHand.straights_and_flushes());
    /// assert!(! HandScale::AceToFive.straights_and_flushes());
    /// ```
    pub const fn straights_and_flushes(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].straights_and_flushes
    }

    /// Is wheel a straight for high-ace games?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(HandScale::HighHand.high_wheel());
    /// assert!(! HandScale::DeuceToSeven.high_wheel());
    /// ```
    pub const fn high_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].high_wheel
    }

    /// Does wheel beat K-high straight?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(! HandScale::HighHand.pai_gow_wheel());
    /// assert!(HandScale::PaiGow.pai_gow_wheel());
    /// ```
    pub const fn pai_gow_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].pai_gow_wheel
    }

    /// Is Broadway a straight for low-ace games?
    /// ```rust
    /// use onejoker::*;
    ///
    /// // The only game that might: but I prefer this rule
    /// assert!(! HandScale::AceToSix.low_broadway());
    /// ```
    pub const fn low_broadway(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_broadway
    }

    /// Does deck have 8s, 9s, and 10s removed?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(HandScale::Mexican.spanish_gap());
    /// ```
    pub const fn spanish_gap(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].spanish_gap
    }

    /// Game-specific function to get full English name of hand
    ///
    /// Users will generally call these through the hand value object
    /// rather than directly. If you do call this directly, note that
    /// the function here returns a function, so you then need to call
    /// *that* function to get the actual name.
    /// ```rust
    /// use onejoker::*;
    ///
    /// let hand = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
    /// let v = ojp_high_eval_full(&hand).unwrap();
    /// println!("{}", v.full_name());
    /// println!("{}", HandScale::HighHand.full_name()(&v));
    /// ```
    pub const fn full_name(&self) -> fn(&HandValue) -> String {
        SCALE_INFO_TABLE[*self as usize - 1].full_name
    }

    /// Game-specific full hand evaluation function
    ///
    /// These functions are also not generally called directly by users,
    /// but are used by the public ojp_xxx_eval_full functions.
    pub const fn eval_full(&self) -> fn(&Hand) -> Result<HandValue, OjError> {
        SCALE_INFO_TABLE[*self as usize - 1].eval_full
    }

    /// Game-specific quick value-only hand evaluation function
    ///
    /// These functions are also not generally called directly by users,
    /// but are used by the public ojp_xxx_eval_full functions.
    pub const fn eval_quick(&self) -> fn(&Hand) -> u32 {
        SCALE_INFO_TABLE[*self as usize - 1].eval_quick
    }
}

impl std::convert::From<u8> for HandScale {
    fn from(n: u8) -> Self {
        HandScale::from_u8(n)
    }
}

// Static game info struct
#[derive(Debug, Clone)]
struct HandScaleInfo {
    /// Name of the game or hand type
    name: &'static str,
    /// Index of the preferred deck for this game
    deck_type: u8,
    /// Number of cards in a complete hand
    complete_hand: u8,
    /// Calculations and comparisons expect low aces
    low_aces: bool,
    /// Hand values are calculated with for low hands
    low_hands: bool,
    /// Does the game include straights and flushes?
    straights_and_flushes: bool,
    /// Is wheel a straight for high-ace games?
    high_wheel: bool,
    /// Does wheel beat K-high straight?
    pai_gow_wheel: bool,
    /// Is Broadway a straight for low-ace games?
    low_broadway: bool,
    /// 8s, 9s, and 10s removed
    spanish_gap: bool,
    /// Mapping from general hand class to numeric level
    value_from_level: fn(HandLevel) -> u32,
    /// Mapping from numeric level to general hand class
    level_from_value: fn(u32) -> HandLevel,
    /// Full English name of hand e.g. "sevens full of fours"
    full_name: fn(&HandValue) -> String,
    /// Full hand evaluation function
    eval_full: fn(&Hand) -> Result<HandValue, OjError>,
    /// Quick value-only hand evaluation function
    eval_quick: fn(&Hand) -> u32,
}

#[inline]
fn value_from_level_high(l: HandLevel) -> u32 {
    match l {
        HandLevel::FiveOfAKind => 1,
        HandLevel::StraightFlush => 2,
        HandLevel::Quads => 3,
        HandLevel::FullHouse => 4,
        HandLevel::Flush => 5,
        HandLevel::Straight => 6,
        HandLevel::Trips => 7,
        HandLevel::TwoPair => 8,
        HandLevel::Pair => 9,
        HandLevel::NoPair => 10,
        _ => 11,
    }
}

#[inline]
fn level_from_value_high(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::FiveOfAKind,
        2 => HandLevel::StraightFlush,
        3 => HandLevel::Quads,
        4 => HandLevel::FullHouse,
        5 => HandLevel::Flush,
        6 => HandLevel::Straight,
        7 => HandLevel::Trips,
        8 => HandLevel::TwoPair,
        9 => HandLevel::Pair,
        10 => HandLevel::NoPair,
        _ => HandLevel::None,
    }
}

#[inline]
fn value_from_level_ace_to_five(l: HandLevel) -> u32 {
    match l {
        HandLevel::NoPair => 1,
        HandLevel::Pair => 2,
        HandLevel::TwoPair => 3,
        HandLevel::Trips => 4,
        HandLevel::FullHouse => 5,
        HandLevel::Quads => 6,
        HandLevel::FiveOfAKind => 7,
        _ => 8,
    }
}

#[inline]
fn level_from_value_ace_to_five(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::NoPair,
        2 => HandLevel::Pair,
        3 => HandLevel::TwoPair,
        4 => HandLevel::Trips,
        5 => HandLevel::FullHouse,
        6 => HandLevel::Quads,
        7 => HandLevel::FiveOfAKind,
        _ => HandLevel::None,
    }
}

#[inline]
fn value_from_level_deuce_to_seven(l: HandLevel) -> u32 {
    match l {
        HandLevel::NoPair => 1,
        HandLevel::Pair => 2,
        HandLevel::TwoPair => 3,
        HandLevel::Trips => 4,
        HandLevel::Straight => 5,
        HandLevel::Flush => 6,
        HandLevel::FullHouse => 7,
        HandLevel::Quads => 8,
        HandLevel::StraightFlush => 9,
        HandLevel::FiveOfAKind => 10,
        _ => 11,
    }
}

#[inline]
fn level_from_value_deuce_to_seven(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::NoPair,
        2 => HandLevel::Pair,
        3 => HandLevel::TwoPair,
        4 => HandLevel::Trips,
        5 => HandLevel::Straight,
        6 => HandLevel::Flush,
        7 => HandLevel::FullHouse,
        8 => HandLevel::Quads,
        9 => HandLevel::StraightFlush,
        10 => HandLevel::FiveOfAKind,
        _ => HandLevel::None,
    }
}

#[inline]
fn value_from_level_badugi(l: HandLevel) -> u32 {
    match l {
        HandLevel::FourCard => 1,
        HandLevel::ThreeCard => 2,
        HandLevel::TwoCard => 3,
        HandLevel::OneCard => 4,
        _ => 5,
    }
}

#[inline]
fn level_from_value_badugi(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::FourCard,
        2 => HandLevel::ThreeCard,
        3 => HandLevel::TwoCard,
        4 => HandLevel::OneCard,
        _ => HandLevel::None,
    }
}

#[inline]
fn value_from_level_stripped(l: HandLevel) -> u32 {
    match l {
        HandLevel::FiveOfAKind => 1,
        HandLevel::StraightFlush => 2,
        HandLevel::Quads => 3,
        HandLevel::Flush => 4,
        HandLevel::FullHouse => 5,
        HandLevel::Straight => 6,
        HandLevel::Trips => 7,
        HandLevel::TwoPair => 8,
        HandLevel::Pair => 9,
        HandLevel::NoPair => 10,
        _ => 11,
    }
}

#[inline]
fn level_from_value_stripped(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::FiveOfAKind,
        2 => HandLevel::StraightFlush,
        3 => HandLevel::Quads,
        4 => HandLevel::Flush,
        5 => HandLevel::FullHouse,
        6 => HandLevel::Straight,
        7 => HandLevel::Trips,
        8 => HandLevel::TwoPair,
        9 => HandLevel::Pair,
        10 => HandLevel::NoPair,
        _ => HandLevel::None,
    }
}

#[inline]
fn value_from_level_action_razz(l: HandLevel) -> u32 {
    match l {
        HandLevel::NoPair => 1,
        HandLevel::Pair => 2,
        HandLevel::TwoPair => 3,
        HandLevel::Trips => 4,
        HandLevel::FullHouse => 5,
        HandLevel::Quads => 6,
        HandLevel::FiveOfAKind => 7,
        HandLevel::UnqualifiedNoPair => 8,
        HandLevel::UnqualifiedPair => 9,
        HandLevel::UnqualifiedTwoPair => 10,
        HandLevel::UnqualifiedTrips => 11,
        HandLevel::UnqualifiedFullHouse => 12,
        HandLevel::UnqualifiedQuads => 13,
        HandLevel::UnqualifiedFiveOfAKind => 14,
        _ => 15,
    }
}

#[inline]
fn level_from_value_action_razz(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::NoPair,
        2 => HandLevel::Pair,
        3 => HandLevel::TwoPair,
        4 => HandLevel::Trips,
        5 => HandLevel::FullHouse,
        6 => HandLevel::Quads,
        7 => HandLevel::FiveOfAKind,
        8 => HandLevel::UnqualifiedNoPair,
        9 => HandLevel::UnqualifiedPair,
        10 => HandLevel::UnqualifiedTwoPair,
        11 => HandLevel::UnqualifiedTrips,
        12 => HandLevel::UnqualifiedFullHouse,
        13 => HandLevel::UnqualifiedQuads,
        14 => HandLevel::UnqualifiedFiveOfAKind,
        _ => HandLevel::None,
    }
}

/// Static array of info on various games
const SCALE_INFO_TABLE: [HandScaleInfo; HANDSCALE_MAX] = [
    HandScaleInfo {     // 1
        name: "high-hand",
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_high,
        level_from_value: level_from_value_high,
        full_name: ojp_high_full_name,
        eval_full: ojp_high_eval_full,
        eval_quick: ojp_high_eval_quick,
    },
    HandScaleInfo {     // 2
        name: "ace-to-five",
        deck_type: DeckType::Low as u8,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_ace_to_five,
        level_from_value: level_from_value_ace_to_five,
        full_name: ojp_ace_to_five_full_name,
        eval_full: ojp_ace_to_five_eval_full,
        eval_quick: ojp_ace_to_five_eval_quick,
    },
    HandScaleInfo {     // 3
        name: "deuce-to-seven",
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_deuce_to_seven,
        level_from_value: level_from_value_deuce_to_seven,
        full_name: ojp_deuce_to_seven_full_name,
        eval_full: ojp_deuce_to_seven_eval_full,
        eval_quick: ojp_deuce_to_seven_eval_quick,
    },
    HandScaleInfo {     // 4
        name: "ace-to-six",
        deck_type: DeckType::Low as u8,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_deuce_to_seven,
        level_from_value: level_from_value_deuce_to_seven,
        full_name: ojp_ace_to_six_full_name,
        eval_full: ojp_ace_to_six_eval_full,
        eval_quick: ojp_ace_to_six_eval_quick,
    },
    HandScaleInfo {     // 5
        name: "badugi",
        deck_type: DeckType::Low as u8,
        complete_hand: 4,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_badugi,
        level_from_value: level_from_value_badugi,
        full_name: ojp_badugi_full_name,
        eval_full: ojp_badugi_eval_full,
        eval_quick: ojp_badugi_eval_quick,
    },
    HandScaleInfo {     // 6
        name: "badeucy",
        deck_type: DeckType::English as u8,
        complete_hand: 4,
        low_aces: false,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_badugi,
        level_from_value: level_from_value_badugi,
        full_name: ojp_badugi_full_name,
        eval_full: ojp_badeucy_eval_full,
        eval_quick: ojp_badeucy_eval_quick,
    },
    HandScaleInfo {     // 7
        name: "paigow",
        deck_type: DeckType::LowJoker as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: true,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_high,
        level_from_value: level_from_value_high,
        full_name: ojp_pai_gow_full_name,
        eval_full: ojp_pai_gow_eval_full,
        eval_quick: ojp_pai_gow_eval_quick,
    },
    HandScaleInfo {     // 8
        name: "stripped",
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_stripped,
        level_from_value: level_from_value_stripped,
        full_name: ojp_high_full_name,
        eval_full: ojp_stripped_eval_full,
        eval_quick: ojp_stripped_eval_quick,
    },
    HandScaleInfo {     // 9
        name: "action-razz",
        deck_type: DeckType::Low as u8,
        complete_hand: 6,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_action_razz,
        level_from_value: level_from_value_action_razz,
        full_name: ojp_action_razz_full_name,
        eval_full: ojp_action_razz_eval_full,
        eval_quick: ojp_action_razz_eval_quick,
    },
    HandScaleInfo {     // 10
        name: "high-hand-bug",
        deck_type: DeckType::OneJoker as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_high,
        level_from_value: level_from_value_high,
        full_name: ojp_high_full_name,
        eval_full: ojp_high_bug_eval_full,
        eval_quick: ojp_high_bug_eval_quick,
    },
    HandScaleInfo {     // 11
        name: "ace-to-five-bug",
        deck_type: DeckType::LowJoker as u8,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,

        value_from_level: value_from_level_ace_to_five,
        level_from_value: level_from_value_ace_to_five,
        full_name: ojp_ace_to_five_full_name,
        eval_full: ojp_ace_to_five_bug_eval_full,
        eval_quick: ojp_ace_to_five_bug_eval_quick,
    },
    HandScaleInfo {     // 12
        name: "mexican",
        deck_type: DeckType::Mexican as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: true,

        value_from_level: value_from_level_ace_to_five,
        level_from_value: level_from_value_ace_to_five,
        full_name: ojp_ace_to_five_full_name,
        eval_full: ojp_ace_to_five_bug_eval_full,
        eval_quick: ojp_ace_to_five_bug_eval_quick,
    },
];

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_scale_info() -> Result<(), OjError> {
        assert_eq!(HandScale::default(), HandScale::HighHand);
        assert_eq!(HandScale::by_name("default"), HandScale::HighHand);
        assert_eq!(HandScale::by_name("poker"), HandScale::HighHand);
        assert_eq!(HandScale::from_u8(1), HandScale::by_name("high-hand"));

        assert_eq!(HandScale::by_name("ace-to-five"), HandScale::AceToFive);
        assert_eq!(HandScale::by_name("low"), HandScale::AceToFive);
        assert_eq!(HandScale::from_u8(2), HandScale::by_name("razz"));

        assert_eq!(HandScale::by_name("deuce-to-seven"), HandScale::DeuceToSeven);
        assert_eq!(HandScale::by_name("kansas-city-lowball"), HandScale::DeuceToSeven);
        assert_eq!(HandScale::from_u8(3), HandScale::DeuceToSeven);

        assert_eq!(HandScale::by_name("ace-to-six"), HandScale::AceToSix);
        assert_eq!(HandScale::by_name("london-lowball"), HandScale::AceToSix);
        assert_eq!(HandScale::from_u8(4), HandScale::AceToSix);

        assert_eq!(HandScale::by_name("badugi"), HandScale::Badugi);
        assert_eq!(HandScale::from_u8(5), HandScale::Badugi);

        assert_eq!(HandScale::by_name("paigow"), HandScale::PaiGow);
        assert_eq!(HandScale::from_u8(6), HandScale::PaiGow);

        assert_eq!(HandScale::by_name("stripped"), HandScale::Stripped);
        assert_eq!(HandScale::by_name("manila"), HandScale::Stripped);
        assert_eq!(HandScale::from_u8(7), HandScale::Stripped);

        assert_eq!(HandScale::by_name("action-razz"), HandScale::ActionRazz);
        assert_eq!(HandScale::from_u8(8), HandScale::ActionRazz);

        Ok(())
    }
}
