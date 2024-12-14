//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Value) | Evaluated hand data

use crate::error::{Error,Result};
use crate::cards::*;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Level) | Class for categories of poker hands
///
/// Poker hands are ranked by first grouping them into categories (which
/// here we call "level"), and then comparing the ranks of the cards
/// within that level to break ties. We use the actual numbers here to
/// index into tables for calculating comparator values.
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum HandLevel {
    #[default]
    None = 0,
    FiveOfAKind = 1,
    StraightFlush = 2,
    Quads = 3,
    FullHouse = 4,
    Flush = 5,
    Straight = 6,
    Trips = 7,
    TwoPair = 8,
    Pair = 9,
    NoPair = 10,
    FourCard = 11, // Badugi
    ThreeCard = 12,
    TwoCard = 13,
    OneCard = 14,
    UnqualifiedFiveOfAKind = 15, // Action Razz
    UnqualifiedQuads = 16,
    UnqualifiedFullHouse = 17,
    UnqualifiedTrips = 18,
    UnqualifiedTwoPair = 19,
    UnqualifiedPair = 20,
    UnqualifiedNoPair = 21,
}

impl std::fmt::Display for HandLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HandLevel {
    /// Convert integer to level
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(HandLevel::FullHouse, HandLevel::from_u8(4));
    /// ```
    pub const fn from_u8(v: u8) -> Self {
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
            11 => HandLevel::FourCard,
            12 => HandLevel::ThreeCard,
            13 => HandLevel::TwoCard,
            14 => HandLevel::OneCard,
            15 => HandLevel::UnqualifiedFiveOfAKind,
            16 => HandLevel::UnqualifiedQuads,
            17 => HandLevel::UnqualifiedFullHouse,
            18 => HandLevel::UnqualifiedTrips,
            19 => HandLevel::UnqualifiedTwoPair,
            20 => HandLevel::UnqualifiedPair,
            21 => HandLevel::UnqualifiedNoPair,
            _ => HandLevel::None,
        }
    }
}

/// Sentinel value for hand level
pub const HAND_LEVEL_BEST: u8 = 0;
/// Sentinel value for hand level
pub const HAND_LEVEL_WORST: u8 = 0xFF;

/*
 * HandValue
 */

/// The plain integer value used to compare hands--low value wins.
pub type HandValue = u32;

/// For all hand value comparisons, lower is better.
pub const HAND_VALUE_BEST: u32 = 0;
/// Maximum u32
pub const HAND_VALUE_WORST: u32 = 0xFFFF_FFFF;

/*
 * HandDescription
 */

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Extras) | Extra information about a hand
#[derive(Debug, Default, Clone, Copy, Hash)]
#[repr(u8)]
pub enum HandExtras {
    /// No extra information
    #[default]
    None = 0,
    /// Cards that each bug represents
    Bugs([Card;2]),
    /// Cards that each wildcard represents
    Wilds([Card;4]),
    /// Bug replacement, up or down?
    Mexican(Card,bool),
}

impl std::fmt::Display for HandExtras {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HandExtras::None => write!(f, "None"),
            HandExtras::Bugs(b) => write!(f, "Bugs: {} {}", b[0], b[1]),
            HandExtras::Wilds(w) => write!(f, "Wilds: {} {} {} {}", w[0], w[1], w[2], w[3]),
            HandExtras::Mexican(c,b) => write!(f, "Mexican: {} {}", c, b),
        }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Description) | Info about an evaluated hand
///
/// Describes all the information about a hand after evaluation.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HandDescription {
    /// Copy of the hand under evaluation. Modified during evaluation.
    pub hand: Hand,
    /// Comparison value: lower is better
    pub value: HandValue,
    /// Which game type we are evaluating for
    pub scale: Scale,
    /// Level of the hand
    pub level: HandLevel,
    /// "Meaningful" length
    pub length: u8,
    /// Extra game-specific information about the hand
    pub extras: HandExtras,
}

impl HandDescription {
    /// Create a new HandDescription object from HandValue
    pub fn from_value(hand: &Hand, scale: Scale, value: HandValue) -> Self {
        let mut h = *hand;
        let l = get_ranks_from_value(&mut h, scale, value);

        HandDescription {
            hand: h, scale, value, level: l,
            length: scale.complete_hand() as u8,
            extras: HandExtras::None,
        }
    }

    /// Full english description, e.g. "aces full of kings"
    pub fn full_text(&self) -> String {
        self.scale.full_text(self)
    }

    /// Print relevant part of hand
    pub fn hand_to_string(&self) -> String {
        let mut h = self.hand;
        h.truncate(self.length as usize);
        h.to_string()
    }

    /// Add bug replacement values
    pub fn add_bugs(&mut self, bugs: [Card; 2]) {
        self.extras = HandExtras::Bugs(bugs);
    }
}

impl std::fmt::Display for HandDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [{}] = {}", self.scale.name(), self.hand, self.value)
    }
}

impl Default for HandDescription {
    fn default() -> Self {
        HandDescription {
            hand: Hand::new(DeckType::English),
            scale: Scale::HighHand,
            value: HAND_VALUE_WORST,
            level: HandLevel::None,
            length: 0,
            extras: HandExtras::None,
        }
    }
}

