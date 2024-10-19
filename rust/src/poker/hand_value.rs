//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Value) | Poker hand evaluation

use crate::errors::*;
use crate::cards::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Level) | Base class for categories of poker hands.
/// 
/// Poker hands are ranked by first grouping them into categories (which
/// here we call "level"), and then comparing the ranks of the cards
/// within that level to break ties. We use the actual numbers here
/// for calculating comparators, and comparators are valued such that
/// lower numbers mean better hands.
pub trait HandLevelTrait: Copy {
    /// Integer value of the level, used in comparator calculations.
    fn index(&self) -> u32;
    /// Convert an integer index to a level.
    fn from_index(i: u32) -> Self;
    /// Level representing the best hand: this is the lowest number.
    fn best() -> Self;
    /// Level representing the worst hand: this is the highest number.
    fn worst() -> Self;
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelHigh) | Traditional "high" poker hands
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
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
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> HandLevelHigh {
        match i {
            1 => HandLevelHigh::FiveOfAKind,
            2 => HandLevelHigh::StraightFlush,
            3 => HandLevelHigh::Quads,
            4 => HandLevelHigh::FullHouse,
            5 => HandLevelHigh::Flush,
            6 => HandLevelHigh::Straight,
            7 => HandLevelHigh::Trips,
            8 => HandLevelHigh::TwoPair,
            9 => HandLevelHigh::Pair,
            10 => HandLevelHigh::NoPair,
            _ => panic!("invalid HandLevelHigh index"),
        }
    }
    fn best() -> HandLevelHigh { HandLevelHigh::FiveOfAKind }
    fn worst() -> HandLevelHigh { HandLevelHigh::NoPair }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelHigh) | Pai Gow hand levels
///  Pai Gow uses same high hands, but slightly different rules.
pub type HandLevelPaiGow = HandLevelHigh;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelStripped) | Stripped-deck poker hands
/// High poker hands for stripped deck games where flush beats full house.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum HandLevelStripped {
    FiveOfAKind = 1,
    StraightFlush = 2,
    Quads = 3,
    Flush = 4,
    FullHouse = 5,
    Straight = 6,
    Trips = 7,
    TwoPair = 8,
    Pair = 9,
    NoPair = 10,
}

impl HandLevelTrait for HandLevelStripped {
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> HandLevelStripped {
        match i {
            1 => HandLevelStripped::FiveOfAKind,
            2 => HandLevelStripped::StraightFlush,
            3 => HandLevelStripped::Quads,
            4 => HandLevelStripped::Flush,
            5 => HandLevelStripped::FullHouse,
            6 => HandLevelStripped::Straight,
            7 => HandLevelStripped::Trips,
            8 => HandLevelStripped::TwoPair,
            9 => HandLevelStripped::Pair,
            10 => HandLevelStripped::NoPair,
            _ => panic!("invalid HandLevelHigh index"),
        }
    }
    fn best() -> HandLevelStripped { HandLevelStripped::FiveOfAKind }
    fn worst() -> HandLevelStripped { HandLevelStripped::NoPair }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelStripped) | Manilla poker hand levels
/// Manilla uses stripped deck high hands.
pub type HandLevelManilla = HandLevelStripped;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelStripped) | Mexican poker hand levels
/// Mexican poker uses stripped deck high hands.
pub type HandLevelMexican = HandLevelStripped;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelAceToFive) | Ace-to-five lowball hand levels
/// Ace-to-five lowball hands, e.g. Razz, California lowball, Stud/8, etc.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum HandLevelAceToFive {
    NoPair = 1,
    Pair = 2,
    TwoPair = 3,
    Trips = 4,
    FullHouse = 5,
    Quads = 6,
}

impl HandLevelTrait for HandLevelAceToFive {
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> Self {
        match i {
            1 => HandLevelAceToFive::NoPair,
            2 => HandLevelAceToFive::Pair,
            3 => HandLevelAceToFive::TwoPair,
            4 => HandLevelAceToFive::Trips,
            5 => HandLevelAceToFive::FullHouse,
            6 => HandLevelAceToFive::Quads,
            _ => panic!("invalid HandLevelAceToFive index"),
        }
    }
    fn best() -> HandLevelAceToFive { HandLevelAceToFive::NoPair }
    fn worst() -> HandLevelAceToFive { HandLevelAceToFive::Quads }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelDeuceToSeven) | Deuce-to-seven lowball hand levels
