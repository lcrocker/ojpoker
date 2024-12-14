//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToFive) | Ace-to-five low hands

use crate::cards::*;
use crate::poker::*;

#[cfg(feature = "ace-to-five-tables")]
use crate::poker::tables::ace_to_five_tables::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_full_name) | Describe ace-to-five hand
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::games::ace_to_five::*;
///
/// let hand = Hand::new(DeckType::Low).init(hand!("5s","9s","As","Ks","Js"));
/// let v = ojp_a5_value(&hand);
/// let d = ojp_a5_description(&hand, v);
/// println!("{}", ojp_a5_full_text(&d));
/// // Output: "king, jack, nine, five, ace" (no flush, ace is low)
/// ```
pub fn ojp_a5_full_text(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    debug_assert!(d.level != HandLevel::StraightFlush);
    debug_assert!(d.level != HandLevel::Flush);
    debug_assert!(d.level != HandLevel::Straight);

    match d.level {
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_hh_full_text(d)
    }
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_description) | Ace-to-five low hand description
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_a5_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToFive, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_description) | Ace-to-five low hand description
#[cfg(feature = "ace-to-five-tables")]
pub fn ojp_a5_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToFive,
        OJP_A5_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_5) | Ace-to-five 5-card evaluator
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_a5_eval_5(h: &Hand) -> HandValue {
    ojp_reference_evaluator(h, Scale::AceToFive)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_5) | Ace-to-five 5-card evaluator
#[cfg(feature = "ace-to-five-tables")]
pub fn  ojp_a5_eval_5(h: &Hand) -> HandValue {
    let h = ojh_base13_mp5_low(&h[..]);
    OJP_A5_TABLE_1[h as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_7) | Ace-to-five 7-card evaluator
#[cfg(feature = "ace-to-five-tables")]
pub fn  ojp_a5_eval_7(hand: &Hand) -> HandValue {
    let h = ojh_base13_mp5_low(&hand[..5]);
    let v1 = OJP_A5_TABLE_1[h as usize] as usize;

    let mut r1 = hand[5].rank() as u8 - 1;
    if r1 > 10 { r1 -= 1; }
    let mut r2 = hand[6].rank() as u8 - 1;
    if r2 > 10 { r2 -= 1; }

    OJP_A5_TABLE_3[169 * (v1 - 1) + 13 * r1 as usize + r2 as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_a5_value) | Ace-to-five evaluator
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_a5_value};
///
/// let h1 = Hand::new(DeckType::Low).init(hand!("3s","9s","7d","5c","2c"));
/// let h2 = Hand::new(DeckType::Low).init(hand!("7s","4s","As","Qd","6c"));
/// let v1 = ojp_a5_value(&h1);
/// let v2 = ojp_a5_value(&h2);
/// assert!(v1 < v2);   // nine-high beats queen-high
/// ```
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_a5_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::AceToFive.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::AceToFive),
        5 => ojp_a5_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::AceToFive, ojp_a5_eval_5),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_a5_value) | Ace-to-five evaluator
#[cfg(feature = "ace-to-five-tables")]
pub fn ojp_a5_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::AceToFive.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::AceToFive),
        5 => ojp_a5_eval_5(h),
        6 => ojp_best_of(h, 5, Scale::AceToFive, ojp_a5_eval_5),
        7 => ojp_a5_eval_7(h),
        8.. => ojp_best_of(h, 7, Scale::AceToFive, ojp_a5_eval_7),
    }
}

/*
 * ACE-TO-FIVE HANDS WITH BUG ("California Lowball")
 */

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_cl_full_text) | Full English text of hand
pub fn ojp_cl_full_text(d: &HandDescription) -> String {
    ojp_a5_full_text(d)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_cl_description) | Describe ace-to-five hand with bug
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_cl_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToFiveBug, v)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_cl_description) | Describe ace-to-five hand with bug
#[cfg(feature = "ace-to-five-tables")]
pub fn ojp_cl_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToFiveBug,
        OJP_A5_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_cl_eval_5) | Ace-to-five with bug 5-card evaluator
