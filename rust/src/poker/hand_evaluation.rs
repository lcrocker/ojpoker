//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand_Evaluation) | Poker hand evaluation types

use crate::utils::*;
use crate::cards::*;
use crate::errors::*;
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
    UnqualifiedQuads = 15, // Action Razz
    UnqualifiedFullHouse = 16,
    UnqualifiedTrips = 17,
    UnqualifiedTwoPair = 18,
    UnqualifiedPair = 19,
    UnqualifiedNoPair = 20,
}
/// Need to allocate array in game info
const POKER_HAND_LEVEL_COUNT: usize = 21;

/// Used to create level from integer
const LEVELS: [HandLevel; POKER_HAND_LEVEL_COUNT] = [
    HandLevel::None,
    HandLevel::FiveOfAKind,
    HandLevel::StraightFlush,
    HandLevel::Quads,
    HandLevel::FullHouse,
    HandLevel::Flush,
    HandLevel::Straight,
    HandLevel::Trips,
    HandLevel::TwoPair,
    HandLevel::Pair,
    HandLevel::NoPair,
    HandLevel::FourCard,
    HandLevel::ThreeCard,
    HandLevel::TwoCard,
    HandLevel::OneCard,
    HandLevel::UnqualifiedQuads,
    HandLevel::UnqualifiedFullHouse,
    HandLevel::UnqualifiedTrips,
    HandLevel::UnqualifiedTwoPair,
    HandLevel::UnqualifiedPair,
    HandLevel::UnqualifiedNoPair,
];

impl HandLevel {
    /// How many levels?
    pub const fn count() -> usize {
        POKER_HAND_LEVEL_COUNT
    }

    /// Convert integer to level
    pub const fn from_u8(v: u8) -> Self {
        if (v as usize) >= POKER_HAND_LEVEL_COUNT {
            HandLevel::None
        } else {
            LEVELS[v as usize]
        }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValue) | Complete descriptor of evaluated hand
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct HandValue {
    /// The hand under evaluation. Order of cards may be changed.
    pub hand: Hand,
    /// Comparison value: lower is better.
    pub value: u32,
    /// Index of the game in the game info array.
    pub scale: u8,
    /// Level of the hand.
    pub level: u8,
}

impl HandValue {
    /// Create a new hand value with the given game, hand, and level.
    pub fn new(hand: Hand, scale: HandScale, level: HandLevel) -> Self {
        debug_assert!(scale.low_aces() == hand.deck_type().low_aces());

        HandValue {
            hand, scale: scale as u8, level: level as u8,
            value: ojp_default_hand_value(&hand, scale,  level),
        }
    }

    /// Create a new hand value with the given game, hand, level, and value.
    pub fn new_with_value(hand: Hand, scale: HandScale,
    level: HandLevel, value: u32) -> Self {
        debug_assert!(scale.low_aces() == hand.deck_type().low_aces());

        HandValue {
            hand, scale: scale as u8, level: level as u8, value,
        }
    }

    /// Expose hand as slice of cards.
    pub fn as_slice(&self) -> &[Card] {
        self.hand.as_slice()
    }

    /// Clone the hand.
    pub fn hand(&self) -> Hand {
        self.hand
    }

    /// Call full_name function from game info.
    pub fn full_name(&self) -> String {
        (HandScale::from_u8(self.scale).full_name())(self)
    }

    /// Rearrange cards in the rank order given.
    pub fn order_for_display(&mut self, ranks: &[Rank]) {
        debug_assert!(ranks.len() <= self.hand.len());

        for i in 0..(ranks.len() - 1) {
            if Rank::None == ranks[i] { break; }

            for j in i..ranks.len() {
                if self.hand[j].rank() == ranks[i] {
                    if i != j { self.hand.as_mut_slice().swap(i, j); }
                    break;
                }
            }
        }
    }
}

impl PartialEq for HandValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for HandValue {}

impl Ord for HandValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for HandValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Hand value calculation that works for many high-hand games.
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_hand_value) | Default hand value calculation
pub fn ojp_default_hand_value(h: &Hand, g: HandScale, p: HandLevel) -> u32 {
    let h: u32 = ojh_positional_32cs(h.as_slice()).
        expect("should be checked earlier");

    if g.low_hands() {
        g.multiplier() * g.value_from_level(p) + h
    } else {
        g.multiplier() * g.value_from_level(p) - h
    }
}

