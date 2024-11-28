//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Reference_Evaluators) | Standard evaluators

use crate::error::Result;
use crate::utils::*;
use crate::cards::*;
use crate::poker::*;

/// Return just numeric value to compare hands.
pub type FixedHandEvaluatorQuick = fn(&Hand) -> HandValue;
/// Return full record of hand value info.
pub type FixedHandEvaluatorFull = fn(&Hand) -> Result<HandDescription>;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_of) | Best n-card hand from larger set
///
/// Given a large hand and fixed-length evaluator, find the best hand
/// of that length from the larger set.
pub fn ojp_best_of(h: &Hand, g: Scale,
eval: FixedHandEvaluatorFull) -> Result<HandDescription> {
    let mut best_value = HAND_VALUE_WORST;
    let mut best: HandDescription = HandDescription::worst();
    debug_assert!(h.len() >= g.complete_hand());

    for sub in h.combinations(g.complete_hand()) {
        let desc = eval(&sub)?;
        if desc.value < best_value {
            best_value = desc.value;
            best = desc;
        }
    }
    Ok(best)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_value_of) | Best n-card hand value from larger set
///
/// Given a large hand and evaluator, find the best 5-card hand value, no description.
pub fn ojp_best_value_of(h: &Hand, g: Scale,
eval: FixedHandEvaluatorQuick) -> HandValue {
    let mut best = HAND_VALUE_WORST;
    debug_assert!(h.len() >= g.complete_hand());

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

fn is_flush(h: &Hand, g: Scale) -> bool {
    if h.len() != g.complete_hand() {
        return false;
    }
    let suit = h[0].suit();
    debug_assert!(suit != Suit::None);

    for i in 1..h.len() {
        if h[i].suit() != suit {
            return false;
        }
    }
    true
}

/// Work around knight gap
const POKER_RANK_ORDER: [i8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, -1, 12, 13, 14
];

/// Mexican poker uses Spanish deck with no 8/9/10, and Q for C.
const MEXICAN_RANK_ORDER: [i8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, -1, -1, -1, 8, -1, 9, 10, 11
];

fn is_straight(h: &mut Hand, g: Scale) -> bool {
    debug_assert!(is_sorted_descending(&h[..]));
    if h.len() != g.complete_hand() {
        return false;
    }
    if 5 == g.complete_hand() {
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
    }
    if g.spanish_gap() {
        for i in 1..g.complete_hand() {
            if MEXICAN_RANK_ORDER[h[i].rank() as usize] + 1 !=
                MEXICAN_RANK_ORDER[h[i - 1].rank() as usize] {

                return false;
            }
        }
    } else {
        for i in 1..g.complete_hand() {
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
/// let v = ojp_reference_evaluator_full(&hand, Scale::HighHand).unwrap();
/// // assert_eq!(8969227, v.value());   // default value of AJT75 flush
/// ```
pub fn ojp_reference_evaluator_full(hand: &Hand, g: Scale)
-> Result<HandDescription> {
    let mut h = *hand;
    debug_assert!(h.len() <= g.complete_hand());
    debug_assert!(h.deck_type().low_aces() == g.low_aces());

    if h.is_empty() {
        return HandDescriptionBuilder::new(&h, g)
            .with_level(HandLevel::None)
            .with_value(HAND_VALUE_WORST).complete();
    }
    oj_sort(&mut h[..]);
    let mut state = HandEvaluatorState::Initial;

    loop {
        match state {
            HandEvaluatorState::Initial => {
                state =
                if ! g.straights_and_flushes() {
                    HandEvaluatorState::NotStraightOrFlush
                } else if is_flush(&h, g) {
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
                if is_quads(&mut h) {
                    if 5 == h.len() && h[0].rank() == h[4].rank() {
                        return HandDescriptionBuilder::new(&h, g)
                            .with_level(HandLevel::FiveOfAKind)
                            .with_default_value().complete();
                    }
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
/// let cmp = ojp_reference_evaluator_quick(&hand, Scale::HighHand);
/// // assert_eq!(8969227, cmp);
/// ```
pub fn ojp_reference_evaluator_quick(hand: &Hand, g: Scale)
-> HandValue {
    let mut h = *hand;
    debug_assert!(h.len() <= g.complete_hand());
    debug_assert!(h.deck_type().low_aces() == g.low_aces());

    if h.is_empty() {
        return HAND_VALUE_WORST;
    }
    oj_sort(&mut h[..]);
    let mut state = HandEvaluatorState::Initial;

    loop {
        match state {
            HandEvaluatorState::Initial => {
                state =
                if ! g.straights_and_flushes() {
                    HandEvaluatorState::NotStraightOrFlush
                } else if is_flush(&h, g) {
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
                    if 5 == h.len() && h[0].rank() == h[4].rank() {
                        return ojp_default_hand_value(&h, g, HandLevel::FiveOfAKind);
                    }
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
