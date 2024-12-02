//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Reference_Evaluators) | Standard evaluators

use crate::prelude::*;
use crate::utils::oj_sort;
use crate::cards::hashes::ojh_positional_32cs;
use crate::poker::*;

/// Should be enough to leave room for all rank combinations between levels
pub const HAND_LEVEL_MULTIPLIER: u32 = 0x100000;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_hand_value) | Default hand value calculator
pub fn ojp_default_hand_value(h: &Hand, g: Scale, l: HandLevel) -> HandValue {
    let max = if h.len() > g.complete_hand() {
        g.complete_hand()
    } else {
        h.len()
    };
    let h: u32 = ojh_positional_32cs(&h[..max])
        .expect("already checked");

    HAND_LEVEL_MULTIPLIER * g.value_from_level(l)
    + if g.low_hands() {
        h
    } else {
        h ^ 0xFFFFF
    }
}

/// Return numeric value to compare hands.
pub type HandEvaluator = fn(&Hand) -> HandValue;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_best_value_of) | Best n-card hand value from larger set
///
/// Given a large hand and evaluator, find the best n-card hand value.
pub fn ojp_best_of(h: &Hand, k: usize, g: Scale, eval: HandEvaluator)
-> HandValue {
    let mut best = HAND_VALUE_WORST;
    debug_assert!(k >= g.complete_hand());
    debug_assert!(h.len() > k);

    for sub in h.combinations(k) {
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
        for i in 1..h.len() {
            if MEXICAN_RANK_ORDER[h[i].rank() as usize] + 1 !=
                MEXICAN_RANK_ORDER[h[i - 1].rank() as usize] {

                return false;
            }
        }
    } else {
        for i in 1..h.len() {
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

        h[..].swap(0, 4);
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

        h[..].swap(0, 3);
        h[..].swap(1, 4);
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

        h[..].swap(0, 3);
        return true;
    }
    if h.len() < 5 {
        return false;
    }
    // BCAAA
    if h[2].rank() == h[3].rank() &&
        h[2].rank() == h[4].rank() {

        h[..].swap(0, 3);
        h[..].swap(1, 4);
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

        h[..].swap(2, 4);
        return true;
    }
    // CAABB
    if h[1].rank() == h[2].rank() &&
        h[3].rank() == h[4].rank() {

        h[..].swap(0, 2);
        h[..].swap(2, 4);
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
        h[..].swap(0, 2);
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
        h[..].swap(2, 4);
        h[..].swap(1, 3);
        h[..].swap(0, 2);
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_default_eval_quick) | Default quick hand evaluator
///
/// This function is the default general-purpose evaluator for hands up to
/// the "complete hand" size of the given scale (usually 5 cards). It returns
/// a comparator value with level and rank information packed into a single
/// 32-bit integer. The value can be compared directly: winning hands are
/// smaller numbers. Such comparisons between hands of different sizes are
/// not meaningful.
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_reference_evaluator};
///
/// let hand = Hand::new(DeckType::English).init(hand!("As","Js","Ts","7s","5s"));
/// let v = ojp_reference_evaluator(&hand, Scale::HighHand);
/// assert_eq!(0x50458A, v);
/// ```
pub fn ojp_reference_evaluator(hand: &Hand, g: Scale) -> HandValue {
    debug_assert!(hand.len() <= g.complete_hand());
    debug_assert!(hand.is_not_empty());

    let mut h = if hand.deck_type().low_aces() == g.low_aces() {
        *hand
    } else {
        hand.convert_decktype(g.deck_type())
    };
    oj_sort(&mut h[..]);
    let mut state = HandEvaluatorState::Initial;

    let mut loop_guard = 20;
    loop {
        loop_guard -= 1;
        assert!(loop_guard > 0);

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

// Return HAND_VALUE_WORST if not a badugi, or the 32-bit hand value
fn badugi_value(cards: &[Card]) -> u32 {
    let mut suits: u32 = 0;
    let mut ranks: u32 = 0;

    let mut v: u32 = 0;
    for c in cards {
        let s = c.suit() as u32;
        let r = c.rank() as u32;

        if 0 != (suits & (1 << s)) || 0 != (ranks & (1 << r)) {
            return HAND_VALUE_WORST;
        }
        suits |= 1 << s;
        ranks |= 1 << r;

        v <<= 4;
        v += c.rank() as u32;
    }
    v + (5 - cards.len() as u32) * HAND_LEVEL_MULTIPLIER
}

enum BadugiEvaluatorState {
    Initial,
    NotFour,
    NotThree,
    NotTwo
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_reference_evaluator) | Badugi/Badeucy evaluator
///
/// Will produce Badugi values if given a hand with low aces, or Badeucy values
/// given a hand with high aces.
pub fn ojp_badugi_reference_evaluator(hand: &Hand) -> HandValue {
    let mut h = *hand;
    oj_sort(&mut h[..]);
    let mut state = BadugiEvaluatorState::Initial;

    let mut loop_guard = 10;
    loop {
        loop_guard -= 1;
        assert!(loop_guard > 0);

        match state {
            BadugiEvaluatorState::Initial => {
                if h.len() < 4 {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                let v= badugi_value(&h[..]);
                if HAND_VALUE_WORST == v {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                return v;
            },
            BadugiEvaluatorState::NotFour => {
                if h.len() < 3 {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h3 in h.combinations(3) {
                    v = badugi_value(&h3[..]);
                    if v < best_value {
                        best_value = v;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                return best_value;
            },
            BadugiEvaluatorState::NotThree => {
                if h.len() < 2 {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h2 in h.combinations(2) {
                    v = badugi_value(&h2[..]);
                    if v < best_value {
                        best_value = v;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                return best_value;
            },
            BadugiEvaluatorState::NotTwo => {
                let mut least = 0;

                for i in 1..h.len() {
                    if h[i].rank() < h[least].rank() {
                        least = i;
                    }
                }
                return Scale::Badugi.value_from_level(HandLevel::OneCard) *
                    HAND_LEVEL_MULTIPLIER + h[least].rank() as u32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::error::Result;

    #[test]
    fn test_hand_evaluator() -> Result<()> {
        Ok(())
    }
}
