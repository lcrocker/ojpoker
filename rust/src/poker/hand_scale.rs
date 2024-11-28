//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Scale) | Poker hand evaluation info

use crate::error::{Error,Result};
use crate::cards::*;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Scale) | Poker hand evaluation info
///
/// Enum representing hand "scales", or ways in which poker hands are
/// evaluated in different games.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Scale {
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
    /// Three-card high hands
    ThreeCard = 13,
}

const SCALE_MAX: usize = Scale::ThreeCard as usize;

impl Scale {
    /// Get a hand scale by index
    pub const fn from_u8(index: u8) -> Scale {
        match index {
            1 => Scale::HighHand,
            2 => Scale::AceToFive,
            3 => Scale::DeuceToSeven,
            4 => Scale::AceToSix,
            5 => Scale::Badugi,
            6 => Scale::Badeucy,
            7 => Scale::PaiGow,
            8 => Scale::Stripped,
            9 => Scale::ActionRazz,
            10 => Scale::HighHandBug,
            11 => Scale::AceToFiveBug,
            12 => Scale::Mexican,
            13 => Scale::ThreeCard,
            _ => Scale::None,
        }
    }

    /// Get hand scale by name
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let scale = Scale::by_name("ace-to-five");
    /// assert_eq!(scale, Scale::AceToFive);
    /// ```
    pub fn by_name(sname: &str) -> Scale {
        scale_by_alias(sname)
    }

    /// Create a new deck for this game
    pub fn new_deck(&self) -> Deck {
        Deck::new(SCALE_INFO_TABLE[*self as usize - 1].deck_type)
    }

