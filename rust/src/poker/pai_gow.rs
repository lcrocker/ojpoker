//! [wiki](https://github.com/lcrocker/ojpoker/wiki/PaiGow) | Pai Gow poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full english name of hand e.g. "king-high straight"
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_full_name) | Full english name of hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::OneJoker).init(cards!("5s","As","2d","4s","3h"));
/// let v = ojp_pai_gow_eval_full(&hand).unwrap();
/// println!("{}", ojp_pai_gow_full_name(&v));
/// // Output: "ace-five-high straight"
/// ```
pub fn ojp_pai_gow_full_name(v: &HandValue) -> String {
    macro_rules! sng {
        ($x:literal) => { v.hand[$x as usize].rank().name() }
    }

    match HandLevel::from_u8(v.level) {
        HandLevel::StraightFlush => {
            if v.hand[0].rank() == Rank::Ace {
                if v.hand[1].rank() == Rank::King {
                    String::from("royal flush")
                } else {
                    debug_assert!(v.hand[1].rank() == Rank::Five);
                    format!("{}-{}-high straight flush", sng!(0), sng!(1))
                }
            } else {
                format!("{}-high straight flush", sng!(0))
            }
        },
        HandLevel::Straight => {
            if v.hand[0].rank() == Rank::Ace {
                format!("{}-{}-high straight", sng!(0), sng!(1))
            } else {
                format!("{}-high straight", sng!(0))
            }
        },
        _ => ojp_high_full_name(v),
    }
}

/*
 * TODO
 *
 * Pai Gow evaluators do not handle the bug. These functions will
 * eventually be called by the functions that do handle the bug.
 */

#[cfg(feature = "high-hand-tables")]
// Change high hand equivalence class into pai gow equivalence class.
// Adjust all values by one for A-A-A-A-Jk, the move the wheel and
// steel wheel up by seven spots.
fn pai_gow_adjust_ec(ec: u32) -> u32 {
    if ec == 10 {
        3
    } else if ec == 1609 {
        1602
    } else if ec > 1 && ec < 10 {
        ec + 2
    } else if ec > 1600 && ec < 1609 {
        ec + 2
    } else {
        ec + 1
    }
}

#[cfg(feature = "high-hand-tables")]
/// Full pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_full) | Full pai gow poker hand evaluator
pub fn ojp_pai_gow_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_high(h)
    } else {
        ojp_best_value_of(h, HandScale::HighHand, lookup_high)
    };
    let vv = HIGH_HAND_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::HighHand,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    v.value = pai_gow_adjust_ec(v.value);
    Ok(v)
}

#[cfg(feature = "high-hand-tables")]
/// Value-only pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_quick) | Value-only pai gow poker hand evaluator
pub fn ojp_pai_gow_eval_quick(h: &Hand) -> u32 {
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    let mut ec =
    if 5 == h.len() {
        lookup_high(h)
    } else {
        ojp_best_value_of(h, HandScale::HighHand, lookup_high)
    };
    pai_gow_adjust_ec(ec)
}

#[cfg(not(feature = "high-hand-tables"))]
fn curried_evaluator_pai_gow_full(h: &Hand) -> Result<HandValue, OjError> {
    let Some(repl) = ojp_bug_replace_pai_gow(h) else {
        return ojp_default_eval_full(h, HandScale::PaiGow);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    let mut v = ojp_default_eval_full(&dup, HandScale::PaiGow)?;
    v.bug_is = repl.replacement;
    Ok(v)
}

#[cfg(not(feature = "high-hand-tables"))]
fn curried_evaluator_pai_gow_quick(h: &Hand) -> u32 {
    let Some(repl) = ojp_bug_replace_pai_gow(h) else {
        return ojp_default_eval_quick(h, HandScale::PaiGow);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    ojp_default_eval_quick(&dup, HandScale::PaiGow)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Full pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_full) | Full pai gow poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let v = ojp_pai_gow_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_pai_gow_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::HighHand,
            curried_evaluator_pai_gow_full);
    }
    curried_evaluator_pai_gow_full(h)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Value-only pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_quick) | Value-only pai gow poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let h2 = Hand::new(DeckType::English).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_pai_gow_eval_quick(&h1);