// Re-order the given hand according to the ranks decoded from the value
fn get_ranks_from_value(h: &mut Hand, g: Scale, value: HandValue)
-> HandLevel {
    assert!(h.len() > 1 && h.len() <= 9);

    let v = value;
    let mut r: Vec<Rank> = Vec::new();
    let lvl = g.level_from_value(0xFF & (v >> 20));

    let digits =
    if g == Scale::Badugi || g == Scale::Badeucy {
        5 - g.value_from_level(lvl) as usize
    } else if h.len() < g.complete_hand() {
        h.len()
    } else {
        g.complete_hand()
    };

    for i in (5-digits)..5 {
        let mut d = 0xF & (v >> (4 * (4 - i)));
        if ! g.low_hands() { d ^= 0xF; }
        r.push(Rank::from_u8(d as u8));
    }

    if h.len() > 5 && (lvl == HandLevel::Flush || lvl == HandLevel::StraightFlush) {
        let mut counts = [0; 4];
        let mut flush_suit = Suit::None;

        // This code may fail for 10-card hands with two flushes
        for i in 0..h.len() {
            counts[h[i].suit() as usize - 1] += 1;
            if counts[h[i].suit() as usize - 1] > 4 {
                flush_suit = h[i].suit();
                break;
            }
        }
        for i in 0..r.len() {
            h[i] = Card::from_rank_suit(r[i], flush_suit);
        }
        return lvl;
    }
    for i in 0..r.len() {
        for j in i..h.len() {
            if h[j].rank() == r[i] {
                if i == j { continue; }
                if h[i].rank() != r[i] || h[j] > h[i] {
                    h[..].swap(i, j);
                }
            }
        }
    }
    lvl
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Description_Builder) | Builder for hand description
///
/// Builder for new HandDescription objects. Does some error checking while allowing
/// construction im multiple ways.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::HandDescriptionBuilder;
///
/// let hand = Hand::new(DeckType::English).init(hand!("As","Js","Ts","7s","5s"));
/// let v = HandDescriptionBuilder::new(&hand, Scale::HighHand)
///    .with_level(HandLevel::Flush)
///    .with_default_value().complete();
/// ```
pub struct HandDescriptionBuilder {
    hand: Hand,
    scale: Scale,
    level: Option<HandLevel>,
    value: Option<HandValue>,
    length: u8,
    extras: HandExtras,
}

impl HandDescriptionBuilder {
    /// Begin creating a new HandDescription object
    pub fn new(hand: &Hand, scale: Scale) -> Self {
        debug_assert!(scale.low_aces() == hand.deck_type().low_aces());
        debug_assert!(hand.len() > 1 && hand.len() <= 13);

        HandDescriptionBuilder {
            hand: *hand, scale, level: None, value: None,
            length: scale.complete_hand() as u8,
            extras: HandExtras::None,
        }
    }

    /// Set the level of the hand
    pub fn with_level(mut self, level: HandLevel) -> Self {
        debug_assert!(self.level.is_none());
        debug_assert!(HAND_LEVEL_WORST as u32 != self.scale.value_from_level(level));

        self.level = Some(level);
        self
    }

    /// Set the value of the hand to a custom number
    pub fn with_value(mut self, value: HandValue) -> Self {
        debug_assert!(self.value.is_none());

        self.value = Some(value);
        self
    }


    /// Set the value of the hand using the default calculation
    pub fn with_default_value(mut self) -> Self {
        debug_assert!(self.value.is_none());
        debug_assert!(self.level.is_some());

        self.value = Some(ojp_default_hand_value(&self.hand,
            self.scale, self.level.unwrap()));
        self
    }

    /// Set "meaningful" length for display
    pub fn truncate(mut self, l: usize) -> Self {
        self.length = l as u8;
        self
    }

    /// Finish building the HandDescription object
    pub fn complete(self) -> Result<HandDescription> {
        if self.level.is_none() {
            return Err(Error::BadDescription("level not set".into()));
        }
        if self.value.is_none() {
            return Err(Error::BadDescription("value not set".into()));
        }
        Ok(
            HandDescription {
                hand: self.hand,
                scale: self.scale,
                level: self.level.expect("can't happen"),
                value: self.value.expect("can't happen"),
                length: self.length,
                extras: self.extras,
            }
        )
    }
}

impl PartialEq for HandDescription {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for HandDescription {}

impl PartialOrd for HandDescription {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for HandDescription {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::hash::Hash for HandDescription {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{PartialOrd, PartialEq, Eq, Ord};
    use std::marker::{Sized, Send, Sync, Unpin};
    use std::fmt::{Debug, Display};

    fn has_traits<T: Debug + Display + PartialOrd + PartialEq + Eq + Ord + Clone + Copy +
        std::hash::Hash + std::default::Default + Sized + Send + Sync + Unpin>() {}

    #[test]
    fn test_hand_evaluator_high() -> Result<()> {
        has_traits::<HandLevel>();
        // has_traits::<HandExtras>(); No meaningful Ord
        has_traits::<HandDescription>();
        Ok(())
    }
}
