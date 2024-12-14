//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Badugi) | Badugi hand values

use crate::cards::*;
use crate::poker::*;

#[cfg(feature = "badugi-tables")]
use crate::poker::tables::badugi_tables::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bg_full_text) | Describe Badugi hand
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::Low).init(hand!("9s","7c","5h","8d"));
/// let v = Scale::Badugi.value(&hand);
/// let d = Scale::Badugi.description(&hand, v);
/// println!("{}", d.full_text());
/// // Output: "nine, eight, seven, five"
/// ```
pub fn ojp_bg_full_text(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    match d.level {
        HandLevel::FourCard => {
            if Rank::Four == d.hand[0].rank() {
                String::from("perfect badugi")
            } else {
                format!("{}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3))
            }
        },
        HandLevel::ThreeCard => {
            format!("three-card {}, {}, {}", sng!(0), sng!(1), sng!(2))
        },
        HandLevel::TwoCard => {
            format!("two-card {}, {}", sng!(0), sng!(1))
        },
        HandLevel::OneCard => {
            format!("one-card {}", sng!(0))
        },
        _ => String::from("unknown hand"),
    }
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_bg_description) | Badugi hand description
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_bg_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::Badugi, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_bc_description) | Badeucy hand description
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_bc_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::Badeucy, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_bg_description) | Badugi hand description
#[cfg(feature = "badugi-tables")]
pub fn ojp_bg_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::Badugi,
        OJP_BG_TABLE_2[v as usize])
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_bg_description) | Badeucy hand description
#[cfg(feature = "badugi-tables")]
pub fn ojp_bc_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::Badeucy,
        OJP_BC_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bg_eval_4) | 4-card Badugi evaluator
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_bg_eval_4(hand: &Hand) -> HandValue {
    ojp_badugi_reference_evaluator(hand)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bc_eval_4) | 4-card Badeucy evaluator
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_bc_eval_4(hand: &Hand) -> HandValue {
    ojp_badugi_reference_evaluator(hand)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bg_eval_4) | 4-card Badugi evaluator
