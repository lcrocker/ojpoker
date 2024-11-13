//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToFive) | Ace-to-five low hand values

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_full_name) | Describe ace-to-five hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("5s","9s","As","Ks","Js"));
/// let v = ojp_ace_to_five_eval_full(&hand).unwrap();
/// println!("{}", ojp_ace_to_five_full_name(&v));
/// // Output: "king, jack, nine, five, ace" (no flush, ace is low)
/// ```
pub fn ojp_ace_to_five_full_name(v: &HandValue) -> String {
    macro_rules! sng {
        ($x:literal) => { v.hand[$x as usize].rank().name() }
    }

    debug_assert!(v.level != HandLevel::StraightFlush as u8);
    debug_assert!(v.level != HandLevel::Flush as u8);
    debug_assert!(v.level != HandLevel::Straight as u8);

    match HandLevel::from_u8(v.level) {
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_high_full_name(v)
    }
}

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_full_name) | Describe action razz hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("5s","9s","As","Ks","Js"));
/// let v = ojp_action_razz_eval_full(&hand).unwrap();
/// println!("{}", ojp_action_razz_full_name(&v));
/// // Output: "king, jack, nine, five, ace" (no flush, ace is low)
/// ```
pub fn ojp_action_razz_full_name(v: &HandValue) -> String {
    macro_rules! sng {
        ($x:literal) => { v.hand[$x as usize].rank().name() }
    }
    macro_rules! plr {
        ($x:literal) => { v.hand[$x as usize].rank().plural() }
    }
    macro_rules! art {
        ($x:literal) => { v.hand[$x as usize].rank().article() }
    }

    match HandLevel::from_u8(v.level) {
        HandLevel::UnqualifiedQuads => {
            format!("unqualified four {} with {} {}", plr!(0), art!(4), sng!(4))
        },
        HandLevel::UnqualifiedFullHouse => {
            format!("unqualified {} full of {}", plr!(0), plr!(3))
        },
        HandLevel::UnqualifiedTrips => {
            format!("unqualified three {}, {}, {}", plr!(0), sng!(3), sng!(4))
        },
        HandLevel::UnqualifiedTwoPair => {
            format!("unqualified {} and {} with {} {}", plr!(0), plr!(2), art!(4), sng!(4))
        },
        HandLevel::UnqualifiedPair => {
            format!("unqualified pair of {}, {}, {}, {}", plr!(0), sng!(2), sng!(3), sng!(4))
        },
        HandLevel::UnqualifiedNoPair => {
            format!("unqualified {}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        HandLevel::NoPair => {
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => ojp_high_full_name(v)
    }
}

fn curried_evaluator_full(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::AceToFive)
}

fn curried_evaluator_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::AceToFive)
}

fn action_razz_adjust_level(level: HandLevel) -> HandLevel {
    match level {
        HandLevel::Quads => HandLevel::UnqualifiedQuads,
        HandLevel::FullHouse => HandLevel::UnqualifiedFullHouse,
        HandLevel::Trips => HandLevel::UnqualifiedTrips,
        HandLevel::TwoPair => HandLevel::UnqualifiedTwoPair,
        HandLevel::Pair => HandLevel::UnqualifiedPair,
        HandLevel::NoPair => HandLevel::UnqualifiedNoPair,
        _ => level
    }
}

#[cfg(feature = "ace-to-five-tables")]
use crate::poker::ace_to_five_tables::*;

#[cfg(feature = "ace-to-five-tables")]
/// Quick lookup table evaluator
fn lookup_ace_to_five(h: &Hand) -> u32 {
    let h = ojh_positional_32cs_mp5_low(h.as_slice()).
    expect("should have been checked by this time");
    ACE_TO_FIVE_TABLE_1[h as usize] as u32
}

#[cfg(feature = "ace-to-five-tables")]
/// Full ace-to-five poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_full) | Ace-to-five full evaluator
pub fn ojp_ace_to_five_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_ace_to_five(h)
    } else {
        ojp_best_value_of(h, HandScale::AceToFive, lookup_ace_to_five)
    };
    let vv = ACE_TO_FIVE_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::AceToFive,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "ace-to-five-tables")]
/// Value-only ace-to-five poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_quick) | Ace-to-five quick evaluator
pub fn ojp_ace_to_five_eval_quick(h: &Hand) -> u32 {
    if 5 == h.len(){
        return lookup_ace_to_five(h);
    }
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    ojp_best_value_of(h, HandScale::AceToFive, lookup_ace_to_five)
}

#[cfg(not(feature = "ace-to-five-tables"))]
/// Full ace-to-five hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_full) | Ace-to-five full evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("7s","4s","As","Qd","Ac"));
/// let v = ojp_ace_to_five_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, ojp_ace_to_five_full_name(&v));
/// // Output: "[AsAcQd7s4s]: pair of aces, queen, seven, four"
/// ```
pub fn ojp_ace_to_five_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::AceToFive,
            curried_evaluator_full);
    }
    curried_evaluator_full(h)
}

#[cfg(not(feature = "ace-to-five-tables"))]
/// Value-only ace-to-five hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_quick) | Ace-to-five quick evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::Low).init(cards!("3s","9s","7d","5c","2c"));
/// let h2 = Hand::new(DeckType::Low).init(cards!("7s","4s","As","Qd","6c"));
/// let v1 = ojp_ace_to_five_eval_quick(&h1);
/// let v2 = ojp_ace_to_five_eval_quick(&h2);
/// assert!(v1 < v2);   // nine-high beats queen-high
/// ```
pub fn ojp_ace_to_five_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::AceToFive,
            curried_evaluator_quick);
    }
    curried_evaluator_quick(h)
}

