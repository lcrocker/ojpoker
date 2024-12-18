//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HighHand) | Traditional "high" poker hands

use crate::cards::*;
use crate::poker::*;

#[cfg(feature = "high-hand-tables")]
use crate::poker::tables::high_tables::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_full_text) | Full english description of hand
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::games::high_hand::*;
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
/// let v = ojp_hh_value(&hand);
/// let d = ojp_hh_description(&hand, v);
/// println!("{}", ojp_hh_full_text(&d));
/// // Output: "aces and nines with a king"
/// ```
pub fn ojp_hh_full_text(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    macro_rules! plr {
        ($x:literal) => { d.hand[$x as usize].rank().plural() }
    }
    macro_rules! art {
        ($x:literal) => { d.hand[$x as usize].rank().article() }
    }

    match d.level {
        HandLevel::FiveOfAKind => {
            format!("five {}", plr!(0))
        },
        HandLevel::StraightFlush => {
            if d.hand[0].rank() == Rank::Ace {
                String::from("royal flush")
            } else {
                format!("{}-high straight flush", sng!(0))
            }
        },
        HandLevel::Quads => {
            format!("four {} with {} {}", plr!(0), art!(4), sng!(4))
        },
        HandLevel::FullHouse => {
            format!("{} full of {}", plr!(0), plr!(3))
        },
        HandLevel::Flush => {
            format!("flush: {}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        HandLevel::Straight => {
            format!("{}-high straight", sng!(0))
        },
        HandLevel::Trips => {
            format!("three {}, {}, {}", plr!(0), sng!(3), sng!(4))
        },
        HandLevel::TwoPair => {
            format!("{} and {} with {} {}", plr!(0), plr!(2), art!(4), sng!(4))
        },
        HandLevel::Pair => {
            format!("pair of {}, {}, {}, {}", plr!(0), sng!(2), sng!(3), sng!(4))
        },
        HandLevel::NoPair => {
            format!("no pair: {}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => String::from("unknown hand"),
    }
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_hh_description) | High hand description
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_hh_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::HighHand, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_high_description) | High hand description
#[cfg(feature = "high-hand-tables")]
pub fn ojp_hh_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::HighHand,
        OJP_HH_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_eval_5) | 5-card high poker hand evaluator
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_hh_eval_5(h: &Hand) -> HandValue {
    ojp_reference_evaluator(h, Scale::HighHand)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_value) | High poker hand evaluator
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_hh_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::HighHand.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::HighHand),
        5 => ojp_hh_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::HighHand, ojp_hh_eval_5),
    }
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_eval_5) | 5-card high poker hand evaluator
pub fn ojp_hh_eval_5(hand: &Hand) -> HandValue {
    let hash = ojh_bitfield_mp5_english(&hand[..5]);
    OJP_HH_TABLE_1[hash as usize] as HandValue
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_eval_7) | 7-card high poker hand evaluator
pub fn ojp_hh_eval_7(hand: &Hand) -> HandValue {
    let hash = ojh_bitfield_mp7_english(&hand[..7]);
    OJP_HH_TABLE_3[hash as usize] as HandValue
}

/// High poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hh_value) | High poker hand evaluator
#[cfg(feature = "high-hand-tables")]
pub fn ojp_hh_value(h: &Hand) -> HandValue {
    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::HighHand),
        5 => ojp_hh_eval_5(h),
        6 => ojp_best_of(h, 5, Scale::HighHand, ojp_hh_eval_5),
        7 => ojp_hh_eval_7(h),
        8.. => ojp_best_of(h, 7, Scale::HighHand, ojp_hh_eval_7),
    }
}

/*
 * HIGH HANDS WITH BUG
 */

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hb_full_text) | Full description of high hand with bug
pub fn ojp_hb_full_text(d: &HandDescription) -> String {
    ojp_hh_full_text(d)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_hb_description) | High hand with bug description
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_hb_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::HighHandBug, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_high_bug_description) | High hand with bug description
#[cfg(feature = "high-hand-tables")]
pub fn ojp_hb_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::HighHandBug,
        OJP_HH_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hb_eval_5) | 5-card high evaluator with bug
pub fn ojp_hb_eval_5(h: &Hand) -> HandValue {
    let Some(sr) = ojp_bug_scan_5_1(h, Scale::HighHandBug) else {
        return ojp_hh_eval_5(h);
    };
    let mut bh = *h;
    bh[sr.index as usize] = sr.replacement;
    ojp_hh_eval_5(&bh)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_hb_value) | High hand with bug evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::OneJoker).init(hand!("9s","As","9d","Ks","Jk"));