/// Return just numeric value to compare hands.
pub type HandEvaluatorQuick = fn(&Hand) -> u32;
/// Return full record of hand value info.
pub type HandEvaluatorFull = fn(&Hand) -> Result<HandValue, OjError>;

/// Given a large hand and evaluator, find the best 5-card hand.
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_of) | Best n-card hand from larger set
pub fn ojp_best_of(h: &Hand, g: HandScale,
eval: HandEvaluatorFull) -> Result<HandValue, OjError> {
    let mut best = g.worst();

    for sub in h.combinations(g.complete_hand()) {
        let v = eval(&sub)?;
        if v.value < best.value {
            best = v;
        }
    }
    Ok(best)
}

/// Given a large hand and evaluator, find the best 5-card hand value.
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_value_of) | Best n-card hand value from larger set
pub fn ojp_best_value_of(h: &Hand, g: HandScale,
eval: HandEvaluatorQuick) -> u32 {
    let mut best = 0xFFFFFFFF;

    for sub in h.combinations(g.complete_hand()) {
        let v = eval(&sub);
        if v < best {
            best = v;
        }
    }
    best
}

/// Is this a valid hand for the game?
/// [wiki](http://github.com/lcrocker/ojpoker/wiki/ojp_valid_hand_for_game) | Check if hand is valid for game
pub fn ojp_valid_hand_for_game(hand: &Hand, g: HandScale) -> bool {
    if hand.is_empty() { return false; }
    if hand.len() > g.complete_hand() { return false; }

    if g.low_aces() {
        if ! hand.deck_type().low_aces() { return false; }

        for c in hand {
            let r = c.rank();
            if r == Rank::Ace || r == Rank::None || r == Rank::Knight {
                return false;
            }
        }
    } else {
        if hand.deck_type().low_aces() { return false; }

        for c in hand {
            let r = c.rank();
            if r == Rank::LowAce || r == Rank::None || r == Rank::Knight {
                return false;
            }
        }
    }
    true
}

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

fn is_straight(h: &mut Hand, g: HandScale) -> bool {
    if h.len() != 5 {
        return false;
    }
    debug_assert!(is_sorted_descending(h.as_slice()));

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
    for i in 1..5 {
        if POKER_RANK_ORDER[h[i].rank() as usize] + 1 !=
            POKER_RANK_ORDER[h[i - 1].rank() as usize] {

            return false;
        }
    }
    true
}

