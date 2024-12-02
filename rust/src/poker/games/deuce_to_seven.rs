//! [wiki](https://github.com/lcrocker/ojpoker/wiki/DeuceToSeven) | Deuce-to-seven "Kansas City" low poker hands

use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_full_name) | Describe deuce-to-seven low hands
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","7s","4s","5s"));
/// let v = Scale::DeuceToSeven.value(&hand);
/// let d = Scale::DeuceToSeven.description(&hand, v);
/// println!("{}", d.full_text());
/// // Output: "flush: ace, nine, seven, five, four"
pub fn ojp_deuce_to_seven_full_text(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    match d.level {
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_high_full_text(d),
    }
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_description) | Deuce-to-seven low hand description
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::DeuceToSeven, v)
}

/// [wiki](https:://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_description) | Deuce-to-seven low hand description
#[cfg(feature = "deuce-to-seven-tables")]
pub fn ojp_deuce_to_seven_description(h: &Hand, v: HandValue) -> HandDescription {
    HandDescription::from_value(h, Scale::DeuceToSeven,
        DEUCE_TO_SEVEN_TABLE_2[v as usize])
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_5_quick) | Deuce-to-seven 5-card evaluator
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_eval_5(h: &Hand) -> HandValue {
    ojp_reference_evaluator(h, Scale::DeuceToSeven)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_5_quick) | Deuce-to-seven 5-card evaluator
#[cfg(feature = "deuce-to-seven-tables")]
pub fn ojp_deuce_to_seven_eval_5(h: &Hand) -> HandValue {
    let h = ojh_mp5_english(h[..]);
    DEUCE_TO_SEVEN_TABLE_1[h as usize] as u32
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_value) | Deuce-to-seven evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::English).init(hand!("5c","Ah","3s","4s","2d"));
/// let h2 = Hand::new(DeckType::English).init(hand!("5c","7h","3s","4s","3d"));
/// let v1 = Scale::DeuceToSeven.value(&h1);
/// let v2 = Scale::DeuceToSeven.value(&h2);
/// assert!(v1 < v2);   // ace-high beats pair
/// ```
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::DeuceToSeven.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::DeuceToSeven),
        5 => ojp_deuce_to_seven_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::DeuceToSeven, ojp_deuce_to_seven_eval_5),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_value) | Deuce-to-seven evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::English).init(hand!("5c","Ah","3s","4s","2d"));
/// let h2 = Hand::new(DeckType::English).init(hand!("5c","7h","3s","4s","3d"));
/// let v1 = Scale::DeuceToSeven.value(&h1);
/// let v2 = Scale::DeuceToSeven.value(&h2);
/// assert!(v1 < v2);   // ace-high beats pair
/// ```
#[cfg(feature = "deuce-to-seven-tables")]
pub fn ojp_deuce_to_seven_value(h: &Hand) -> HandValue {
    debug_assert!(Scale::DeuceToSeven.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator(h, Scale::DeuceToSeven),
        5 => ojp_deuce_to_seven_eval_5(h),
        6.. => ojp_best_of(h, 5, Scale::DeuceToSeven, ojp_deuce_to_seven_eval_5),
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
    fn test_hand_evaluator_deuce_to_seven() -> Result<()> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("8c","Jc","9c","7c","Tc"));
        let mut v1 = ojp_deuce_to_seven_value(&hand);
        let mut d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::StraightFlush);

        hand.set(hand!("Td","7d","8d","9d","Jd"));
        let mut v2 = ojp_deuce_to_seven_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4c","Jc","4s","4d","4h"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Quads);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Kc","Kd","8d","8c","Kh"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("8c","8h","Kc","8d","Ks"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::FullHouse);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("4s","As","5s","3s","2s"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5h","Th","7h","4h","Jh"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Flush);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("Qd","Kh","Ac","Jc","Td"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("7d","9s","8d","Ts","Jh"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Straight);

        hand.set(hand!("9h","7d","Ts","Jc","8h"));
        v2 = ojp_deuce_to_seven_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2h","5s","2c","2d","9c"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Trips);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5s","Ts","Ks","Td","5c"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::TwoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("3h","2s","9c","3s","8h"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::Pair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("2d","Ah","5s","3s","4s"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("8d","3d","4c","Kc","Th"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("4c","8s","Ks","Td","3h"));
        v2 = ojp_deuce_to_seven_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);
        best = v1;

        hand.set(hand!("5h","2s","3h","7s","4h"));
        v1 = ojp_deuce_to_seven_value(&hand);
        d1 = ojp_deuce_to_seven_description(&hand, v1);
        assert_eq!(d1.level, HandLevel::NoPair);

        hand.set(hand!("3s","4h","7d","2d","5d"));
        v2 = ojp_deuce_to_seven_value(&hand);
        assert_eq!(v1, v2);
        assert!(v1 < best);

        Ok(())
    }
}