/// let h2 = Hand::new(DeckType::OneJoker).init(hand!("9c","Ac","9h","Td","Ad"));
/// let v1 = Scale::HighHandBug.value(&h1);
/// let v2 = Scale::HighHandBug.value(&h2);
/// assert!(v1 < v2);
/// ```
pub fn ojp_hb_value(h: &Hand) -> u32 {
    match h.len() {
        ..5 => {
            if let Some(sr) = ojp_bug_scan_p_1(h, Scale::HighHandBug) {
                let mut bh = *h;
                bh[sr.index as usize] = sr.replacement;
                return ojp_hh_value(&bh);
            }
            ojp_hh_value(h)
        },
        5 => ojp_hb_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::HighHandBug, ojp_hb_eval_5),
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
    fn test_hand_evaluator_hh() -> Result<()> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best = HAND_VALUE_WORST;

        hand.set(hand!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_hh_value(&hand);
        let mut d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_hh_value(&hand);
        let d2 = ojp_hh_description(&hand, v2);
        assert_eq!(v1, v2);
        assert_eq!(d1, d2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("9d","3d","Qc","Kc","Th"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("Qc","9s","Ks","Td","3h"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6h","2d","9c","6d","Ts"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4h","8c","8d","Ad","4c"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","Qd","8s","7s","8d","Ad","4c"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("5h","7d","5c","5s","Kd"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4h","7d","5c","5s","3d","Ks","5h"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ah","5s","3s","4s","2d"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert_eq!(d1.hand[0].rank(), Rank::Five);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ah","As","3s","5s","Ac","4s","2d"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("7d","9h","8d","Ts","6s"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);

        hand.set(hand!("9c","7d","Tc","6c","8h"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5s","5c","7d","Tc","9d","6c","8h"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Kd","As","Js","Th","Qh"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kd","Ad","Ks","Js","Qd","Th","Qh"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("5d","Td","8d","4d","Qd"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","Td","3d","8d","Qs","4d","Qd"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ad","7h","Ac","Kc","Kh","7d","7c"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ac","As","7d","7h","Ah"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3c","3s","3d","3h","Kd"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ad","5d","3d","2d","4d"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ad","5d","Kd","5c","3d","2d","4d"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_hh_value(&hand);
        d1 = ojp_hh_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);

        hand.set(hand!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);

        hand.set(hand!("9s","Ks","Ts","Qs","Ad","8c","Js"));
        v2 = ojp_hh_value(&hand);
        assert_eq!(v1, v2);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_hb() -> Result<()> {
        let deck = Deck::new_by_name("onejoker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("9d","3d","Qc","As","Th"));
        let mut v1 = ojp_hb_value(&hand);
        let mut d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("Qc","9s","Jk","Td","3h"));
        let mut v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6h","2d","9c","6d","Jk"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4h","8c","8d","Jk","4c"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","Ks","Td","8c","8d","Jk","4c"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("5h","Jk","5c","5s","Kd"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5h","Ac","9h","6d","5c","5s","Ks"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ah","5s","Jk","4s","2d"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ah","5s","As","3c","Js","4s","2d"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Jk","9h","8d","Ts","6s"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);

        hand.set(hand!("9c","7d","Jk","6c","8h"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("9c","7d","Jk","6c","8h","As","5d"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Kd","Jk","Js","Th","Qh"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","Td","8d","Ad","Jk"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5d","Td","8d","Ad","Kd","Th","3d"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("7s","7h","Jk","7d","Ad"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ac","Jk","7d","7h","Ah"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3c","3s","3d","3h","Jk"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ad","5d","3d","Jk","4d"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ts","Qs","9s","Jk","Ks"));
        v1 = ojp_hb_value(&hand);
        d1 = ojp_hb_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);

        hand.set(hand!("Qh","As","9h","Jk","Ad","Th","Jh"));
        v2 = ojp_hb_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);

        Ok(())
    }

    #[cfg(feature = "high-hand-tables")]
    #[test]
    fn test_tables() -> Result<()> {
        use crate::utils::Random;

        fn curried_evaluator(h: &Hand) -> HandValue {
            ojp_reference_evaluator(h, Scale::HighHand)
        }
        fn ref_val(h: &Hand) -> HandValue {
            if h.len() <= 5 {
                return ojp_reference_evaluator(h, Scale::HighHand);
            }
            ojp_best_of(h, 5, Scale::HighHand, curried_evaluator)
        }

        let mut deck = Deck::new_by_name("english");
        let mut rng = Random::new();

        for _ in 0..1000 {
            deck.refill_and_shuffle();
            let len = 1 + rng.uniform16(4) +
                rng.uniform16(4) + rng.uniform16(4);
            let hand1 = deck.new_hand().init(deck.draw(len));
            let hand2 = deck.new_hand().init(deck.draw(len));

            let vt1 = ojp_hh_value(&hand1);
            let vr1 = ref_val(&hand1);
            let vt2 = ojp_hh_value(&hand2);
            let vr2 = ref_val(&hand2);

            if vt1 < vt2 {
                assert!(vr1 < vr2);
            } else if vt1 > vt2 {
                assert!(vr1 > vr2);
            } else {
                assert_eq!(vr1, vr2);
            }
            if 5 == len {
                assert_eq!(OJP_HH_TABLE_2[vt1 as usize], vr1);
            }
        }
        Ok(())
    }
}
