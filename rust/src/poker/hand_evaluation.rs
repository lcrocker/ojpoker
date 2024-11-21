//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Evaluation) | Poker hand evaluation types

use crate::error::{Error,Result};
use crate::utils::*;
use crate::cards::*;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Level) | Class for categories of poker hands.
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Description) | Info about an evaluated hand
///
/// This trait describes all the information about a hand after evaluation.
/// My vary from game to game (though every game should have these functions).
pub trait HandDescriptionTrait {
    /// Hand as modified during evaluation
    fn hand(&self) -> &Hand;
    /// Same, but as mutable pointer
    fn mut_hand(&mut self) -> &mut Hand;
    /// Again, but as slice, and trimmed to proper length
    fn as_slice(&self) -> &[Card];
    /// Comparison value: low is better
    fn value(&self) -> HandValue;
    /// Which scale/game we are evaluating for
    fn scale(&self) -> HandScale;
    /// Evaluated level of hand
    fn level(&self) -> HandLevel;
    /// Full english name, e.g. "aces full of kings"
    fn full_name(&self) -> String;
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Description) | Info about an evaluated hand
pub type HandDescription = Box<dyn HandDescriptionTrait>;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValue) | Complete descriptor of evaluated hand
///
/// Contains all the information about a hand's value after evaluation, including
/// a simple numeric comparator value for determining a winner, and also the hand
/// itself re-arranged for appropriate display, and the "level" of the hand and
/// scale used to evaluate it.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HandDescriptionBase {
    /// Copy of the hand under evaluation. Modified during evaluation.
    hand: Hand,
    /// Which game type we are evaluating for
    scale: HandScale,
    /// Level of the hand
    level: HandLevel,
    /// Comparison value: lower is better
    value: HandValue,
}

impl std::fmt::Display for HandDescriptionBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] = {}", self.hand, self.value)
    }
}

impl HandDescriptionBase {
    fn worst() -> Self {
        HandDescriptionBase {
            hand: Hand::new(DeckType::English),
            scale: HandScale::HighHand,
            level: HandLevel::None,
            value: HAND_VALUE_WORST,
        }
    }

    fn to_boxed_trait(self) -> HandDescription {
        Box::new(self)
    }
}