/// Deuce-to-seven lowball hands, e.g. Kansas City lowball, triple draw, etc.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
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
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> Self {
        match i {
            1 => HandLevelDeuceToSeven::NoPair,
            2 => HandLevelDeuceToSeven::Pair,
            3 => HandLevelDeuceToSeven::TwoPair,
            4 => HandLevelDeuceToSeven::Trips,
            5 => HandLevelDeuceToSeven::Straight,
            6 => HandLevelDeuceToSeven::Flush,
            7 => HandLevelDeuceToSeven::FullHouse,
            8 => HandLevelDeuceToSeven::Quads,
            9 => HandLevelDeuceToSeven::StraightFlush,
            _ => panic!("invalid HandLevelDeuceToSeven index"),
        }
    }
    fn best() -> HandLevelDeuceToSeven { HandLevelDeuceToSeven::NoPair }
    fn worst() -> HandLevelDeuceToSeven { HandLevelDeuceToSeven::StraightFlush }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelDeuceToSeven) | Ace-to-six lowball hand levels
/// Ace-to-six lowball hands, e.g. London lowball, etc. Same as Deuce-to-seven.
pub type HandLevelAceToSix = HandLevelDeuceToSeven;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelBadugi) | Badugi hand levels
/// Badugi: any four-card hand beats any 3-card, etc.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum HandLevelBadugi {
    FourCard = 1,
    ThreeCard = 2,
    TwoCard = 3,
    OneCard = 4,
}

impl HandLevelTrait for HandLevelBadugi {
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> Self {
        match i {
            1 => HandLevelBadugi::FourCard,
            2 => HandLevelBadugi::ThreeCard,
            3 => HandLevelBadugi::TwoCard,
            4 => HandLevelBadugi::OneCard,
            _ => panic!("invalid HandLevelBadugi index"),
        }
    }
    fn best() -> HandLevelBadugi { HandLevelBadugi::FourCard }
    fn worst() -> HandLevelBadugi { HandLevelBadugi::OneCard }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandLevelActionRazz) | Action Razz hand levels
/// Action Razz: any "qualified" hand with paint beats any "unqualified" hand.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[allow(missing_docs)]
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
    fn index(&self) -> u32 { *self as u32 }
    fn from_index(i: u32) -> Self {
        match i {
            1 => HandLevelActionRazz::QualifiedNoPair,
            2 => HandLevelActionRazz::QualifiedPair,
            3 => HandLevelActionRazz::QualifiedTwoPair,
            4 => HandLevelActionRazz::QualifiedTrips,
            5 => HandLevelActionRazz::QualifiedFullHouse,
            6 => HandLevelActionRazz::QualifiedQuads,
            7 => HandLevelActionRazz::UnqualifiedNoPair,
            8 => HandLevelActionRazz::UnqualifiedPair,
            9 => HandLevelActionRazz::UnqualifiedTwoPair,
            10 => HandLevelActionRazz::UnqualifiedTrips,
            11 => HandLevelActionRazz::UnqualifiedFullHouse,
            12 => HandLevelActionRazz::UnqualifiedQuads,
            _ => panic!("invalid HandLevelActionRazz index"),
        }
    }
    fn best() -> HandLevelActionRazz { HandLevelActionRazz::QualifiedNoPair }
    fn worst() -> HandLevelActionRazz { HandLevelActionRazz::UnqualifiedQuads }
}

/// Hand value calculation that works for many high-hand games.
pub fn oj_high_hand_value_function(lvl: u32, r: &[Rank]) -> u64 {
    let h: u64 = ojh_positional_u64csr(r).expect("should be checked earlier");
    10000000 * (lvl as u64) - h
}
/// Hand value calculation that works for many low-hand games.
pub fn oj_low_hand_value_function(lvl: u32, r: &[Rank]) -> u64 {
    let h: u64 = ojh_positional_u64csr(r).expect("should be checked earlier");
    10000000 * (lvl as u64) + h
}

