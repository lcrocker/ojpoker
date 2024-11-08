//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandScale) | Poker game info

use static_assertions::const_assert;
use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

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
    /// High hands, except wheel beats K-high straight
    PaiGow = 6,
    /// Stripped deck: flush beats full house
    Stripped = 7,
    /// Low hands, but face card needed to qualify
    ActionRazz = 8,
}

const HAND_SCALE_COUNT: usize = 9;
const HAND_SCALE_BY_INDEX: [HandScale; HAND_SCALE_COUNT] = [
    HandScale::None,
    HandScale::HighHand,
    HandScale::AceToFive,
    HandScale::DeuceToSeven,
    HandScale::AceToSix,
    HandScale::Badugi,
    HandScale::PaiGow,
    HandScale::Stripped,
    HandScale::ActionRazz,
];

fn scale_by_alias(alias: &str) -> HandScale {
    match &alias.to_lowercase()[..] {
        "high-hand" | "high" | "traditional" | "standard" | "poker"
            | "default" => HandScale::HighHand,

        "ace-to-five" | "razz" | "low" | "california" => HandScale::AceToFive,
        "deuce-to-seven" | "kansas-city" => HandScale::DeuceToSeven,
        "ace-to-six" | "london" => HandScale::AceToSix,
        "badugi" => HandScale::Badugi,
        "paigow" | "pai-gow" => HandScale::PaiGow,
        "stripped" | "manila" | "mexican" => HandScale::Stripped,
        "action-razz" => HandScale::ActionRazz,

        _ => HandScale::HighHand,
    }
}

impl HandScale {
    /// How many?
    pub const fn count() -> usize {
        HAND_SCALE_COUNT
    }

    /// Get a hand scale by index
    pub const fn from_u8(index: u8) -> HandScale {
        HAND_SCALE_BY_INDEX[index as usize]
    }

    /// Get hand scale by name
    pub fn by_name(sname: &str) -> HandScale {
        scale_by_alias(sname)
    }   

    /// Name of the game or hand type
    pub fn name(&self) -> &'static str {
        SCALE_INFO_TABLE[*self as usize - 1].name
    }

    /// Mapping from generic hand level to numeric value
    pub const fn value_from_level(&self, hl: HandLevel) -> u32 {
        SCALE_INFO_TABLE[*self as usize - 1].value_from_level[hl as usize] as u32
    }

    /// Mapping from numeric value to generic hand level
    pub const fn level_from_value(&self, v: usize) -> HandLevel {
        HandLevel::from_u8(
            SCALE_INFO_TABLE[*self as usize - 1].level_from_value[v]
        )
    }

    /// Best possible hand for this game
    pub const fn best(&self) -> HandValue {
        SCALE_INFO_TABLE[*self as usize - 1].best
    }

    /// Worst possible hand for this game
    pub const fn worst(&self) -> HandValue {
        SCALE_INFO_TABLE[*self as usize - 1].worst
    }

    /// Multiplier for hand value calculation
    pub const fn multiplier(&self) -> u32 {
        SCALE_INFO_TABLE[*self as usize - 1].multiplier
    }

    /// Index of the preferred deck for this game
    pub const fn deck_type(&self) -> DeckType {
        DeckType::from_u8(SCALE_INFO_TABLE[*self as usize].deck_type)
    }

    /// Number of cards in a complete hand
    pub const fn complete_hand(&self) -> usize {
        SCALE_INFO_TABLE[*self as usize - 1].complete_hand as usize
    }

    /// Calculations and comparisons expect low aces
    pub const fn low_aces(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_aces
    }

    /// Hand values are calculated with for low hands
    pub const fn low_hands(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_hands
    }

    /// Does the game include straights and flushes?
    pub const fn straights_and_flushes(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].straights_and_flushes
    }

    /// Is wheel a straight for high-ace games?
    pub const fn high_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].high_wheel
    }

    /// Does wheel beat K-high straight?
    pub const fn pai_gow_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].pai_gow_wheel
    }

    /// Is Broadway a straight for low-ace games?
    pub const fn low_broadway(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_broadway
    }

    /// Full English name of hand e.g. "sevens full of fours"
    pub const fn full_name(&self) -> fn(&HandValue) -> String {
        SCALE_INFO_TABLE[*self as usize - 1].full_name
    }

    /// Full hand evaluation function
    pub const fn eval_full(&self) -> fn(&Hand) -> Result<HandValue, OjError> {
        SCALE_INFO_TABLE[*self as usize - 1].eval_full
    }

    /// Quick value-only hand evaluation function
    pub const fn eval_quick(&self) -> fn(&Hand) -> u32 {
        SCALE_INFO_TABLE[*self as usize - 1].eval_quick
    }
}

impl std::convert::From<u8> for HandScale {
    fn from(n: u8) -> Self {
        HandScale::from_u8(n)
    }
}

const LEVELS: usize = 24;
const_assert!(LEVELS >= HandLevel::count());