impl HandDescriptionTrait for HandDescriptionBase {
    fn hand(&self) -> &Hand { &self.hand }
    fn mut_hand(&mut self) -> &mut Hand { &mut self.hand }
    fn as_slice(&self) -> &[Card] { &self.hand[..self.scale.complete_hand()] }
    fn value(&self) -> HandValue { self.value }
    fn scale(&self) -> HandScale { self.scale }
    fn level(&self) -> HandLevel { self.level }
    fn full_name(&self) -> String {
        self.scale.full_name(&self.to_boxed_trait())
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
/// let v = HandDescriptionBuilder::new(&hand, HandScale::HighHand)
///    .with_level(HandLevel::Flush)
///    .with_default_value().complete();
/// ```
pub struct HandDescriptionBuilder {
    hand: Hand,
    scale: HandScale,
    level: Option<HandLevel>,
    value: Option<HandValue>,
}

impl HandDescriptionBuilder {
    /// Begin creating a new HandDescription object
    pub fn new(hand: &Hand, scale: HandScale) -> Self {
        debug_assert!(scale.low_aces() == hand.deck_type().low_aces());
        debug_assert!(hand.len() >= scale.complete_hand());

        HandDescriptionBuilder {
            hand: *hand, scale, level: None, value: None
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

    /// Finish building the HandDescription object
    pub fn complete(self) -> Result<HandDescription> {
        if self.level.is_none() {
            return Err(Error::BadDescription("level not set".into()));
        }
        if self.value.is_none() {
            return Err(Error::BadDescription("value not set".into()));
        }
        Ok(
            Box::new(
                HandDescriptionBase {
                    hand: self.hand, scale: self.scale,
                    level: self.level.expect("can't happen"),
                    value: self.value.expect("can't happen"),
                }
            )
        )
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_hand_value) | Default hand value calculator
pub fn ojp_default_hand_value(h: &Hand, g: HandScale, l: HandLevel) -> HandValue {
    let h: u32 = ojh_positional_32cs(&h[..g.complete_hand()])
        .expect("already checked");

    HAND_LEVEL_MULTIPLIER * g.value_from_level(l)
    + if g.low_hands() {
        h
    } else {
        h ^ 0xFFFFF
    }
}

/// Return just numeric value to compare hands.
pub type FixedHandEvaluatorQuick = fn(&Hand) -> HandValue;
/// Return full record of hand value info.
pub type FixedHandEvaluatorFull = fn(&Hand) -> Result<HandDescription>;


/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_of) | Best n-card hand from larger set
///
/// Given a large hand and fixed-length evaluator, find the best hand
/// of that length from the larger set.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_best_of,ojp_high_eval_5_full};
///
/// let hand = Hand::new(DeckType::English).
///     init(hand!("Qh","Js","5s","Ts","7d","7s","As"));
/// let v = ojp_best_of(&hand, HandScale::HighHand,
///     ojp_high_eval_5_full).unwrap();
/// // assert_eq!("AsJsTs7s5s", v.hand().to_string());
/// ```
pub fn ojp_best_of(h: &Hand, g: HandScale,
eval: FixedHandEvaluatorFull) -> Result<HandDescription> {
    let mut best_value = HAND_VALUE_WORST;
    let mut best: HandDescription = Box::new(HandDescriptionBase::worst());

    for sub in h.combinations(g.complete_hand()) {
        let desc = eval(&sub)?;
        if desc.value() < best_value {
            best_value = desc.value();
            best = desc;
        }
    }
    Ok(best)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_value_of) | Best n-card hand value from larger set
///
/// Given a large hand and evaluator, find the best 5-card hand value, no descrioption.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_best_value_of,ojp_high_eval_quick};
///
/// let h7 = Hand::new(DeckType::English).
///     init(hand!("Qh","Js","5s","Ts","7d","7s","As"));
/// let h5 = Hand::new(DeckType::English).
///     init(hand!("As","Js","Ts","7s","5s"));
/// let v7 = ojp_best_value_of(&h7, HandScale::HighHand,
///     ojp_high_eval_quick);
/// let v5 = ojp_high_eval_quick(&h5);
/// assert_eq!(v5, v7);
///
/// ```
pub fn ojp_best_value_of(h: &Hand, g: HandScale,
eval: FixedHandEvaluatorQuick) -> HandValue {
    let mut best = HAND_VALUE_WORST;

    for sub in h.combinations(g.complete_hand()) {
        let v = eval(&sub);
        if v < best {
            best = v;
        }
    }
    best
}

/*
 * Private functions called by the default evaluators.
 * These make many assumptions about how they are called and in what order,
 * so they are not generally useful outside of the default evaluators.
 */

fn is_sorted_descending<T: PartialOrd>(vals: &[T]) -> bool {
    for i in 1..vals.len() {
        if vals[i] > vals[i - 1] {
            return false;
        }
    }
    true
}

fn is_flush(h: &Hand) -> bool {
    if h.len() != 5 {
        return false;
    }
    let suit = h[0].suit();
    debug_assert!(suit != Suit::None);

    h[1].suit() == suit &&
    h[2].suit() == suit &&
    h[3].suit() == suit &&
    h[4].suit() == suit
}

/// Work around knight gap
const POKER_RANK_ORDER: [i8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, -1, 12, 13, 14
];

/// Mexican poker uses Spanish deck with no 8/9/10, and Q for C.
const MEXICAN_RANK_ORDER: [i8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, -1, -1, -1, 8, -1, 9, 10, 11
];

fn is_straight(h: &mut Hand, g: HandScale) -> bool {
    if h.len() != 5 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    if g.high_wheel() &&
        h[0].rank() == Rank::Ace &&
        h[1].rank() == Rank::Five &&
        h[2].rank() == Rank::Four &&
        h[3].rank() == Rank::Trey &&
        h[4].rank() == Rank::Deuce {

        if ! g.pai_gow_wheel() {
            let tc = h[0];
            h[0] = h[1];
            h[1] = h[2];
            h[2] = h[3];
            h[3] = h[4];
            h[4] = tc;
        }
        return true;
    }
    if g.low_broadway() &&
        h[0].rank() == Rank::King &&
        h[1].rank() == Rank::Queen &&
        h[2].rank() == Rank::Jack &&
        h[3].rank() == Rank::Ten &&
        h[4].rank() == Rank::LowAce {

        let tc = h[4];
        h[4] = h[3];
        h[3] = h[2];
        h[2] = h[1];
        h[1] = h[0];
        h[0] = tc;
        return true;
    }
    if g.spanish_gap() {
        for i in 1..5 {
            if MEXICAN_RANK_ORDER[h[i].rank() as usize] + 1 !=
                MEXICAN_RANK_ORDER[h[i - 1].rank() as usize] {

                return false;
            }
        }
    } else {
        for i in 1..5 {
            if POKER_RANK_ORDER[h[i].rank() as usize] + 1 !=
                POKER_RANK_ORDER[h[i - 1].rank() as usize] {

                return false;
            }
        }
    }
    true
}

fn is_quads(h: &mut Hand) -> bool {
    if h.len() < 4 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    // AAAAB
    if h[0].rank() == h[1].rank() &&
        h[0].rank() == h[2].rank() &&
        h[0].rank() == h[3].rank() {

        return true;
    }
    if h.len() < 5 {
        return false;
    }
    // BAAAA
    if h[1].rank() == h[2].rank() &&
        h[1].rank() == h[3].rank() &&
        h[1].rank() == h[4].rank() {

        let tc = h[0];
        h[0] = h[1];
        h[1] = h[2];
        h[2] = h[3];
        h[3] = h[4];
        h[4] = tc;
        return true;
    }
    false
}

fn is_full_house(h: &mut Hand) -> bool {
    if h.len() < 5 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    // AAABB
    if h[0].rank() == h[1].rank() &&
        h[0].rank() == h[2].rank() &&
        h[3].rank() == h[4].rank() {

        return true;
    }
    // BBAAA
    if h[0].rank() == h[1].rank() &&
        h[2].rank() == h[3].rank() &&
        h[2].rank() == h[4].rank() {

        let tc = h[0];
        h[0] = h[2];
        h[2] = h[4];
        h[4] = h[1];
        h[1] = h[3];
        h[3] = tc;
        return true;
    }
    false
}

fn is_trips(h: &mut Hand) -> bool {
    if h.len() < 3 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    // AAABC
    if h[0].rank() == h[1].rank() &&
        h[0].rank() == h[2].rank() {

        return true;
    }
    if h.len() < 4 {
        return false;
    }
    // BAAAC
    if h[1].rank() == h[2].rank() &&
        h[1].rank() == h[3].rank() {

        let tc = h[0];
        h[0] = h[1];
        h[1] = h[2];
        h[2] = h[3];
        h[3] = tc;
        return true;
    }
    if h.len() < 5 {
        return false;
    }
    // BCAAA
    if h[2].rank() == h[3].rank() &&
        h[2].rank() == h[4].rank() {

        let tc = h[0];
        h[0] = h[2];
        h[2] = h[4];
        h[4] = h[1];
        h[1] = h[3];
        h[3] = tc;
        return true;
    }
    false
}

fn is_two_pair(h: &mut Hand) -> bool {
    if h.len() < 4 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    // AABBC
    if h[0].rank() == h[1].rank() &&
        h[2].rank() == h[3].rank() {

        return true;
    }
    if h.len() < 5 {
        return false;
    }
    // AACBB
    if h[0].rank() == h[1].rank() &&
        h[3].rank() == h[4].rank() {

        let tc = h[2];
        h[2] = h[3];
        h[3] = h[4];
        h[4] = tc;
        return true;
    }
    // CAABB
    if h[1].rank() == h[2].rank() &&
        h[3].rank() == h[4].rank() {

        let tc = h[0];
        h[0] = h[1];
        h[1] = h[2];
        h[2] = h[3];
        h[3] = h[4];
        h[4] = tc;
        return true;
    }
    false
}

fn is_one_pair(h: &mut Hand) -> bool {
    if h.len() < 2 {
        return false;
    }
    debug_assert!(is_sorted_descending(&h[..]));

    // AABCD
    if h[0].rank() == h[1].rank() {
        return true;
    }
    if h.len() < 3 {
        return false;
    }
    // BAACD
    if h[1].rank() == h[2].rank() {
        let tc = h[0];
        h[0] = h[1];
        h[1] = h[2];
        h[2] = tc;
        return true;
    }
    if h.len() < 4 {
        return false;
    }
    // BCAAD
    if h[2].rank() == h[3].rank() {
        h[..].swap(0, 2);
        h[..].swap(1, 3);
        return true;
    }
    if h.len() < 5 {
        return false;
    }
    // BCDAA
    if h[3].rank() == h[4].rank() {
        let tc = h[0];
        h[0] = h[3];
        h[3] = h[1];
        h[1] = h[4];
        h[4] = h[2];
        h[2] = tc;
        return true;
    }
    false
}

fn verify_no_pair(h: &Hand) -> bool {
    if h.len() < 2 {
        return true;
    }
    if h[0].rank() == h[1].rank() {
        return false;
    }
    if h.len() < 3 {
        return true;
    }
    if h[0].rank() == h[2].rank() ||
        h[1].rank() == h[2].rank() {
       return false;
    }
    if h.len() < 4 {
        return true;
    }
    if h[0].rank() == h[3].rank() ||
        h[1].rank() == h[3].rank() ||
        h[2].rank() == h[3].rank() {
        return false;
    }
    if h.len() < 5 {
        return true;
    }
    h[0].rank() != h[4].rank() &&
    h[1].rank() != h[4].rank() &&
    h[2].rank() != h[4].rank() &&
    h[3].rank() != h[4].rank()
}

enum HandEvaluatorState {
    Initial,
    NotStraightOrFlush,
    Flush,
    NotFlush,
    NotQuads,
    NotFullHouse,
    NotTrips,
    NotTwoPair,
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_reference_eval_full) | Reference full hand evaluator
///
/// This function is the reference evaluator used to define and create
/// game-specific evaluators. It is not generally called by the user directly
/// unless you are coding a new game. Returns a full [HandDescription].
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_reference_evaluator_full};
///
/// let hand = Hand::new(DeckType::English).init(hand!("As","Js","Ts","7s","5s"));
/// let v = ojp_reference_evaluator_full(&hand, HandScale::HighHand).unwrap();
/// // assert_eq!(8969227, v.value());   // default value of AJT75 flush
/// ```
pub fn ojp_reference_evaluator_full(hand: &Hand, g: HandScale)
-> Result<HandDescription> {
    let mut h = *hand;
    debug_assert!(h.len() <= g.complete_hand());
    debug_assert!(h.deck_type().low_aces() == g.low_aces());

    oj_sort(&mut h[..]);
    let mut state = HandEvaluatorState::Initial;

    loop {
        match state {
            HandEvaluatorState::Initial => {
                state =
                if ! g.straights_and_flushes() {
                    HandEvaluatorState::NotStraightOrFlush
                } else if is_flush(&h) {
                    HandEvaluatorState::Flush
                } else {
                    HandEvaluatorState::NotFlush
                }
            },
            HandEvaluatorState::Flush => {
                if is_straight(&mut h, g) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::StraightFlush)
                        .with_default_value().complete();
                }
                return HandDescriptionBuilder::new(&h, g)
                    .with_level(HandLevel::Flush)
                    .with_default_value().complete();
            },
            HandEvaluatorState::NotFlush => {
                if is_straight(&mut h, g) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::Straight)
                        .with_default_value().complete();
                }
                state = HandEvaluatorState::NotStraightOrFlush;
            },
            HandEvaluatorState::NotStraightOrFlush => {
                if h[0].rank() == h[1].rank() &&
                    h[0].rank() == h[2].rank() &&
                    h[0].rank() == h[3].rank() &&
                    h[0].rank() == h[4].rank() {

                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::FiveOfAKind)
                        .with_default_value().complete();
                }
                if is_quads(&mut h) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::Quads)
                        .with_default_value().complete();
                }
                state = HandEvaluatorState::NotQuads;
            },
            HandEvaluatorState::NotQuads => {
                if is_full_house(&mut h) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::FullHouse)
                        .with_default_value().complete();
                }
                state = HandEvaluatorState::NotFullHouse;
            },
            HandEvaluatorState::NotFullHouse => {
                if is_trips(&mut h) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::Trips)
                        .with_default_value().complete();
                }
                state = HandEvaluatorState::NotTrips;
            },
            HandEvaluatorState::NotTrips => {
                if is_two_pair(&mut h) {
                    return HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::TwoPair)
                        .with_default_value().complete();
                }
                state = HandEvaluatorState::NotTwoPair;
            },
            HandEvaluatorState::NotTwoPair => {
                return if is_one_pair(&mut h) {
                    HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::Pair)
                        .with_default_value().complete()
                } else {
                    debug_assert!(verify_no_pair(&h));

                    HandDescriptionBuilder::new(&h, g)
                        .with_level(HandLevel::NoPair)
                        .with_default_value().complete()
                }
            },
        }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_eval_quick) | Default quick hand evaluator
