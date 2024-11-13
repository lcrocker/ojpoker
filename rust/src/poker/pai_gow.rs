//! [wiki](https://github.com/lcrocker/ojpoker/wiki/PaiGow) | Pai Gow poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full english name of hand e.g. "king-high straight"
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_full_name) | Full english name of hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("5s","As","2d","4s","3h"));
/// let v = ojp_pai_gow_eval_full_no_bug(&hand).unwrap();
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
pub fn ojp_pai_gow_eval_full_no_bug(h: &Hand) -> Result<HandValue, OjError> {
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
pub fn ojp_pai_gow_eval_quick_no_bug(h: &Hand) -> u32 {
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
fn curried_evaluator_pai_gow_full_no_bug(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::PaiGow)
}

#[cfg(not(feature = "high-hand-tables"))]
fn curried_evaluator_pai_gow_quick_no_bug(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::PaiGow)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Full pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_full) | Full pai gow poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let v = ojp_pai_gow_eval_full_no_bug(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_pai_gow_eval_full_no_bug(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::HighHand,
            curried_evaluator_pai_gow_full_no_bug);
    }
    curried_evaluator_pai_gow_full_no_bug(h)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Value-only pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_quick) | Value-only pai gow poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let h2 = Hand::new(DeckType::English).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_pai_gow_eval_quick_no_bug(&h1);
/// let v2 = ojp_pai_gow_eval_quick_no_bug(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_pai_gow_eval_quick_no_bug(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::HighHand,
            curried_evaluator_pai_gow_quick_no_bug);
    }
    curried_evaluator_pai_gow_quick_no_bug(h)
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_pai_gow() -> Result<(), OjError> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFF_FFFF;

        hand.set(cards!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9d","3d","Qc","Kc","Th"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("Qc","9s","Ks","Td","3h"));
        v2 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6h","2d","9c","6d","Ts"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4h","8c","8d","Ad","4c"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","7d","5c","5s","Kd"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","9h","8d","Ts","6s"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9c","7d","Tc","6c","8h"));
        v2 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","5s","3s","4s","2d"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","As","Js","Th","Qh"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","Td","8d","4d","Qd"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","As","7d","7h","Ah"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3c","3s","3d","3h","Kd"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        hand.set(cards!("Ad","5d","3d","2d","4d"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qs","Ks","Ts","As","Js"));
        v1 = ojp_pai_gow_eval_full_no_bug(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);

        Ok(())
    }
}