// Static game info struct
#[derive(Debug, Clone)]
struct HandScaleInfo {
    /// Name of the game or hand type
    name: &'static str,
    /// Mapping from general hand class to numeric level
    value_from_level: [u8; LEVELS],
    /// Mapping from numeric level to general hand class
    level_from_value: [u8; LEVELS],
    /// Best possible hand for this game
    best: HandValue,
    /// Worst possible hand for this game
    worst: HandValue,
    /// Multiplier for hand value calculation
    multiplier: u32,
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

    /// Full English name of hand e.g. "sevens full of fours"
    full_name: fn(&HandValue) -> String,
    /// Full hand evaluation function
    eval_full: fn(&Hand) -> Result<HandValue, OjError>,
    /// Quick value-only hand evaluation function
    eval_quick: fn(&Hand) -> u32,
}

macro_rules! const_hand {
    ( $id:path, $a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal ) => {
        Hand {
            cards: [ Card::from_const_str($a), Card::from_const_str($b),
                Card::from_const_str($c), Card::from_const_str($d),
                Card::from_const_str($e), Card::from_const_str($f),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0)
            ],
            length: 6,
            deck_type: $id as u8,
        }
    };
    ( $id:path, $a:literal, $b:literal, $c:literal, $d:literal, $e:literal ) => {
        Hand {
            cards: [ Card::from_const_str($a),
                Card::from_const_str($b), Card::from_const_str($c),
                Card::from_const_str($d), Card::from_const_str($e),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0)
            ],
            length: 5,
            deck_type: $id as u8,
        }
    };
    ( $id:path, $a:literal, $b:literal, $c:literal, $d:literal ) => {
        Hand {
            cards: [ Card::from_const_str($a),
                Card::from_const_str($b), Card::from_const_str($c),
                Card::from_const_str($d), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0), Card(0),
                Card(0), Card(0), Card(0), Card(0), Card(0)
            ],
            length: 4,
            deck_type: $id as u8,
        }
    };
}