pub fn ojp_cl_eval_5(h: &Hand) -> HandValue {
    let Some(sr) = ojp_bug_scan_5_1(h, Scale::AceToFiveBug) else {
        return ojp_a5_eval_5(h)
    };
    let mut bh = *h;
    bh[sr.index as usize] = sr.replacement;
    ojp_a5_eval_5(&bh)
}
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_cl_value) | Ace-to-five with bug evaluator
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_cl_value, ojp_cl_description, ojp_cl_full_text};
///
/// let hand = Hand::new(DeckType::LowJoker).init(hand!("7s","4s","As","5d","Jk"));
/// let v = ojp_cl_value(&hand);
/// let d = ojp_cl_description(&hand, v);
/// println!("[{}]: {}", d.hand, ojp_cl_full_text(&d));
/// // Output: "[7s5d4s2sAs]: seven, five, four, deuce, ace"
/// ```
pub fn ojp_cl_value(h: &Hand) -> HandValue {
    match h.len() {
        ..5 => {
            if let Some(sr) = ojp_bug_scan_p_1(h, Scale::AceToFiveBug) {
                let mut bh = *h;
                bh[sr.index as usize] = sr.replacement;
                ojp_a5_value(&bh)
            } else {
                ojp_a5_value(h)
            }
        },
        5 => ojp_cl_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::AceToFiveBug, ojp_cl_eval_5),
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;

    #[test]
    fn test_hand_evaluator_a5() -> Result<()> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("Ks","Kh","Kd","Kc","Qs"));
        let mut v1 = ojp_a5_value(&hand);
        let mut d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);

        hand.set(hand!("Kd","Qc","Kc","Kh","Ks"));
        let mut v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Jd","5c","Jc","Jh","5s"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","5c","Js","5h","Jc"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","5c","Js","5h","Jc","Jd"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("7d","4c","7s","7h","Kc"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2d","Tc","2s","3h","Ts"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2c","2d","Tc","Th","2s","3h","Ts"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("4s","3c","9d","9h","Qc"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("9c","9s","4s","3c","9d","9h","Qc"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ts","Js","Ks","9s","Qs"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kh","Td","9s","Qc","Jc","9h"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Kd","Ks","Td","9s","9c","Qd","Jc"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Kc","3d","9d","6h","2d"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6c","9c","3c","Kc","2c"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("6d","9c","3c","3h","Kd","3s","2s"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("5c","3d","4s","7s","2d"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","4h","3s","9d","4s","7s","2d"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ah","2c","4s","5d","3d"));
        v1 = ojp_a5_value(&hand);
        d1 = ojp_a5_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);

        hand.set(hand!("Qh","2c","Ad","4s","5d","7h","3h"));
        v2 = ojp_a5_value(&hand);
        assert_eq!(v1, v2);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_cl() -> Result<()> {
        let deck = Deck::new_by_name("lowjoker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("Ks","Kh","Kd","Kc","Qs"));
        let mut v1 = ojp_cl_value(&hand);
        let mut d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);

        hand.set(hand!("5d","5c","Js","5h","Jc"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("7d","4c","7s","7h","Kc"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2d","Tc","2s","3h","Ts"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","Ac","9d","9h","Qc"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);

        hand.set(hand!("4s","Jk","9d","9h","Qc"));
        let v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("9d","Qc","Jk","4s","9c"));
        let mut v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ts","Js","Ks","9s","Qs"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kh","Td","9s","Qc","Jc"));
        v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ac","3d","9d","6h","2d"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ac","3d","9d","6h","Jk"));
        v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("6c","9c","Jk","Ac","2c"));
        v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("5c","3d","4s","7s","2d"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ah","2c","4s","5d","3d"));
        v1 = ojp_cl_value(&hand);
        d1 = ojp_cl_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("Jk","2c","Ah","3s","4d"));
        v2 = ojp_cl_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);

        Ok(())
    }

    #[cfg(feature = "ace-to-five-tables")]
    #[test]
    fn test_a5_tables() -> Result<()> {
        use crate::utils::Random;

        fn curried_evaluator(h: &Hand) -> HandValue {
            ojp_reference_evaluator(h, Scale::AceToFive)
        }
        fn ref_val(h: &Hand) -> HandValue {
            if h.len() <= 5 {
                return ojp_reference_evaluator(h, Scale::AceToFive);
            }
            ojp_best_of(h, 5, Scale::AceToFive, curried_evaluator)
        }

        let mut deck = Deck::new_by_name("low");
        let mut rng = Random::new();

        for _ in 0..1000 {
            deck.refill_and_shuffle();
            let len = 1 + rng.uniform16(4) +
                rng.uniform16(4) + rng.uniform16(4);
            let hand1 = deck.new_hand().init(deck.draw(len));
            let hand2 = deck.new_hand().init(deck.draw(len));

            let vt1 = ojp_a5_value(&hand1);
            let vr1 = ref_val(&hand1);
            let vt2 = ojp_a5_value(&hand2);
            let vr2 = ref_val(&hand2);

            if vt1 < vt2 {
                assert!(vr1 < vr2);
            } else if vt1 > vt2 {
                assert!(vr1 > vr2);
            } else {
                assert_eq!(vr1, vr2);
            }
            if 5 == len {
                assert_eq!(OJP_A5_TABLE_2[vt1 as usize], vr1);
            }
        }
        Ok(())
    }
}