    /// Canonical name of the game or hand type
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(Scale::AceToFive.name(), "ace-to-five");
    /// ```
    pub fn name(&self) -> &'static str {
        SCALE_INFO_TABLE[*self as usize - 1].name
    }


    /// Preferred deck for this game
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(DeckType::Low, Scale::AceToFive.deck_type());
    /// ```
    pub const fn deck_type(&self) -> DeckType {
        SCALE_INFO_TABLE[*self as usize - 1].deck_type
    }

    /// Number of cards in a complete hand
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(5, Scale::AceToFive.complete_hand());
    /// ```
    pub const fn complete_hand(&self) -> usize {
        SCALE_INFO_TABLE[*self as usize - 1].complete_hand as usize
    }

    /// Calculations and comparisons expect low aces
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(Scale::AceToFive.low_aces());
    /// assert!(! Scale::DeuceToSeven.low_aces());
    /// ```
    pub const fn low_aces(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_aces
    }

    /// Hand values are calculated for low hands
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(Scale::AceToFive.low_hands());
    /// assert!(Scale::DeuceToSeven.low_hands());
    /// assert!(! Scale::HighHand.low_hands());
    /// ```
    pub const fn low_hands(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_hands
    }

    /// Does the game include straights and flushes?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(Scale::HighHand.straights_and_flushes());
    /// assert!(! Scale::AceToFive.straights_and_flushes());
    /// ```
    pub const fn straights_and_flushes(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].straights_and_flushes
    }

    /// Is wheel a straight for high-ace games?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(Scale::HighHand.high_wheel());
    /// assert!(! Scale::DeuceToSeven.high_wheel());
    /// ```
    pub const fn high_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].high_wheel
    }

    /// Does wheel beat K-high straight?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(! Scale::HighHand.pai_gow_wheel());
    /// assert!(Scale::PaiGow.pai_gow_wheel());
    /// ```
    pub const fn pai_gow_wheel(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].pai_gow_wheel
    }

    /// Is Broadway a straight for low-ace games?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// // The only game that might: but I prefer this rule
    /// assert!(! Scale::AceToSix.low_broadway());
    /// ```
    pub const fn low_broadway(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].low_broadway
    }

    /// Does deck have 8s, 9s, and 10s removed?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert!(Scale::Mexican.spanish_gap());
    /// ```
    pub const fn spanish_gap(&self) -> bool {
        SCALE_INFO_TABLE[*self as usize - 1].spanish_gap
    }

    /// Mapping from generic hand level to numeric value
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(4, Scale::AceToFive.value_from_level(HandLevel::Trips));
    /// ```
    pub fn value_from_level(&self, hl: HandLevel) -> u32 {
        match *self {
            Scale::HighHand | Scale::HighHandBug | Scale::PaiGow => {
                value_from_level_high(hl)
            },
            Scale::AceToFive | Scale::AceToFiveBug => {
                value_from_level_ace_to_five(hl)
            },
            Scale::DeuceToSeven | Scale::AceToSix => {
                value_from_level_deuce_to_seven(hl)
            },
            Scale::Badugi | Scale::Badeucy => {
                value_from_level_badugi(hl)
            },
            Scale::Stripped | Scale::Mexican => {
                value_from_level_stripped(hl)
            },
            Scale::ActionRazz => {
                value_from_level_action_razz(hl)
            },
            Scale::ThreeCard => {
                value_from_level_three_card(hl)
            },
            Scale::None => HAND_LEVEL_WORST as u32,
        }
    }

    /// Mapping from numeric value to generic hand level
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(HandLevel::Trips, Scale::AceToFive.level_from_value(4));
    /// ```
    pub fn level_from_value(&self, v: u32) -> HandLevel {
        match *self {
            Scale::HighHand | Scale::HighHandBug | Scale::PaiGow => {
                level_from_value_high(v)
            },
            Scale::AceToFive | Scale::AceToFiveBug => {
                level_from_value_ace_to_five(v)
            },
            Scale::DeuceToSeven | Scale::AceToSix => {
                level_from_value_deuce_to_seven(v)
            },
            Scale::Badugi | Scale::Badeucy => {
                level_from_value_badugi(v)
            },
            Scale::Stripped | Scale::Mexican => {
                level_from_value_stripped(v)
            },
            Scale::ActionRazz => {
                level_from_value_action_razz(v)
            },
            Scale::ThreeCard => {
                level_from_value_three_card(v)
            },
            Scale::None => HandLevel::None,
        }
    }

    /// Is the given card valid for this game?
    pub fn valid_card(&self, c: Card) -> bool {
        self.deck_type().has(c)
    }

    /// Is this hand valid for this game?
    pub fn valid_hand(&self, h: &Hand) -> bool {
        if h.is_empty() || h.len() > 13 {
            return false;
        }
        if h.deck_type().low_aces() != self.low_aces() {
            return false;
        }
        for i in 0..h.len() {
            if ! self.valid_card(h[i]) {
                return false;
            }
        }
        true
    }

    /// Print hand
    pub fn to_string(&self, d: &HandDescription) -> String {
        let mut s = String::with_capacity(2 * d.hand.len() + 2);

        for i in 0..d.hand.len() {
            if i == (d.length as usize) {
                s.push('(');
            }
            s.push_str(&d.hand[i].to_string());
        }
        if (d.length as usize) < d.hand.len() {
            s.push(')');
        }
        s
    }

    /// Game-specific function to get full English name of hand
    /// ```rust
    /// use onejoker::prelude::*;
    /// use onejoker::poker::{ojp_high_eval_full};
    ///
    /// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
    /// let v = ojp_high_eval_full(&hand).unwrap();
    /// println!("{}", v.full_name());
    /// println!("{}", Scale::HighHand.full_name(&v));
    /// ```
    pub fn full_name(&self, d: &HandDescription) -> String {
        match *self {
            Scale::HighHand | Scale::HighHandBug | Scale::PaiGow |
            Scale::Stripped | Scale::Mexican | Scale::ThreeCard => {
                ojp_high_full_name(d)
            },
            Scale::AceToFive | Scale::AceToFiveBug => {
                ojp_ace_to_five_full_name(d)
            },
            Scale::DeuceToSeven | Scale::AceToSix => {
                ojp_deuce_to_seven_full_name(d)
            },
            Scale::Badugi | Scale::Badeucy => {
                ojp_badugi_full_name(d)
            },
            Scale::ActionRazz => {
                // ojp_action_razz_full_name(d)
                ojp_ace_to_five_full_name(d)
            },
            Scale::None => "".to_string(),
        }
    }

    /// Game-specific full hand evaluation function
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
    /// let v = Scale::HighHand.eval(&hand).unwrap();
    /// println!("{}", v.full_name());
    /// ```
    pub fn eval(&self, hand: &Hand) -> Result<HandDescription> {
        match *self {
            Scale::HighHand => {
                ojp_high_eval_full(hand)
            },
            Scale::AceToFive => {
                ojp_ace_to_five_eval_full(hand)
            },
            Scale::DeuceToSeven => {
                ojp_deuce_to_seven_eval_full(hand)
            },
            Scale::AceToSix => {
                ojp_ace_to_six_eval_full(hand)
            },
            Scale::Badugi => {
                ojp_badugi_eval_full(hand)
            },
            Scale::Badeucy => {
                // ojp_badeucy_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::HighHandBug => {
                // ojp_high_bug_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::AceToFiveBug => {
                // ojp_ace_to_five_bug_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::PaiGow => {
                // ojp_pai_gow_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::Stripped => {
                // ojp_stripped_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::Mexican => {
                // ojp_mexican_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::ActionRazz => {
                // ojp_action_razz_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::ThreeCard => {
                // ojp_three_card_eval_full(hand)
                Err(Error::NotImplemented("".into()))
            },
            Scale::None => Err(Error::Internal("unknown scale".into())),
        }
    }

    /// Game-specific quick value-only hand evaluation function
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
    /// let v = Scale::HighHand.eval_quick(&hand);
    /// println!("{}", v);
    /// ```
    pub fn eval_quick(&self, hand: &Hand) -> HandValue {
        match *self {
            Scale::HighHand => {
                ojp_high_eval_quick(hand)
            },
            Scale::AceToFive => {
                ojp_ace_to_five_eval_quick(hand)
            },
            Scale::DeuceToSeven => {
                ojp_deuce_to_seven_eval_quick(hand)
            },
            Scale::AceToSix => {
                ojp_ace_to_six_eval_quick(hand)
            },
            Scale::Badugi => {
                ojp_badugi_eval_quick(hand)
            },
            Scale::Badeucy => {
                // ojp_badeucy_eval_quick(hand)
                HAND_VALUE_WORST
            },
            Scale::HighHandBug => {
                // ojp_high_bug_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::AceToFiveBug => {
                // ojp_ace_to_five_bug_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::PaiGow => {
                // ojp_pai_gow_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::Stripped => {
                // ojp_stripped_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::Mexican => {
                // ojp_mexican_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::ActionRazz => {
                // ojp_action_razz_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::ThreeCard => {
                // ojp_three_card_eval_full(hand)
                HAND_VALUE_WORST
            },
            Scale::None => HAND_VALUE_WORST,
        }
    }
}