/// Static array of info on various games
const SCALE_INFO_TABLE: [HandScaleInfo; HAND_SCALE_COUNT - 1] = [
    HandScaleInfo {     // 1
        name: "high-hand",
        value_from_level: [
            0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "As", "Ks", "Qs", "Js", "Ts"),
            value: 0,
            scale: HandScale::HighHand as u8,
            level: HandLevel::StraightFlush as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "7c", "5c", "4c", "3c", "2d"),
            value: 0xFFFFFFFF,
            scale: HandScale::HighHand as u8,
            level: HandLevel::NoPair as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_high_full_name,
        eval_full: ojp_high_eval_full,
        eval_quick: ojp_high_eval_quick,
    },
    HandScaleInfo {     // 2
        name: "ace-to-five",
        value_from_level: [
            0, 7, 0, 6, 5, 0, 0, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 10, 9, 8, 7, 4, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "5c", "4c", "3c", "2c", "1c"),
            value: 0,
            scale: HandScale::AceToFive as u8,
            level: HandLevel::NoPair as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "Ks", "Kh", "Kd", "Kc", "Ks"),
            value: 0xFFFFFFFF,
            scale: HandScale::AceToFive as u8,
            level: HandLevel::FiveOfAKind as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::Low as u8,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_ace_to_five_full_name,
        eval_full: ojp_ace_to_five_eval_full,
        eval_quick: ojp_ace_to_five_eval_quick,
    },
    HandScaleInfo {     // 3
        name: "deuce-to-seven",
        value_from_level: [
            0, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "7c", "5c", "4c", "3c", "2d"),
            value: 0,
            scale: HandScale::DeuceToSeven as u8,
            level: HandLevel::NoPair as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "As", "Ks", "Qs", "Js", "Ts"),
            value: 0xFFFFFFFF,
            scale: HandScale::DeuceToSeven as u8,
            level: HandLevel::StraightFlush as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_deuce_to_seven_full_name,
        eval_full: ojp_deuce_to_seven_eval_full,
        eval_quick: ojp_deuce_to_seven_eval_quick,
    },
    HandScaleInfo {     // 4
        name: "ace-to-six",
        value_from_level: [
            0, 0, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 10, 9, 8, 7, 6, 5, 4, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "6c", "4c", "3c", "2c", "1d"),
            value: 0,
            scale: HandScale::AceToFive as u8,
            level: HandLevel::NoPair as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "Ks", "Qs", "Js", "Ts", "9s"),
            value: 0xFFFFFFFF,
            scale: HandScale::AceToFive as u8,
            level: HandLevel::StraightFlush as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::Low as u8,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_ace_to_six_full_name,
        eval_full: ojp_ace_to_six_eval_full,
        eval_quick: ojp_ace_to_six_eval_quick,
    },
    HandScaleInfo {     // 5
        name: "badugi",
        value_from_level: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "4c", "3d", "2h", "1s"),
            value: 0,
            scale: HandScale::Badugi as u8,
            level: HandLevel::FourCard as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "Ks", "Kh", "Kd", "Kc"),
            value: 0xFFFFFFFF,
            scale: HandScale::Badugi as u8,
            level: HandLevel::OneCard as u8,
        },
        multiplier: 100000,
        deck_type: DeckType::Low as u8,
        complete_hand: 4,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_badugi_full_name,
        eval_full: ojp_badugi_eval_full,
        eval_quick: ojp_badugi_eval_quick,
    },
    HandScaleInfo {     // 6
        name: "paigow",
        value_from_level: [
            0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "As", "Ks", "Qs", "Js", "Ts"),
            value: 0,
            scale: HandScale::PaiGow as u8,
            level: HandLevel::StraightFlush as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "7c", "5c", "4c", "3c", "2d"),
            value: 0xFFFFFFFF,
            scale: HandScale::PaiGow as u8,
            level: HandLevel::NoPair as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::LowJoker as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: true,
        low_broadway: false,

        full_name: |_: &HandValue| -> String { String::from("") },
        eval_full: |_: &Hand| -> Result<HandValue, OjError> {
            Err(OjError::NotImplemented(String::from("")))
        },
        eval_quick: |_: &Hand| -> u32 { 0 },
    },
    HandScaleInfo {     // 7
        name: "stripped",
        value_from_level: [
            0, 0, 1, 2, 4, 3, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        level_from_value: [
            0, 2, 3, 5, 4, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "As", "Ks", "Qs", "Js", "Ts"),
            value: 0,
            scale: HandScale::HighHand as u8,
            level: HandLevel::StraightFlush as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "7c", "5c", "4c", "3c", "2d"),
            value: 0xFFFFFFFF,
            scale: HandScale::HighHand as u8,
            level: HandLevel::NoPair as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::English as u8,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_high_full_name,
        eval_full: ojp_high_eval_full,
        eval_quick: ojp_high_eval_quick,
    },
    HandScaleInfo {     // 8
        name: "action-razz",
        value_from_level: [
            0, 0, 0, 6, 5, 0, 0, 4, 3, 2, 1, 0, 0, 0, 0, 12, 11, 10, 9, 8, 7, 0, 0, 0
        ],
        level_from_value: [
            0, 10, 9, 8, 7, 4, 3, 20, 19, 18, 17, 16, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ],
        best: HandValue {
            hand: const_hand!(DeckType::English, "5c", "4c", "3c", "2c", "1c", "Jc"),
            value: 0,
            scale: HandScale::ActionRazz as u8,
            level: HandLevel::NoPair as u8,
        },
        worst: HandValue {
            hand: const_hand!(DeckType::English, "Ts", "Th", "Td", "Tc", "9d", "9c"),
            value: 0xFFFFFFFF,
            scale: HandScale::ActionRazz as u8,
            level: HandLevel::UnqualifiedQuads as u8,
        },
        multiplier: 2000000,
        deck_type: DeckType::Low as u8,
        complete_hand: 6,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,

        full_name: ojp_ace_to_five_full_name,
        eval_full: ojp_ace_to_five_eval_full,
        eval_quick: ojp_ace_to_five_eval_quick,
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
        assert_eq!(HandScale::count(), HAND_SCALE_COUNT);
        assert_eq!(HandScale::default(), HandScale::HighHand);
        assert_eq!(HandScale::by_name("default"), HandScale::HighHand);
        assert_eq!(HandScale::by_name("poker"), HandScale::HighHand);
        assert_eq!(HandScale::from_u8(1), HandScale::by_name("high-hand"));

        assert_eq!(HandScale::by_name("ace-to-five"), HandScale::AceToFive);
        assert_eq!(HandScale::by_name("low"), HandScale::AceToFive);
        assert_eq!(HandScale::by_name("razz"), HandScale::AceToFive);
        assert_eq!(HandScale::from_u8(2), HandScale::by_name("california"));

        assert_eq!(HandScale::by_name("deuce-to-seven"), HandScale::DeuceToSeven);
        assert_eq!(HandScale::by_name("kansas-city"), HandScale::DeuceToSeven);
        assert_eq!(HandScale::from_u8(3), HandScale::DeuceToSeven);

        assert_eq!(HandScale::by_name("ace-to-six"), HandScale::AceToSix);
        assert_eq!(HandScale::by_name("london"), HandScale::AceToSix);
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

        for i in 1..HAND_SCALE_COUNT {
            let info = &SCALE_INFO_TABLE[i - 1];

            let mut bits: u32 = 0;
            for j in 0..LEVELS {
                let l = info.level_from_value[j];
                let v = info.value_from_level[j];
                bits |= 1 << v;

                if 0 != l {
                    assert!(0 != info.value_from_level[l as usize]);
                }
                if 0 != v {
                    assert!(0 != info.level_from_value[v as usize]);
                }
            }
            // consecutive bits
            assert!(0 == (bits & (bits + 1)));
        }
        Ok(())
    }
}