#[cfg(feature = "ace-to-five-tables")]
/// Full action razz poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_eval_full) | Action razz full evaluator
pub fn ojp_action_razz_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_ace_to_five(h)
    } else {
        ojp_best_value_of(h, HandScale::AceToFive, lookup_ace_to_five)
    };
    let vv = ACE_TO_FIVE_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::AceToFive,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);

    for i in 0..h.len() {
        if v.hand[i].rank() > Rank::Ten {
            return Ok(v);
        }
    }
    v.value = ACTION_RAZZ_ADJUST[ec as usize] as u32;
    debug_assert!(v.value > 6175);
    v.level = action_razz_adjust_level(HandLevel::from_u8(v.level)) as u8;
    Ok(v)
}

#[cfg(feature = "ace-to-five-tables")]
/// Value-only action razz poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_eval_quick) | Action razz quick evaluator
pub fn ojp_action_razz_eval_quick(h: &Hand) -> u32 {
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    let ec = if 5 == h.len(){
        lookup_ace_to_five(h)
    } else {
        ojp_best_value_of(h, HandScale::AceToFive, lookup_ace_to_five)
    };
    for i in 0..h.len() {
        if h[i].rank() > Rank::Ten {
            return ec;
        }
    }
    return ACTION_RAZZ_ADJUST[ec as usize] as u32;
}

#[cfg(not(feature = "ace-to-five-tables"))]
/// Full action razz hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_raz_eval_full) | Action razz full evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("7s","4s","As","Qd","Ac"));
/// let v = ojp_action_razz_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, ojp_ace_to_five_full_name(&v));
/// // Output: "[AsAcQd7s4s]: pair of aces, queen, seven, four"
/// ```
pub fn ojp_action_razz_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    // Incomplete hands are considered qualified
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let mut v = if h.len() > 5 {
        ojp_best_of(h, HandScale::AceToFive, curried_evaluator_full)?
    } else {
        curried_evaluator_full(h)?
    };
    for i in 0..h.len() {
        if v.hand[i].rank() > Rank::Ten {
            return Ok(v);
        }
    }
    v.value += 7 * HandScale::ActionRazz.multiplier();
    v.level = action_razz_adjust_level(HandLevel::from_u8(v.level)) as u8;
    Ok(v)
}

#[cfg(not(feature = "ace-to-five-tables"))]
/// Value-only action razz hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_eval_quick) | Action razz quick evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::Low).init(cards!("7s","4s","As","Qd","6c"));
/// let h2 = Hand::new(DeckType::Low).init(cards!("3s","9s","7d","5c","2c"));
/// let v1 = ojp_action_razz_eval_quick(&h1);
/// let v2 = ojp_action_razz_eval_quick(&h2);
/// assert!(v1 < v2);   // nine-high beats queen-high
/// ```
pub fn ojp_action_razz_eval_quick(h: &Hand) -> u32 {
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    let v = if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::AceToFive,
            curried_evaluator_quick)
    } else {
        curried_evaluator_quick(h)
    };
    for i in 0..h.len() {
        if h[i].rank() > Rank::Ten {
            return v
        }
    }
    v + 7 * HandScale::ActionRazz.multiplier()
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_ace_to_five() -> Result<(), OjError> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFF_FFFF;

        hand.set(cards!("Ks","Kh","Kd","Kc","Qs"));
        let mut v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);

        hand.set(cards!("Kd","Qc","Kc","Kh","Ks"));
        let mut v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Jd","5c","Jc","Jh","5s"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","5c","Js","5h","Jc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","4c","7s","7h","Kc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2d","Tc","2s","3h","Ts"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","3c","9d","9h","Qc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Js","Ks","9s","Qs"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kh","Td","9s","Qc","Jc"));
        v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(cards!("Kc","3d","9d","6h","2d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6c","9c","3c","Kc","2c"));
        v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(cards!("5c","3d","4s","7s","2d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","2c","4s","5d","3d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_action_razz() -> Result<(), OjError> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFF_FFFF;

        hand.set(cards!("Ts","Th","Td","Tc","9s"));
        let mut v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedQuads as u8);

        hand.set(cards!("Td","9c","Tc","Th","Ts"));
        let mut v2 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9d","5c","9c","9h","5s"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedFullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","5c","9s","5h","9c"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedFullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","4c","7s","7h","Tc"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedTrips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2d","Tc","2s","3h","Ts"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedTwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","3c","9d","9h","5c"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6c","9c","3c","Tc","2c"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5c","3d","4s","7s","2d"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","2c","4s","5d","3d"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ks","Kh","Kd","Kc","Qs"));
        let mut v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);

        hand.set(cards!("Kd","Qc","Kc","Kh","Ks"));
        v2 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1, v2);
        best = v1.value;

        hand.set(cards!("Jd","5c","Jc","Jh","5s"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","5c","Js","5h","Jc"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","4c","7s","7h","Kc"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2d","Tc","2s","Qh","Ts"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","3c","9d","9h","Qc"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Js","Ks","9s","Qs"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kh","Td","9s","Qc","Jc"));
        v2 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert_eq!(v1, v2);

        hand.set(cards!("Kc","3d","9d","6h","2d"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6c","9c","3c","Kc","2c"));
        v2 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(cards!("5c","3d","4s","7s","2d","Qh"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","2c","4s","5d","3d","Ks"));
        v1 = ojp_action_razz_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);
        assert!(v1.value < best);

        Ok(())
    }

}
