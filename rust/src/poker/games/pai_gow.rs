//! [wiki](https://github.com/lcrocker/ojpoker/wiki/PaiGow) | Pai Gow poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full english name of hand e.g. "king-high straight"
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_full_name) | Full english name of hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::OneJoker).init(hand!("5s","As","2d","4s","3h"));
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

#[cfg(feature = "high-hand-tables")]
// Change high hand equivalence class into pai gow equivalence class
// by moving the wheels up 8 spots.
fn pai_gow_adjust_ec(ec: u32) -> u32 {
    match ec {
        ..15 => ec,
        15..23 => ec + 1,
        23 => 15,
        24..1614 => ec,
        1614..1622 => ec + 1,
        1622 => 1614,
        1623.. => ec,
    }
}

#[cfg(feature = "high-hand-tables")]
/// Quick lookup table evaluator
fn lookup_high(h: &Hand) -> u32 {
    let h = ojh_mp5_english(h.as_slice());
    HIGH_HAND_TABLE_1[h as usize] as u32
}

#[cfg(feature = "high-hand-tables")]
/// Full pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_full) | Full pai gow poker hand evaluator
pub fn ojp_pai_gow_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_pai_gow_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_high(h)
    } else {
        ojp_best_value_of(h, Scale::PaiGow, lookup_high)
    };
    let vv = HIGH_HAND_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, Scale::HighHand,
        vv.0, pai_gow_adjust_ec(ec));
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "high-hand-tables")]
/// Value-only pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_quick) | Value-only pai gow poker hand evaluator
pub fn ojp_pai_gow_eval_quick(h: &Hand) -> u32 {
    if h.len() < 5 {
        return curried_evaluator_pai_gow_quick(h);
    }
    let mut ec =
    if 5 == h.len() {
        lookup_high(h)
    } else {
        ojp_best_value_of(h, Scale::HighHand, lookup_high)
    };
    pai_gow_adjust_ec(ec)
}

#[cfg(not(feature = "high-hand-tables"))]
fn curried_evaluator_pai_gow_full(h: &Hand) -> Result<HandValue, OjError> {
    let Some(repl) = ojp_bug_replace_pai_gow(h) else {
        return ojp_default_eval_full(h, Scale::PaiGow);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    let mut v = ojp_default_eval_full(&dup, Scale::PaiGow)?;
    v.bug_is = repl.replacement;
    Ok(v)
}

#[cfg(not(feature = "high-hand-tables"))]
fn curried_evaluator_pai_gow_quick(h: &Hand) -> u32 {
    let Some(repl) = ojp_bug_replace_pai_gow(h) else {
        return ojp_default_eval_quick(h, Scale::PaiGow);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    ojp_default_eval_quick(&dup, Scale::PaiGow)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Full pai gow poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_pai_gow_eval_full) | Full pai gow poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
/// let v = ojp_pai_gow_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_pai_gow_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, Scale::HighHand,
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
/// let h1 = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
/// let h2 = Hand::new(DeckType::English).init(hand!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_pai_gow_eval_quick(&h1);
/// let v2 = ojp_pai_gow_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_pai_gow_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, Scale::HighHand,
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

        hand.set(hand!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(hand!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("9d","3d","Qc","Ac","Th"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(hand!("Qc","9s","As","Td","3h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        hand.set(hand!("9s","Jk","Td","3h","Qc"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        best = v1.value;

        hand.set(hand!("6h","2d","9c","6d","Ts"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4h","8c","8d","Ad","4c"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);

        hand.set(hand!("8c","4h","8d","4c","Jk"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5h","7d","5c","5s","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7d","9h","8d","Ts","6s"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(hand!("9c","7d","Tc","6c","8h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(hand!("9c","Jk","Tc","6c","8h"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ah","5s","3s","4s","2d"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(hand!("5s","3s","2s","Jk","4d"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Kd","As","Js","Th","Qh"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5d","Ad","8d","4d","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert_eq!(v1.level, HandLevel::Flush as u8);

        hand.set(hand!("Ad","8d","Jk","5d","4d"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ac","As","7d","7h","Ah"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("3c","3s","3d","3h","Kd"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ac","As","Jk","Kd","Ah"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(hand!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(hand!("Qh","9h","Kh","Jk","Jh"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        hand.set(hand!("Ad","5d","3d","2d","4d"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Qs","Ks","Ts","As","Js"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(hand!("Jk","Ks","Ts","As","Js"));
        v2 = ojp_pai_gow_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ad","Jk","Ah","As","Ac"));
        v1 = ojp_pai_gow_eval_full(&hand)?;
        assert!(v1.value < best);

        Ok(())
    }
}
