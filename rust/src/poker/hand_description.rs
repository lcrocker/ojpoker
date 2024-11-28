//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Description) | Evaluated hand data

use crate::error::{Error,Result};
use crate::cards::{DeckType,Hand};
use crate::poker::Scale;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Level) | Class for categories of poker hands
///
/// Poker hands are ranked by first grouping them into categories (which
/// here we call "level"), and then comparing the ranks of the cards
/// within that level to break ties. We use the actual numbers here to
/// index into tables for calculating comparator values.
#[allow(missing_docs)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevel {
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

/// The plain integer value used to compare hands--low value wins.
pub type HandValue = u32;

/// For all hand value comparisons, lower is better.
pub const HAND_VALUE_BEST: u32 = 0;
/// Maximum u32
pub const HAND_VALUE_WORST: u32 = 0xFFFF_FFFF;
/// Should be enough to leave room for all rank combinations between levels
pub const HAND_LEVEL_MULTIPLIER: u32 = 0x100000;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Extras) | Extra information about a hand
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum HandExtras {
    /// No extra information
    None = 0,
    /// Cards that each bug represents
    Bugs([u8;2]),
    /// Cards that each wildcard represents
    Wilds([u8;4]),
    /// Bug replacement, up or down?
    Mexican(u8,bool),
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
    /// Full english name, e.g. "aces full of kings"
    pub fn full_name(&self) -> String {
        self.scale.full_name(self)
    }

    /// Worst hand to use as sentinel value
    pub fn worst() -> Self {
        HandDescription {
            hand: Hand::new(DeckType::English),
            length: 0,
            scale: Scale::None,
            level: HandLevel::None,
            value: HAND_VALUE_WORST,
            extras: HandExtras::None,
        }
    }
}

impl std::fmt::Display for HandDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} [{}] = {}", self.scale.name(), self.hand, self.value)
    }
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
            length: hand.len() as u8, extras: HandExtras::None,
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

use crate::cards::hashes::ojh_positional_32cs;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_hand_value) | Default hand value calculator
pub fn ojp_default_hand_value(h: &Hand, g: Scale, l: HandLevel) -> HandValue {
    let h: u32 = ojh_positional_32cs(&h[..g.complete_hand()])
        .expect("already checked");

    HAND_LEVEL_MULTIPLIER * g.value_from_level(l)
    + if g.low_hands() {
        h
    } else {
        h ^ 0xFFFFF
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_high() -> Result<()> {
        Ok(())
    }
}