#[cfg(feature = "badugi-tables")]
pub fn ojp_bg_eval_4(hand: &Hand) -> HandValue {
    let hash = ojh_bitfield_mp4_low(&hand[..]);
    OJP_BG_TABLE_1[hash as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bc_eval_4) | 4-card Badeucy evaluator
#[cfg(feature = "badugi-tables")]
pub fn ojp_bc_eval_4(hand: &Hand) -> HandValue {
    let hash = ojh_bitfield_mp4_english(&hand[..]);
    OJP_BC_TABLE_1[hash as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bg_value) | Badugi hand evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::Low).init(hand!("9s","7c","5h","8d"));
/// let h2 = Hand::new(DeckType::Low).init(hand!("2s","Ac","3h","4c"));
/// let v1 = Scale::Badugi.value(&h1);
/// let v2 = Scale::Badugi.value(&h2);
/// assert!(v1 < v2);   // four-card nine beats three-card trey
/// ```
pub fn ojp_bg_value(h: &Hand) -> u32 {
    match h.len() {
        ..4 => ojp_badugi_reference_evaluator(h),
        4 => ojp_bg_eval_4(h),
        5.. => ojp_best_of(h, 4, Scale::Badugi, ojp_bg_eval_4),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_bc_value) | Badeucy hand evaluator
pub fn ojp_bc_value(h: &Hand) -> u32 {
    match h.len() {
        ..4 => ojp_badugi_reference_evaluator(h),
        4 => ojp_bc_eval_4(h),
        5.. => ojp_best_of(h, 4, Scale::Badeucy, ojp_bc_eval_4),
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
    fn test_hand_evaluator_bg() -> Result<()> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand().init(hand!("Ks","Kh","Kd","Kc"));
        let mut best: u32 = HAND_VALUE_WORST;

        let mut v1 = ojp_bg_value(&hand);
        let mut d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::OneCard);
        assert_eq!(d1.hand[0].rank(), Rank::King);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ad","Ac","Ah","As"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::OneCard);
        assert_eq!(d1.hand[0].rank(), Rank::LowAce);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3d","9d","Ad","Kd"));
        let mut v2 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v2);
        assert_eq!(v1, v2);
        assert_eq!(d1.hand[0].rank(), Rank::LowAce);

        hand.set(hand!("Jd","5c","Jh","7c"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoCard);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("7d","Kd","5d","Jc","Kc"));
        v2 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v2);
        assert_eq!(v1, v2);
        assert_eq!(d1.hand[0].rank(), Rank::Jack);
        assert_eq!(d1.hand[1].rank(), Rank::Five);

        hand.set(hand!("7d","4c","7s","9c"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoCard);
        assert_eq!(d1.hand[0].rank(), Rank::Seven);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("7h","4d","7d","4c","4s","7c"));
        v2 = ojp_bg_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("2h","Tc","2s","5h"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::ThreeCard);
        assert_eq!(d1.hand[0].rank(), Rank::Ten);
        assert_eq!(d1.hand[1].rank(), Rank::Five);
        assert_eq!(d1.hand[2].rank(), Rank::Deuce);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("6h","Jc","Qs","2h","Tc","2s","5h"));
        v2 = ojp_bg_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("4s","3c","9d","9h"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::ThreeCard);
        assert_eq!(d1.hand[0].rank(), Rank::Nine);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert_eq!(d1.hand[2].rank(), Rank::Trey);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Tc","Jd","Kh","Qs"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::King);
        assert_eq!(d1.hand[1].rank(), Rank::Queen);
        assert_eq!(d1.hand[2].rank(), Rank::Jack);
        assert_eq!(d1.hand[3].rank(), Rank::Ten);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3c","2d","4s","5h"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::Five);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert_eq!(d1.hand[2].rank(), Rank::Trey);
        assert_eq!(d1.hand[3].rank(), Rank::Deuce);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2c","4d","6s","3c","2d","4s","5h"));
        v2 = ojp_bg_value(&hand);
        assert_eq!(v1, v2);

        hand.set(hand!("Ac","3d","4s","2h"));
        v1 = ojp_bg_value(&hand);
        d1 = ojp_bg_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::Four);
        assert_eq!(d1.hand[1].rank(), Rank::Trey);
        assert_eq!(d1.hand[2].rank(), Rank::Deuce);
        assert_eq!(d1.hand[3].rank(), Rank::LowAce);
        assert!(v1 < best);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_badeucy() -> Result<()> {
        let deck = Deck::new_by_name("english");
        let mut hand= deck.new_hand().init(hand!("As","Ah","Ad","Ac"));
        let mut best: u32 = HAND_VALUE_WORST;

        let mut v1 = ojp_bc_value(&hand);
        let mut d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::OneCard);
        assert_eq!(d1.hand[0].rank(), Rank::Ace);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kd","Kc","Kh","Ks"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::OneCard);
        assert_eq!(d1.hand[0].rank(), Rank::King);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3d","9d","Ad","Kd"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.hand[0].rank(), Rank::Trey);

        hand.set(hand!("Jd","5c","Jh","7c"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoCard);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kd","5d","Jc","Kc"));
        let v2 = ojp_bc_value(&hand);
        assert_eq!(v1, v2);
        d1 = ojp_bc_description(&hand, v2);
        assert_eq!(d1.hand[0].rank(), Rank::Jack);
        assert_eq!(d1.hand[1].rank(), Rank::Five);

        hand.set(hand!("7d","4c","7s","9c"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoCard);
        assert_eq!(d1.hand[0].rank(), Rank::Seven);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2h","Tc","2s","5h"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::ThreeCard);
        assert_eq!(d1.hand[0].rank(), Rank::Ten);
        assert_eq!(d1.hand[1].rank(), Rank::Five);
        assert_eq!(d1.hand[2].rank(), Rank::Deuce);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","3c","9d","9h"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::ThreeCard);
        assert_eq!(d1.hand[0].rank(), Rank::Nine);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert_eq!(d1.hand[2].rank(), Rank::Trey);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Ac","3d","4s","2h"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::Ace);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert_eq!(d1.hand[2].rank(), Rank::Trey);
        assert_eq!(d1.hand[3].rank(), Rank::Deuce);
        assert!(v1 < best);

        hand.set(hand!("Tc","Jd","Kh","Qs"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::King);
        assert_eq!(d1.hand[1].rank(), Rank::Queen);
        assert_eq!(d1.hand[2].rank(), Rank::Jack);
        assert_eq!(d1.hand[3].rank(), Rank::Ten);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3c","2d","4s","5h"));
        v1 = ojp_bc_value(&hand);
        d1 = ojp_bc_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FourCard);
        assert_eq!(d1.hand[0].rank(), Rank::Five);
        assert_eq!(d1.hand[1].rank(), Rank::Four);
        assert_eq!(d1.hand[2].rank(), Rank::Trey);
        assert_eq!(d1.hand[3].rank(), Rank::Deuce);
        assert!(v1 < best);

        Ok(())
    }

    #[cfg(feature = "badugi-tables")]
    #[test]
    fn test_bg_tables() -> Result<()> {
        use crate::utils::Random;

        fn ref_val(h: &Hand) -> HandValue {
            if h.len() <= 4 {
                return ojp_badugi_reference_evaluator(h);
            }
            ojp_best_of(h, 4, Scale::Badugi, ojp_badugi_reference_evaluator)
        }

        let mut deck = Deck::new_by_name("low");
        let mut rng = Random::new();

        for _ in 0..1000 {
            deck.refill_and_shuffle();
            let len = 1 + rng.uniform16(4) +
                rng.uniform16(4) + rng.uniform16(3);
            let hand1 = deck.new_hand().init(deck.draw(len));
            let hand2 = deck.new_hand().init(deck.draw(len));

            let vt1 = ojp_bg_value(&hand1);
            let vr1 = ref_val(&hand1);
            let vt2 = ojp_bg_value(&hand2);
            let vr2 = ref_val(&hand2);

            if vt1 < vt2 {
                assert!(vr1 < vr2);
            } else if vt1 > vt2 {
                assert!(vr1 > vr2);
            } else {
                assert_eq!(vr1, vr2);
            }
            if 4 == len {
                assert_eq!(OJP_BG_TABLE_2[vt1 as usize], vr1);
            }
        }
        Ok(())
    }

    #[cfg(feature = "badugi-tables")]
    #[test]
    fn test_bc_tables() -> Result<()> {
        use crate::utils::Random;

        fn ref_val(h: &Hand) -> HandValue {
            if h.len() <= 4 {
                return ojp_badugi_reference_evaluator(h);
            }
            ojp_best_of(h, 4, Scale::Badeucy, ojp_badugi_reference_evaluator)
        }
        let mut deck = Deck::new_by_name("english");
        let mut rng = Random::new();

        for _ in 0..1000 {
            deck.refill_and_shuffle();
            let len = 1 + rng.uniform16(4) +
                rng.uniform16(4) + rng.uniform16(3);
            let hand1 = deck.new_hand().init(deck.draw(len));
            let hand2 = deck.new_hand().init(deck.draw(len));

            let vt1 = ojp_bc_value(&hand1);
            let vr1 = ref_val(&hand1);
            let vt2 = ojp_bc_value(&hand2);
            let vr2 = ref_val(&hand2);

            if vt1 < vt2 {
                assert!(vr1 < vr2);
            } else if vt1 > vt2 {
                assert!(vr1 > vr2);
            } else {
                assert_eq!(vr1, vr2);
            }
            if 4 == len {
                assert_eq!(OJP_BC_TABLE_2[vt1 as usize], vr1);
            }
        }
        Ok(())
    }
}
