//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToSix) | Ace-to-six "London" low poker hands

use crate::cards::*;
use crate::poker::*;

#[cfg(feature = "deuce-to-seven-tables")]
use crate::poker::tables::deuce_to_seven_tables::*;

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ll_description) | Ace-to-six low hand description
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_ll_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToSix, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ll_description) | Ace-to-six low hand description
#[cfg(feature = "deuce-to-seven-tables")]
pub fn ojp_ll_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::AceToSix,
        OJP_LL_TABLE_2[v as usize])
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ll_eval_5) | Ace-to-six low hand evaluator
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_ll_eval_5(h: &Hand) -> HandValue {
    ojp_reference_evaluator(h, Scale::AceToSix)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_ll_eval_5) | Ace-to-six low hand evaluator
#[cfg(feature = "deuce-to-seven-tables")]
pub fn ojp_ll_eval_5(h: &Hand) -> HandValue {
    let h = ojh_bitfield_mp5_low(&h[..5]);
    OJP_LL_TABLE_1[h as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_value) | Ace-to-six evaluator
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_ll_value};
///
/// let h1 = Hand::new(DeckType::Low).init(hand!("Qs","Qs","5c","Qh","6c"));
/// let h2 = Hand::new(DeckType::Low).init(hand!("9s","6s","7c","5h","8c"));
/// let v1 = ojp_ll_value(&h1);
/// let v2 = ojp_ll_value(&h2);
/// assert!(v1 < v2);   // trips beats straight
/// ```
pub fn ojp_ll_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::AceToSix.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::AceToSix),
        5 => ojp_ll_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::AceToSix, ojp_ll_eval_5),
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
    fn test_hand_evaluator_ll() -> Result<()> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("8c","Jc","9c","Qc","Tc"));
        let mut v1 = ojp_ll_value(&hand);
        let mut d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);

        hand.set(hand!("Td","Qd","8d","9d","Jd"));
        let mut v2 = ojp_ll_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","As","5s","3s","2s"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6c","Tc","6s","6d","6h"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Qc","Qd","6d","6c","Qh"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6c","6h","Qc","6d","Qs"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Qs","As","Ts","Ks","Js"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5h","Jh","6h","4h","9h"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Qd","9s","8d","Ts","Jh"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);

        hand.set(hand!("9h","Qd","Ts","Jc","8h"));
        v2 = ojp_ll_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2d","Ah","5s","3s","4s"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3h","5s","3c","3d","9c"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","Js","Ks","Jd","4c"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6h","2s","9c","6s","8h"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Qd","Kh","Ac","Jc","Td"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("8d","3d","4c","Kc","Th"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("4c","8s","Ks","Td","3h"));
        v2 = ojp_ll_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5h","2s","3h","7s","4h"));
        v1 = ojp_ll_value(&hand);
        d1 = ojp_ll_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3s","4h","6d","2d","Ad"));
        v1 = ojp_ll_value(&hand);
        assert!(v1 < best);

        Ok(())
    }

    #[cfg(feature = "deuce-to-seven-tables")]
    #[test]
    fn test_ll_tables() -> Result<()> {
        use crate::utils::Random;

        fn curried_evaluator(h: &Hand) -> HandValue {
            ojp_reference_evaluator(h, Scale::AceToSix)
        }
        fn ref_val(h: &Hand) -> HandValue {
            if h.len() <= 5 {
                return ojp_reference_evaluator(h, Scale::AceToSix);
            }
            ojp_best_of(h, 5, Scale::AceToSix, curried_evaluator)
        }

        let mut deck = Deck::new_by_name("low");
        let mut rng = Random::new();

        for _ in 0..1000 {
            deck.refill_and_shuffle();
            let len = 1 + rng.uniform16(4) +
                rng.uniform16(4) + rng.uniform16(4);
            let hand1 = deck.new_hand().init(deck.draw(len));
            let hand2 = deck.new_hand().init(deck.draw(len));

            let vt1 = ojp_ll_value(&hand1);
            let vr1 = ref_val(&hand1);
            let vt2 = ojp_ll_value(&hand2);
            let vr2 = ref_val(&hand2);

            if vt1 < vt2 {
                assert!(vr1 < vr2);
            } else if vt1 > vt2 {
                assert!(vr1 > vr2);
            } else {
                assert_eq!(vr1, vr2);
            }
            if 5 == len {
                assert_eq!(OJP_LL_TABLE_2[vt1 as usize], vr1);
            }
        }
        Ok(())
    }
}