///
/// This function is the default "quick" hand evaluator used to create
/// game-specific evaluators. It is not generally called by the user directly
/// unless you are coding a new game. Produces comparator value only.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_reference_evaluator_quick};
///
/// let hand = Hand::new(DeckType::English).init(hand!("As","Js","Ts","7s","5s"));
/// let cmp = ojp_reference_evaluator_quick(&hand, HandScale::HighHand);
/// // assert_eq!(8969227, cmp);
/// ```
pub fn ojp_reference_evaluator_quick(hand: &Hand, g: HandScale)
-> HandValue {
    let mut h = *hand;
    debug_assert!(h.len() <= g.complete_hand());
    debug_assert!(h.deck_type().low_aces() == g.low_aces());

    oj_sort(&mut h[..]);
    let mut state = HandEvaluatorState::Initial;

    loop {
        match state {
            HandEvaluatorState::Initial => {
                state =
                if ! g.straights_and_flushes() {
                    HandEvaluatorState::NotStraightOrFlush
                } else if is_flush(&h) {
                    HandEvaluatorState::Flush
                } else {
                    HandEvaluatorState::NotFlush
                }
            },
            HandEvaluatorState::Flush => {
                if is_straight(&mut h, g) {
                    return ojp_default_hand_value(&h, g, HandLevel::StraightFlush);
                }
                return ojp_default_hand_value(&h, g,HandLevel::Flush);
            },
            HandEvaluatorState::NotFlush => {
                if is_straight(&mut h, g) {
                    return ojp_default_hand_value(&h, g, HandLevel::Straight);
                }
                state = HandEvaluatorState::NotStraightOrFlush;
            },
            HandEvaluatorState::NotStraightOrFlush => {
                if is_quads(&mut h) {
                    return ojp_default_hand_value(&h, g, HandLevel::Quads);
                }
                state = HandEvaluatorState::NotQuads;
            },
            HandEvaluatorState::NotQuads => {
                if is_full_house(&mut h) {
                    return ojp_default_hand_value(&h, g,HandLevel::FullHouse);
                }
                state = HandEvaluatorState::NotFullHouse;
            },
            HandEvaluatorState::NotFullHouse => {
                if is_trips(&mut h) {
                    return ojp_default_hand_value(&h, g, HandLevel::Trips);
                }
                state = HandEvaluatorState::NotTrips;
            },
            HandEvaluatorState::NotTrips => {
                if is_two_pair(&mut h) {
                    return ojp_default_hand_value(&h, g,HandLevel::TwoPair);
                }
                state = HandEvaluatorState::NotTwoPair;
            },
            HandEvaluatorState::NotTwoPair => {
                return if is_one_pair(&mut h) {
                    ojp_default_hand_value(&h, g, HandLevel::Pair)
                } else {
                    debug_assert!(verify_no_pair(&h));
                    ojp_default_hand_value(&h, g, HandLevel::NoPair)
                }
            },
        }
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
