//! [wiki](https://github.com/lcrocker/ojpoker/wiki/DeuceToSeven) | Deuce-to-seven "Kansas City" low poker hands

use crate::error::Result;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_full_name) | Describe deuce-to-seven low hands
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","7s","4s","5s"));
/// let v = Scale::DeuceToSeven.eval(&hand).unwrap();
/// println!("{}", v.full_name());
/// // Output: "flush: ace, nine, seven, five, four"
pub fn ojp_deuce_to_seven_full_name(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    match d.level {
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_high_full_name(d),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_5_full) | Deuce-to-seven 5-card full evaluator
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_eval_5_full(h: &Hand) -> Result<HandDescription> {
    ojp_reference_evaluator_full(h, Scale::DeuceToSeven)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_5_quick) | Deuce-to-seven 5-card quick evaluator
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_eval_5_quick(h: &Hand) -> HandValue {
    ojp_reference_evaluator_quick(h, Scale::DeuceToSeven)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_full) | Deuce-to-seven full evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","7s","4s","5s"));
/// let v = Scale::DeuceToSeven.eval(&hand).unwrap();
/// println!("[{}]: {}", v, v.full_name());
/// // Output: "[As9s7s5s4s]: flush: ace, nine, seven, five, four"
/// ```
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_eval_full(h: &Hand) -> Result<HandDescription> {
    debug_assert!(Scale::DeuceToSeven.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_full(h, Scale::DeuceToSeven),
        5 => ojp_deuce_to_seven_eval_5_full(h),
        6.. => ojp_best_of(h, Scale::DeuceToSeven, ojp_deuce_to_seven_eval_5_full),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_quick) | Deuce-to-seven quick evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::English).init(hand!("5c","Ah","3s","4s","2d"));
/// let h2 = Hand::new(DeckType::English).init(hand!("5c","7h","3s","4s","3d"));
/// let v1 = Scale::DeuceToSeven.eval_quick(&h1);
/// let v2 = Scale::DeuceToSeven.eval_quick(&h2);
/// assert!(v1 < v2);   // ace-high beats pair
/// ```
#[cfg(not(feature = "deuce-to-seven-tables"))]
pub fn ojp_deuce_to_seven_eval_quick(h: &Hand) -> HandValue {
    debug_assert!(Scale::DeuceToSeven.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_quick(h, Scale::DeuceToSeven),
        5 => ojp_deuce_to_seven_eval_5_quick(h),
        6.. => ojp_best_value_of(h, Scale::DeuceToSeven, ojp_deuce_to_seven_eval_5_quick),
    }
}

#[cfg(feature = "deuce-to-seven-tables")]
use crate::poker::deuce_to_seven_tables::*;

#[cfg(feature = "deuce-to-seven-tables")]
/// Quick lookup table evaluator
fn lookup_deuce_to_seven(h: &Hand) -> u32 {
    let h = ojh_mp5_english(ojh_bitfield_64co(h.as_slice()).
        expect("should have been checked by this time"));
    DEUCE_TO_SEVEN_TABLE_1[h as usize] as u32
}

#[cfg(feature = "deuce-to-seven-tables")]
/// Full deuce-to-seven poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_full) | Deuce-to-seven full evaluator
pub fn ojp_deuce_to_seven_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_deuce_to_seven(h)
    } else {
        ojp_best_value_of(h, Scale::DeuceToSeven, lookup_deuce_to_seven)
    };
    let vv = DEUCE_TO_SEVEN_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, Scale::DeuceToSeven,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "deuce-to-seven-tables")]
/// Value-only deuce-to-seven poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_quick) | Deuce-to-seven quick evaluator
pub fn ojp_deuce_to_seven_eval_quick(h: &Hand) -> u32 {
    if 5 == h.len(){
        return lookup_deuce_to_seven(h)
    }
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    ojp_best_value_of(h, Scale::DeuceToSeven, lookup_deuce_to_seven)
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_deuce_to_seven() -> Result<()> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("8c","Jc","9c","7c","Tc"));
        let mut v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush);

        hand.set(hand!("Td","7d","8d","9d","Jd"));
        let mut v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4c","Jc","4s","4d","4h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Kc","Kd","8d","8c","Kh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("8c","8h","Kc","8d","Ks"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4s","As","5s","3s","2s"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5h","Th","7h","4h","Jh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Qd","Kh","Ac","Jc","Td"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7d","9s","8d","Ts","Jh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight);

        hand.set(hand!("9h","7d","Ts","Jc","8h"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("2h","5s","2c","2d","9c"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5s","Ts","Ks","Td","5c"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("3h","2s","9c","3s","8h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("2d","Ah","5s","3s","4s"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("8d","3d","4c","Kc","Th"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);

        hand.set(hand!("4c","8s","Ks","Td","3h"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5h","2s","3h","7s","4h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);

        hand.set(hand!("3s","4h","7d","2d","5d"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }
}