fn is_quads(h: &mut Hand) -> bool {
    if h.len() < 4 {
        return false;
    }
    debug_assert!(is_sorted_descending(h.as_slice()));

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
    debug_assert!(is_sorted_descending(h.as_slice()));

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
    debug_assert!(is_sorted_descending(h.as_slice()));

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
    debug_assert!(is_sorted_descending(h.as_slice()));

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
    debug_assert!(is_sorted_descending(h.as_slice()));

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
        h.as_mut_slice().swap(0, 2);
        h.as_mut_slice().swap(1, 3);
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

/// Default "full" hand evaluator for most poker games.
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_eval_full) | Default full hand evaluator
pub fn ojp_default_eval_full(hand: &Hand, g: HandScale)
-> Result<HandValue, OjError> {
    debug_assert!(ojp_valid_hand_for_game(hand, g));

    let mut h = *hand;
    oj_sort(h.as_mut_slice());
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
                    return Ok(HandValue::new(h, g,
                        HandLevel::StraightFlush));
                }
                return Ok(HandValue::new(h, g,
                    HandLevel::Flush));
            },
            HandEvaluatorState::NotFlush => {
                if is_straight(&mut h, g) {
                    return Ok(HandValue::new(h, g,
                        HandLevel::Straight));
                }
                state = HandEvaluatorState::NotStraightOrFlush;
            },
            HandEvaluatorState::NotStraightOrFlush => {
                // Special case: shouldn't be able to get here my
                // normal means, but we need this for the programs
                // that build lookup tables.
                if h[0].rank() == h[1].rank() &&
                    h[0].rank() == h[2].rank() &&
                    h[0].rank() == h[3].rank() &&
                    h[0].rank() == h[4].rank() {

                    return Ok(HandValue::new(h, g,
                        HandLevel::FiveOfAKind));
                } 
                if is_quads(&mut h) {
                    return Ok(HandValue::new(h, g,
                        HandLevel::Quads));
                }
                state = HandEvaluatorState::NotQuads;
            },
            HandEvaluatorState::NotQuads => {
                if is_full_house(&mut h) {
                    return Ok(HandValue::new(h, g,
                        HandLevel::FullHouse));
                }
                state = HandEvaluatorState::NotFullHouse;
            },
            HandEvaluatorState::NotFullHouse => {
                if is_trips(&mut h) {
                    return Ok(HandValue::new(h, g,
                        HandLevel::Trips));
                }
                state = HandEvaluatorState::NotTrips;
            },
            HandEvaluatorState::NotTrips => {
                if is_two_pair(&mut h) {
                    return Ok(HandValue::new(h, g,
                        HandLevel::TwoPair));
                }
                state = HandEvaluatorState::NotTwoPair;
            },
            HandEvaluatorState::NotTwoPair => {
                return if is_one_pair(&mut h) {
                    Ok(HandValue::new(h, g,
                        HandLevel::Pair))
                } else {
                    debug_assert!(verify_no_pair(&h));
                    Ok(HandValue::new(h, g,
                        HandLevel::NoPair))
                }
            },
        }
    }
}

/// Default "value only" hand evaluator for most poker games.
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_eval_quick) | Default quick hand evaluator
pub fn ojp_default_eval_quick(hand: &Hand, g: HandScale)
-> u32 {
    debug_assert!(ojp_valid_hand_for_game(hand, g));

    let mut h = *hand;
    oj_sort(h.as_mut_slice());
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
                    return ojp_default_hand_value(&h, g,
                        HandLevel::StraightFlush);
                }
                return ojp_default_hand_value(&h, g,
                    HandLevel::Flush);
            },
            HandEvaluatorState::NotFlush => {
                if is_straight(&mut h, g) {
                    return ojp_default_hand_value(&h, g,
                        HandLevel::Straight);
                }
                state = HandEvaluatorState::NotStraightOrFlush;
            },
            HandEvaluatorState::NotStraightOrFlush => {
                if is_quads(&mut h) {
                    return ojp_default_hand_value(&h, g,
                        HandLevel::Quads);
                }
                state = HandEvaluatorState::NotQuads;
            },
            HandEvaluatorState::NotQuads => {
                if is_full_house(&mut h) {
                    return ojp_default_hand_value(&h, g,
                        HandLevel::FullHouse);
                }
                state = HandEvaluatorState::NotFullHouse;
            },
            HandEvaluatorState::NotFullHouse => {
                if is_trips(&mut h) {
                    return ojp_default_hand_value(&h, g,
                        HandLevel::Trips);
                }
                state = HandEvaluatorState::NotTrips;
            },
            HandEvaluatorState::NotTrips => {
                if is_two_pair(&mut h) {
                    return ojp_default_hand_value(&h, g,
                        HandLevel::TwoPair);
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
    fn test_hand_evaluator_high() -> Result<(), OjError> {
        Ok(())
    }
}