/// let v2 = ojp_pai_gow_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_pai_gow_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::HighHand,
            curried_evaluator_pai_gow_quick);
    }
    curried_evaluator_pai_gow_quick(h)
}

/// If there's a bug in the hand, figure out what card it should
/// be, return it along with its index.
pub fn ojp_bug_replace_pai_gow(h: &Hand) -> Option<BugReplacement> {
    let scan = ojp_bug_scan(h);
    let index = scan.index?;

    if h.len() < 5 {    // partial hand, just ace
        return Some(BugReplacement::new(index as usize,
            ojp_ace_not_present(scan.ace_mask)));
    }
    let mut suit = Suit::None;
    let mut rank = Rank::None;

    if let Some(s) = FLUSH_PATTERNS.get(&scan.suit_mask) {
        suit = Suit::from_u8(*s);
    };
    if let Some(r) = PAI_GOW_STRAIGHT_PATTERNS.get(&scan.rank_mask) {
        rank = Rank::from_u8(*r);
    };
    Some(ojp_bug_replacement(rank, suit, &scan))
}

use lazy_static::lazy_static;

lazy_static! {
    /// After setting a bit for each rank in the hand, these are the patterns
    /// that indicate the rank needed for the bug to complete a straight.
    pub static ref PAI_GOW_STRAIGHT_PATTERNS: std::collections::HashMap<u16, u8> = {
        let mut m = std::collections::HashMap::new();
        m.insert(0b0000000000111101, 15);   // Nevada high-wheel rule
        m.insert(0b0000000010111001, 6);
        m.insert(0b0000000110110001, 6);
        m.insert(0b0000001110100001, 6);
        m.insert(0b0000000001111001, 7);
        m.insert(0b0000000101110001, 7);
        m.insert(0b0000001101100001, 7);
        m.insert(0b0000011101000001, 7);
        m.insert(0b0000000011110001, 8);
        m.insert(0b0000001011100001, 8);
        m.insert(0b0000011011000001, 8);
        m.insert(0b0000111010000001, 8);
        m.insert(0b0000000111100001, 9);
        m.insert(0b0000010111000001, 9);
        m.insert(0b0000110110000001, 9);
        m.insert(0b0010110100000001, 9);
        m.insert(0b0000001111000001, 10);
        m.insert(0b0000101110000001, 10);
        m.insert(0b0010101100000001, 10);
        m.insert(0b0110101000000001, 10);
        m.insert(0b1110100000000001, 10);
        m.insert(0b0000011110000001, 11);
        m.insert(0b0010011100000001, 11);
        m.insert(0b0110011000000001, 11);
        m.insert(0b1110010000000001, 11);
        m.insert(0b0000111100000001, 13);
        m.insert(0b0100111000000001, 13);
        m.insert(0b1100110000000001, 13);
        m.insert(0b0010111000000001, 14);
        m.insert(0b1010110000000001, 14);
        m.insert(0b0110110000000001, 15);
        m
    };
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_pai_gow() -> Result<(), OjError> {
        let deck = Deck::new_by_name("onejoker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(cards!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9d","3d","Qc","Ac","Th"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("Qc","9s","As","Td","3h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        hand.set(cards!("9s","Jk","Td","3h","Qc"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        best = v1.value;

        hand.set(cards!("6h","2d","9c","6d","Ts"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4h","8c","8d","Ad","4c"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);

        hand.set(cards!("8c","4h","8d","4c","Jk"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","7d","5c","5s","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","9h","8d","Ts","6s"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9c","7d","Tc","6c","8h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(cards!("9c","Jk","Tc","6c","8h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","5s","3s","4s","2d"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("5s","3s","2s","Jk","4d"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","As","Js","Th","Qh"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","Ad","8d","4d","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert_eq!(v1.level, HandLevel::Flush as u8);

        hand.set(cards!("Ad","8d","Jk","5d","4d"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","As","7d","7h","Ah"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3c","3s","3d","3h","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","As","Jk","Kd","Ah"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(cards!("Qh","9h","Kh","Jk","Jh"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        hand.set(cards!("Ad","5d","3d","2d","4d"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qs","Ks","Ts","As","Js"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Jk","Ks","Ts","As","Js"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ad","Jk","Ah","As","Ac"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert!(v1.value < best);

        Ok(())
    }
}
