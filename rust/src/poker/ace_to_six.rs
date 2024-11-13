//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToSix) | Ace-to-six "London" low poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_full_name) | Describe ace-to-six hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("9s","6s","7c","5h","8c"));
/// let v = ojp_ace_to_six_eval_full(&hand).unwrap();
/// println!("{}", ojp_ace_to_six_full_name(&v));
/// // Output: "nine-high straight"
/// ```

pub fn ojp_ace_to_six_full_name(v: &HandValue) -> String {
    macro_rules! sng {
        ($x:literal) => { v.hand[$x as usize].rank().name() }
    }

    match HandLevel::from_u8(v.level) {
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_high_full_name(v)
    }
}

fn curried_evaluator_full(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::AceToSix)
}

fn curried_evaluator_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::AceToSix)
}

#[cfg(feature = "ace-to-six-tables")]
use crate::poker::ace_to_six_tables::*;

#[cfg(feature = "ace-to-six-tables")]
/// Quick lookup table evaluator
fn lookup_ace_to_six(h: &Hand) -> u32 {
    let h = ojh_mp5_low(ojh_bitfield_64co(h.as_slice()).
        expect("should have been checked by this time"));
    ACE_TO_SIX_TABLE_1[h as usize] as u32
}

#[cfg(feature = "ace-to-six-tables")]
/// Full ace-to-five poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_eval_full) | Full ace-to-six evaluator
pub fn ojp_ace_to_six_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_ace_to_six(h)
    } else {
        ojp_best_value_of(h, HandScale::AceToSix, lookup_ace_to_six)
    };
    let vv = ACE_TO_SIX_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::AceToSix,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "ace-to-six-tables")]
/// Value-only high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_eval_quick) | Quick ace-to-six evaluator
pub fn ojp_ace_to_six_eval_quick(h: &Hand) -> u32 {
    if 5 == h.len(){
        return lookup_ace_to_six(h);
    }
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    ojp_best_value_of(h, HandScale::AceToSix, lookup_ace_to_six)
}

#[cfg(not(feature = "ace-to-six-tables"))]
/// Full high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_eval_full) | Full ace-to-six evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("9s","6s","7c","5h","8c"));
/// let v = ojp_ace_to_six_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, ojp_ace_to_six_full_name(&v));
/// // Output: "[9s8c7c6s5h]: nine-high straight"
/// ```
pub fn ojp_ace_to_six_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::AceToSix,
            curried_evaluator_full);
    }
    curried_evaluator_full(h)
}

#[cfg(not(feature = "ace-to-six-tables"))]
/// Value-only high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_six_eval_quick) | Quick ace-to-six evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::Low).init(cards!("Qs","Qs","5c","Qh","6c"));
/// let h2 = Hand::new(DeckType::Low).init(cards!("9s","6s","7c","5h","8c"));
/// let v1 = ojp_ace_to_six_eval_quick(&h1);
/// let v2 = ojp_ace_to_six_eval_quick(&h2);
/// assert!(v1 < v2);   // trips beats straight
/// ```
pub fn ojp_ace_to_six_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::AceToSix,
            curried_evaluator_quick);
    }
    curried_evaluator_quick(h)
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_ace_to_six() -> Result<(), OjError> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFF_FFFF;

        hand.set(cards!("8c","Jc","9c","Qc","Tc"));
        let mut v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Td","Qd","8d","9d","Jd"));
        let mut v2 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","As","5s","3s","2s"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6c","Tc","6s","6d","6h"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qc","Qd","6d","6c","Qh"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6c","6h","Qc","6d","Qs"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qs","As","Ts","Ks","Js"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","Jh","6h","4h","9h"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qd","9s","8d","Ts","Jh"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9h","Qd","Ts","Jc","8h"));
        v2 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2d","Ah","5s","3s","4s"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3h","5s","3c","3d","9c"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","Js","Ks","Jd","4c"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6h","2s","9c","6s","8h"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qd","Kh","Ac","Jc","Td"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("8d","3d","4c","Kc","Th"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);

        hand.set(cards!("4c","8s","Ks","Td","3h"));
        v2 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","2s","3h","7s","4h"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3s","4h","6d","2d","Ad"));
        v1 = ojp_ace_to_six_eval_full(&hand)?;
        assert!(v1.value < best);

        Ok(())
    }
}