/// Arrange the hand for display. E.g. "9d3h3cKs3s" -> "3s3h3cKs9d".
pub fn oj_default_ordered_for_display(h: &Hand, r: &[Rank]) -> aResult<Hand> {
    let mut h_in = h.clone();
    if r.len() != 5 || h_in.len() != 5 {
        bail!(OjError::BadHand(format!("{} {}", r.len(), h_in.len())));
    }
    let mut h_out = h.clone();
    h_out.clear();

    for i in 0..5 {
        let r = r[i];
        let mut found = Card(0);
        let mut f_index: i32 = -1;

        for j in 0..h_in.len() {
            if h_in[j].rank()? == r && h_in[j] > found {
                found = h_in[j];
                f_index = j as i32;
            }
        }
        debug_assert!(f_index >= 0);
        debug_assert!(found == h_in.remove_at(f_index as usize).
            expect("cannot happen"));
        h_out.push(found);
    }
    debug_assert!(h_out.len() == 5);
    aOk(h_out)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueTrait) | Hand value common code
/// Common interface to hand value objects.
pub trait HandValueTrait {
    /// Return the comparator value of this hand
    fn value(&self) -> u64;
    /// Best hand of this game: not necessarily a valid hand
    fn best() -> Self;
    /// Worst hand of this game: not necessarily a valid hand
    fn worst() -> Self;
    /// Full English name of hand, e.g. "aces and fours with a nine"
    fn full_name(&self) -> String;
    /// Return a new hand re-ordered for more meaningful display.
    fn ordered_for_display(&self, h: &Hand) -> aResult<Hand>;
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValue) | Hand value information structure
/// All the information resulting for evaluating a poker hand.
/// 
/// This is used for comparing hands to determine a winner, and also for
/// displaying the hand appropriately.
#[derive(Debug, Clone)]
pub struct HandValue<L: HandLevelTrait> {
    /// Value category of the hand; type varies by game.
    pub level: L,
    /// Array of ranks of the cards in the hand.
    pub ranks: Vec<Rank>,
    /// Calculated numeric comparator: low is better.
    pub value: u64,
}

impl<T: HandLevelTrait> PartialEq for HandValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: HandLevelTrait> Eq for HandValue<T> {}

impl<T: HandLevelTrait> PartialOrd for HandValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: HandLevelTrait> Ord for HandValue<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}


/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorTrait) | Common layout of hand evaluators
/// All specific game hand evaluators must implement this trait.
pub trait HandEvaluatorTrait<V: HandValueTrait> {
    /// Default evaluator, must implement.
    fn reference_evaluator(&self, _h: &Hand) -> aResult<V>;

    /// Evaluator for partial hands, for determining stud betting, etc.
    fn partial_evaluator(&self, h: &Hand) -> aResult<V> {
        self.reference_evaluator(h)
    }

    /// Fast lookup-table based evaluator.
    fn lookup_evaluator(&self, h: &Hand) -> aResult<V> {
        self.reference_evaluator(h)
    }

    /// Fast no-frills evaluator if all you care about is what wins.
    fn fast_value(&self, h: &Hand) -> u64 {
        let v = self.lookup_evaluator(h).
            expect("must implement reference_evaluator");
        v.value()
    }

    /// General-use entry point that picks the right evaluator.
    fn value_of(&self, h: &Hand) -> aResult<V> {
        match h.len() {
            1..=4 => self.partial_evaluator(h),
            5 => self.lookup_evaluator(h),
            6..=13 => {
                let mut best = V::worst();
                for sub in h.combinations(5) {
                    let v = self.lookup_evaluator(&sub)?;
                    if v.value() < best.value() {
                        best = v;
                    }
                }
                aOk(best)
            }
            _ => bail!(OjError::BadHand(format!("{} cards", h.len()))),
        }
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_level() -> aResult<()> {
        assert_eq!(HandLevelHigh::best(), HandLevelHigh::FiveOfAKind);
        assert_eq!(HandLevelHigh::worst(), HandLevelHigh::NoPair);
        assert_eq!(HandLevelHigh::from_index(1), HandLevelHigh::FiveOfAKind);
        assert_eq!(HandLevelHigh::from_index(10), HandLevelHigh::NoPair);
        aOk(())
    }

    #[test]
    fn test_hand_value() -> aResult<()> {
        let hv = HandValue::<HandLevelHigh> {
            level: HandLevelHigh::NoPair,
            ranks: vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten],
            value: 10000000 * HandLevelHigh::NoPair.index() as u64 + 0x1FEDCBA987654321,
        };
        assert_eq!(hv.value, 10000000 * HandLevelHigh::NoPair.index() as u64 + 0x1FEDCBA987654321);
        assert_eq!(hv.level, HandLevelHigh::NoPair);
        assert_eq!(hv.ranks, vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);
        aOk(())
    }
}