impl std::convert::From<u8> for Scale {
    fn from(n: u8) -> Self {
        Scale::from_u8(n)
    }
}

// Static game info struct
#[derive(Debug, Clone)]
struct ScaleInfo {
    /// Name of the game or hand type
    name: &'static str,
    /// Index of the preferred deck for this game
    deck_type: DeckType,
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
        _ => HAND_LEVEL_WORST as u32,
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
        _ => HAND_LEVEL_WORST as u32,
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
        _ => HAND_LEVEL_WORST as u32,
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
        _ => HAND_LEVEL_WORST as u32,
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
        _ => HAND_LEVEL_WORST as u32,
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
        _ => HAND_LEVEL_WORST as u32,
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

#[inline]
fn value_from_level_three_card(l: HandLevel) -> u32 {
    match l {
        HandLevel::StraightFlush => 1,
        HandLevel::Trips => 2,
        HandLevel::Straight => 3,
        HandLevel::Flush => 4,
        HandLevel::Pair => 5,
        HandLevel::NoPair => 6,
        _ => HAND_LEVEL_WORST as u32,
    }
}

#[inline]
fn level_from_value_three_card(v: u32) -> HandLevel {
    match v {
        1 => HandLevel::StraightFlush,
        2 => HandLevel::Trips,
        3 => HandLevel::Straight,
        4 => HandLevel::Flush,
        5 => HandLevel::Pair,
        6 => HandLevel::NoPair,
        _ => HandLevel::None,
    }
}

/// Static array of info on various games
const SCALE_INFO_TABLE: [ScaleInfo; SCALE_MAX] = [
    ScaleInfo {     // 1
        name: "high-hand",
        deck_type: DeckType::English,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 2
        name: "ace-to-five",
        deck_type: DeckType::Low,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 3
        name: "deuce-to-seven",
        deck_type: DeckType::English,
        complete_hand: 5,
        low_aces: false,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 4
        name: "ace-to-six",
        deck_type: DeckType::Low,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: true,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 5
        name: "badugi",
        deck_type: DeckType::Low,
        complete_hand: 4,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 6
        name: "badeucy",
        deck_type: DeckType::English,
        complete_hand: 4,
        low_aces: false,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 7
        name: "paigow",
        deck_type: DeckType::LowJoker,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: true,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 8
        name: "stripped",
        deck_type: DeckType::English,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 9
        name: "action-razz",
        deck_type: DeckType::Low,
        complete_hand: 6,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 10
        name: "high-hand-bug",
        deck_type: DeckType::OneJoker,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 11
        name: "ace-to-five-bug",
        deck_type: DeckType::LowJoker,
        complete_hand: 5,
        low_aces: true,
        low_hands: true,
        straights_and_flushes: false,
        high_wheel: false,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
    ScaleInfo {     // 12
        name: "mexican",
        deck_type: DeckType::Mexican,
        complete_hand: 5,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: true,
    },
    ScaleInfo {     // 13
        name: "three-card",
        deck_type: DeckType::English,
        complete_hand: 3,
        low_aces: false,
        low_hands: false,
        straights_and_flushes: true,
        high_wheel: true,
        pai_gow_wheel: false,
        low_broadway: false,
        spanish_gap: false,
    },
];

fn scale_by_alias(alias: &str) -> Scale {
    match &alias.to_lowercase()[..] {
        "high" | "high-hand" | "traditional" | "standard" | "poker" | "default"
        => Scale::HighHand,

        "bug" | "high-bug"
        => Scale::HighHandBug,

        "ace-to-five" | "razz" | "low"
        => Scale::AceToFive,

        "ace-to-five-bug" | "low-bug" | "california-lowball"
        => Scale::AceToFiveBug,

        "deuce-to-seven" | "kansas-city-lowball"
        => Scale::DeuceToSeven,

        "ace-to-six" | "london-lowball"
        => Scale::AceToSix,

        "badugi"
        => Scale::Badugi,

        "badeucy"
        => Scale::Badeucy,

        "paigow" | "pai-gow"
        => Scale::PaiGow,

        "stripped" | "manila"
        => Scale::Stripped,

        "mexican"
        => Scale::Mexican,

        "action-razz"
        => Scale::ActionRazz,

        "three-card"
        => Scale::ThreeCard,

        _ => Scale::HighHand,
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_scale_info() -> Result<()> {
        assert_eq!(Scale::default(), Scale::HighHand);
        assert_eq!(Scale::by_name("default"), Scale::HighHand);
        assert_eq!(Scale::by_name("poker"), Scale::HighHand);
        assert_eq!(Scale::from_u8(1), Scale::by_name("high-hand"));

        assert_eq!(Scale::by_name("ace-to-five"), Scale::AceToFive);
        assert_eq!(Scale::by_name("low"), Scale::AceToFive);
        assert_eq!(Scale::from_u8(2), Scale::by_name("razz"));

        assert_eq!(Scale::by_name("deuce-to-seven"), Scale::DeuceToSeven);
        assert_eq!(Scale::by_name("kansas-city-lowball"), Scale::DeuceToSeven);
        assert_eq!(Scale::from_u8(3), Scale::DeuceToSeven);

        assert_eq!(Scale::by_name("ace-to-six"), Scale::AceToSix);
        assert_eq!(Scale::by_name("london-lowball"), Scale::AceToSix);
        assert_eq!(Scale::from_u8(4), Scale::AceToSix);

        assert_eq!(Scale::by_name("badugi"), Scale::Badugi);
        assert_eq!(Scale::from_u8(5), Scale::Badugi);

        assert_eq!(Scale::by_name("badeucy"), Scale::Badeucy);
        assert_eq!(Scale::from_u8(6), Scale::Badeucy);

        assert_eq!(Scale::by_name("paigow"), Scale::PaiGow);
        assert_eq!(Scale::from_u8(7), Scale::PaiGow);

        assert_eq!(Scale::by_name("stripped"), Scale::Stripped);
        assert_eq!(Scale::by_name("manila"), Scale::Stripped);
        assert_eq!(Scale::from_u8(8), Scale::Stripped);

        assert_eq!(Scale::by_name("action-razz"), Scale::ActionRazz);
        assert_eq!(Scale::from_u8(9), Scale::ActionRazz);

        assert_eq!(Scale::by_name("high-bug"), Scale::HighHandBug);
        assert_eq!(Scale::from_u8(10), Scale::HighHandBug);

        assert_eq!(Scale::by_name("ace-to-five-bug"), Scale::AceToFiveBug);
        assert_eq!(Scale::by_name("california-lowball"), Scale::AceToFiveBug);
        assert_eq!(Scale::from_u8(11), Scale::AceToFiveBug);

        assert_eq!(Scale::by_name("mexican"), Scale::Mexican);
        assert_eq!(Scale::from_u8(12), Scale::Mexican);

        assert_eq!(Scale::by_name("three-card"), Scale::ThreeCard);
        assert_eq!(Scale::from_u8(13), Scale::ThreeCard);

        Ok(())
    }
}
