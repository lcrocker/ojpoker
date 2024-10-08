
use crate::cards::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandScale {
    HighHand = 1,
    AceToFive = 2,
    DeuceToSeven = 3,
    AceToSix = 4,
    Badugi = 5,
    PaiGow = 6,
    Manilla = 7,
    Mexican = 8,
    ActionRazz = 9,
}

pub trait HandLevelTrait {
    fn index(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevelHigh {
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
}

impl HandLevelTrait for HandLevelHigh {
    fn index(&self) -> u32 {
        *self as u32
    }
}

pub type HandLevelPaiGow = HandLevelHigh;
pub type HandLevelManilla = HandLevelHigh;
pub type HandLevelMexican = HandLevelHigh;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevelAceToFive {
    NoPair = 1,
    Pair = 2,
    TwoPair = 3,
    Trips = 4,
    FullHouse = 5,
    Quads = 6,
}

impl HandLevelTrait for HandLevelAceToFive {
    fn index(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevelDeuceToSeven {
    NoPair = 1,
    Pair = 2,
    TwoPair = 3,
    Trips = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    Quads = 8,
    StraightFlush = 9,
}

impl HandLevelTrait for HandLevelDeuceToSeven {
    fn index(&self) -> u32 {
        *self as u32
    }
}

pub type HandLevelAceToSix = HandLevelDeuceToSeven;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevelBadugi {
    FourCard = 1,
    ThreeCard = 2,
    TwoCard = 3,
    OneCard = 4,
}

impl HandLevelTrait for HandLevelBadugi {
    fn index(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum HandLevelActionRazz {
    QualifiedNoPair = 1,
    QualifiedPair = 2,
    QualifiedTwoPair = 3,
    QualifiedTrips = 4,
    QualifiedFullHouse = 5,
    QualifiedQuads = 6,
    UnqualifiedNoPair = 7,
    UnqualifiedPair = 8,
    UnqualifiedTwoPair = 9,
    UnqualifiedTrips = 10,
    UnqualifiedFullHouse = 11,
    UnqualifiedQuads = 12,
}

impl HandLevelTrait for HandLevelActionRazz {
    fn index(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug)]
pub struct HandValueBase<T: HandLevelTrait + Copy> {
    pub hand: Hand,
    pub scale: HandScale,
    pub level: T,
    pub ranks: Vec<Rank>,
    pub value: u64,
}

impl<T> HandValueBase<T>
where T: HandLevelTrait + Copy {
    pub fn new(hand: Hand, scale: HandScale, level: T, r: &[Rank], v: Option<u64>) -> HandValueBase<T> {
        HandValueBase {
            hand,
            scale,
            level,
            ranks: r.to_vec(),
            value : v.unwrap_or(
                HandValueBase::default_value_function(scale, level, r)),
        }
    }

    fn default_value_function(scale: HandScale, level: T, ranks: &[Rank]) -> u64 {
        let mut v: u64 = 0;

        for r in ranks {
            v <<= 4;
            v += 15 - (*r as u64);
        }
        100000000 * (scale as u64) + 2000000 * (level.index() as u64) + v
    }
}

impl<T: HandLevelTrait + Copy> PartialEq for HandValueBase<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: HandLevelTrait + Copy> PartialOrd for HandValueBase<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
