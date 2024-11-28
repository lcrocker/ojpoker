//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToFive) | Ace-to-five low hands

use crate::error::Result;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_full_name) | Describe ace-to-five hand
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_ace_to_five_eval_full,ojp_ace_to_five_full_name};
///
/// let hand = Hand::new(DeckType::Low).init(hand!("5s","9s","As","Ks","Js"));
/// let v = ojp_ace_to_five_eval_full(&hand).unwrap();
/// println!("{}", ojp_ace_to_five_full_name(&v));
/// // Output: "king, jack, nine, five, ace" (no flush, ace is low)
/// ```
pub fn ojp_ace_to_five_full_name(d: &HandDescription) -> String {
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
        _ => ojp_high_full_name(d)
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_5_full) | Ace-to-five 5-card full evaluator
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_ace_to_five_eval_5_full(h: &Hand) -> Result<HandDescription> {
    ojp_reference_evaluator_full(h, Scale::AceToFive)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_5_quick) | Ace-to-five 5-card quick evaluator
pub fn ojp_ace_to_five_eval_5_quick(h: &Hand) -> HandValue {
    ojp_reference_evaluator_quick(h, Scale::AceToFive)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_full) | Ace-to-five full evaluator
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_ace_to_five_eval_full,ojp_ace_to_five_full_name};
///
/// let hand = Hand::new(DeckType::Low).init(hand!("7s","4s","As","Qd","Ac"));
/// let v = ojp_ace_to_five_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, ojp_ace_to_five_full_name(&v));
/// // Output: "[AsAcQd7s4s]: pair of aces, queen, seven, four"
/// ```
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_ace_to_five_eval_full(h: &Hand) -> Result<HandDescription> {
    debug_assert!(Scale::AceToFive.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_full(h, Scale::AceToFive),
        5 => ojp_ace_to_five_eval_5_full(h),
        6.. => ojp_best_of(h, Scale::AceToFive, ojp_ace_to_five_eval_5_full),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_quick) | Ace-to-five quick evaluator
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_ace_to_five_eval_quick};
///
/// let h1 = Hand::new(DeckType::Low).init(hand!("3s","9s","7d","5c","2c"));
/// let h2 = Hand::new(DeckType::Low).init(hand!("7s","4s","As","Qd","6c"));
/// let v1 = ojp_ace_to_five_eval_quick(&h1);
/// let v2 = ojp_ace_to_five_eval_quick(&h2);
/// assert!(v1 < v2);   // nine-high beats queen-high
/// ```
#[cfg(not(feature = "ace-to-five-tables"))]
pub fn ojp_ace_to_five_eval_quick(h: &Hand) -> HandValue {
    debug_assert!(Scale::AceToFive.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_quick(h, Scale::AceToFive),
        5 => ojp_ace_to_five_eval_5_quick(h),
        6.. => ojp_best_value_of(h, Scale::AceToFive, ojp_ace_to_five_eval_5_quick),
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
        ojp_best_value_of(h, Scale::AceToFive, lookup_ace_to_five)
    };
    let vv = ACE_TO_FIVE_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, Scale::AceToFive,
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
    ojp_best_value_of(h, Scale::AceToFive, lookup_ace_to_five)
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
        ojp_best_value_of(h, Scale::AceToFive, lookup_ace_to_five)
    };
    let vv = ACE_TO_FIVE_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, Scale::AceToFive,
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
        ojp_best_value_of(h, Scale::AceToFive, lookup_ace_to_five)
    };
    for i in 0..h.len() {
        if h[i].rank() > Rank::Ten {
            return ec;
        }
    }
    return ACTION_RAZZ_ADJUST[ec as usize] as u32;
}

// #[cfg(not(feature = "ace-to-five-tables"))]
// /// Full action razz hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_raz_eval_full) | Action razz full evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let hand = Hand::new(DeckType::Low).init(hand!("7s","4s","As","Qd","Ac"));
// /// let v = ojp_action_razz_eval_full(&hand).unwrap();
// /// println!("[{}]: {}", v.hand, ojp_ace_to_five_full_name(&v));
// /// // Output: "[AsAcQd7s4s]: pair of aces, queen, seven, four"
// /// ```
// pub fn ojp_action_razz_eval_full(h: &Hand) -> Result<HandValue, OjError> {
//     // Incomplete hands are considered qualified
//     if h.len() < 5 {
//         return curried_evaluator_full(h);
//     }
//     let mut v = if h.len() > 5 {
//         ojp_best_of(h, Scale::AceToFive, curried_evaluator_full)?
//     } else {
//         curried_evaluator_full(h)?
//     };
//     for i in 0..h.len() {
//         if v.hand[i].rank() > Rank::Ten {
//             return Ok(v);
//         }
//     }
//     v.value += 7 * HAND_LEVEL_MULTIPLIER;
//     v.level = action_razz_adjust_level(HandLevel::from_u8(v.level)) as u8;
//     Ok(v)
// }

// #[cfg(not(feature = "ace-to-five-tables"))]
// /// Value-only action razz hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_eval_quick) | Action razz quick evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let h1 = Hand::new(DeckType::Low).init(hand!("7s","4s","As","Qd","6c"));
// /// let h2 = Hand::new(DeckType::Low).init(hand!("3s","9s","7d","5c","2c"));
// /// let v1 = ojp_action_razz_eval_quick(&h1);
// /// let v2 = ojp_action_razz_eval_quick(&h2);
// /// assert!(v1 < v2);   // nine-high beats queen-high
// /// ```
// pub fn ojp_action_razz_eval_quick(h: &Hand) -> u32 {
//     if h.len() < 5 {
//         return curried_evaluator_quick(h);
//     }
//     let v = if h.len() > 5 {
//         return ojp_best_value_of(h, Scale::AceToFive,
//             curried_evaluator_quick)
//     } else {
//         curried_evaluator_quick(h)
//     };
//     for i in 0..h.len() {
//         if h[i].rank() > Rank::Ten {
//             return v
//         }
//     }
//     v + 7 * HAND_LEVEL_MULTIPLIER
// }

// /// If there's a bug in the hand, figure out what card it should
// /// be, return it along with its index.
// pub fn ojp_bug_replace_ace_to_five(h: &Hand) -> Option<BugReplacement> {
//     let scan = ojp_bug_scan(h);
//     let index = scan.index?;

//     let mut r = 1;
//     while r < 15 {
//         if 0 == (scan.rank_mask & (1 << r)) {
//             break;
//         }
//         r += 1;
//         if 12 == r { r += 1; }
//     }
//     Some(BugReplacement::new(index as usize,
//         Card::from_rank_suit(Rank::from_u8(r), Suit::Spade)))
// }

// fn curried_evaluator_bug_full(h: &Hand) -> Result<HandValue, OjError> {
//     let Some(repl) = ojp_bug_replace_ace_to_five(h) else {
//         return ojp_default_eval_full(h, Scale::AceToFive);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     let mut v = ojp_default_eval_full(&dup, Scale::AceToFive)?;
//     v.bug_is = repl.replacement;
//     Ok(v)
// }

// fn curried_evaluator_bug_quick(h: &Hand) -> u32 {
//     let Some(repl) = ojp_bug_replace_ace_to_five(h) else {
//         return ojp_default_eval_quick(h, Scale::AceToFive);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     ojp_default_eval_quick(&dup, Scale::AceToFive)
// }

// /// Full ace-to-five hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_bug_eval_full) | Ace-to-five with bug full evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let hand = Hand::new(DeckType::Low).init(hand!("7s","4s","As","5d","Jk"));
// /// let v = ojp_ace_to_five_bug_eval_full(&hand).unwrap();
// /// println!("[{}]: {}", v.hand, ojp_ace_to_five_full_name(&v));
// /// // Output: "[7s5d4s2sAs]: seven, five, four, deuce, ace"
// /// ```
// pub fn ojp_ace_to_five_bug_eval_full(h: &Hand) -> Result<HandValue, OjError> {
//     if h.len() > 5 {
//         return ojp_best_of(h, Scale::AceToFive,
//             curried_evaluator_bug_full);
//     }
//     curried_evaluator_bug_full(h)
// }

// /// Value-only ace-to-five hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_bug_eval_quick) | Ace-to-five with bug quick evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let h1 = Hand::new(DeckType::LowJoker).init(hand!("3s","9s","7d","5c","2c"));
// /// let h2 = Hand::new(DeckType::LowJoker).init(hand!("7s","Jk","As","Qd","6c"));
// /// let v1 = ojp_ace_to_five_bug_eval_quick(&h1);
// /// let v2 = ojp_ace_to_five_bug_eval_quick(&h2);
// /// assert!(v1 < v2);   // nine-high beats queen-high
// /// ```
// pub fn ojp_ace_to_five_bug_eval_quick(h: &Hand) -> u32 {
//     if h.len() > 5 {
//         return ojp_best_value_of(h, Scale::AceToFive,
//             curried_evaluator_bug_quick);
//     }
//     curried_evaluator_bug_quick(h)
// }

// fn action_razz_adjust_level(level: HandLevel) -> HandLevel {
//     match level {
//         HandLevel::Quads => HandLevel::UnqualifiedQuads,
//         HandLevel::FullHouse => HandLevel::UnqualifiedFullHouse,
//         HandLevel::Trips => HandLevel::UnqualifiedTrips,
//         HandLevel::TwoPair => HandLevel::UnqualifiedTwoPair,
//         HandLevel::Pair => HandLevel::UnqualifiedPair,
//         HandLevel::NoPair => HandLevel::UnqualifiedNoPair,
//         _ => level
//     }
// }

// /// Full English name of hand, e.g. "aces and fours with a jack".
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_action_razz_full_name) | Describe action razz hand
// /// ```rust
// /// use onejoker::*;
// ///
// /// let hand = Hand::new(DeckType::Low).init(hand!("5s","9s","As","Ks","Js"));
// /// let v = ojp_action_razz_eval_full(&hand).unwrap();
// /// println!("{}", ojp_action_razz_full_name(&v));
// /// // Output: "king, jack, nine, five, ace" (no flush, ace is low)
// /// ```
// pub fn ojp_action_razz_full_name(v: &HandValue) -> String {
//     macro_rules! sng {
//         ($x:literal) => { v.hand[$x as usize].rank().name() }
//     }
//     macro_rules! plr {
//         ($x:literal) => { v.hand[$x as usize].rank().plural() }
//     }
//     macro_rules! art {
//         ($x:literal) => { v.hand[$x as usize].rank().article() }
//     }

//     match HandLevel::from_u8(v.level) {
//         HandLevel::UnqualifiedQuads => {
//             format!("unqualified four {} with {} {}", plr!(0), art!(4), sng!(4))
//         },
//         HandLevel::UnqualifiedFullHouse => {
//             format!("unqualified {} full of {}", plr!(0), plr!(3))
//         },
//         HandLevel::UnqualifiedTrips => {
//             format!("unqualified three {}, {}, {}", plr!(0), sng!(3), sng!(4))
//         },
//         HandLevel::UnqualifiedTwoPair => {
//             format!("unqualified {} and {} with {} {}", plr!(0), plr!(2), art!(4), sng!(4))
//         },
//         HandLevel::UnqualifiedPair => {
//             format!("unqualified pair of {}, {}, {}, {}", plr!(0), sng!(2), sng!(3), sng!(4))
//         },
//         HandLevel::UnqualifiedNoPair => {
//             format!("unqualified {}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
//         },
//         HandLevel::NoPair => {
//             format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
//         },
//         _ => ojp_high_full_name(v)
//     }
// }

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_ace_to_five() -> Result<()> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("Ks","Kh","Kd","Kc","Qs"));
        let mut v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads);

        hand.set(hand!("Kd","Qc","Kc","Kh","Ks"));
        let mut v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Jd","5c","Jc","Jh","5s"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5d","5c","Js","5h","Jc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7d","4c","7s","7h","Kc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("2d","Tc","2s","3h","Ts"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4s","3c","9d","9h","Qc"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ts","Js","Ks","9s","Qs"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Kh","Td","9s","Qc","Jc"));
        v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(hand!("Kc","3d","9d","6h","2d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("6c","9c","3c","Kc","2c"));
        v2 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1, v2);

        hand.set(hand!("5c","3d","4s","7s","2d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ah","2c","4s","5d","3d"));
        v1 = ojp_ace_to_five_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);
        assert!(v1.value < best);

        Ok(())
    }

    // #[test]
    // fn test_hand_evaluator_action_razz() -> Result<(), OjError> {
    //     let deck = Deck::new_by_name("low");
    //     let mut hand= deck.new_hand();
    //     let mut best: u32 = HAND_VALUE_WORST;

    //     hand.set(hand!("Ts","Th","Td","Tc","9s"));
    //     let mut v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedQuads as u8);

    //     hand.set(hand!("Td","9c","Tc","Th","Ts"));
    //     let mut v2 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("9d","5c","9c","9h","5s"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedFullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("5d","5c","9s","5h","9c"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedFullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7d","4c","7s","7h","Tc"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedTrips as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("2d","Tc","2s","3h","Ts"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedTwoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("4s","3c","9d","9h","5c"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("6c","9c","3c","Tc","2c"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("5c","3d","4s","7s","2d"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ah","2c","4s","5d","3d"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::UnqualifiedNoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ks","Kh","Kd","Kc","Qs"));
    //     let mut v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Quads as u8);
    //     assert!(v1.value < best);

    //     hand.set(hand!("Kd","Qc","Kc","Kh","Ks"));
    //     v2 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     best = v1.value;

    //     hand.set(hand!("Jd","5c","Jc","Jh","5s"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("5d","5c","Js","5h","Jc"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7d","4c","7s","7h","Kc"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Trips as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("2d","Tc","2s","Qh","Ts"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::TwoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("4s","3c","9d","9h","Qc"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Pair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ts","Js","Ks","9s","Qs"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Kh","Td","9s","Qc","Jc"));
    //     v2 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("Kc","3d","9d","6h","2d"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("6c","9c","3c","Kc","2c"));
    //     v2 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("5c","3d","4s","7s","2d","Qh"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ah","2c","4s","5d","3d","Ks"));
    //     v1 = ojp_action_razz_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);

    //     Ok(())
    // }

    // #[test]
    // fn test_hand_evaluator_ace_to_five_bug() -> Result<(), OjError> {
    //     let deck = Deck::new_by_name("lowjoker");
    //     let mut hand= deck.new_hand();
    //     let mut best: u32 = HAND_VALUE_WORST;

    //     hand.set(hand!("Ks","Kh","Kd","Kc","Qs"));
    //     let mut v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Quads as u8);

    //     hand.set(hand!("5d","5c","Js","5h","Jc"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7d","4c","7s","7h","Kc"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Trips as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("2d","Tc","2s","3h","Ts"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::TwoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("4s","Ac","9d","9h","Qc"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Pair as u8);
    //     assert!(v1.value < best);

    //     hand.set(hand!("9d","Qc","Jk","4s","9c"));
    //     let mut v2 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("Ts","Js","Ks","9s","Qs"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Kh","Td","9s","Qc","Jc"));
    //     v2 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("Ac","3d","9d","6h","2d"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("6c","9c","3c","Ac","2c"));
    //     v2 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("Ac","6c","2c","Jk","9c"));
    //     v2 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1, v2);

    //     hand.set(hand!("5c","3d","4s","7s","2d"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ah","2c","4s","5d","3d"));
    //     v1 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::NoPair as u8);

    //     hand.set(hand!("Jk","2c","Ah","3s","4d"));
    //     v2 = ojp_ace_to_five_bug_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);

    //     Ok(())
    // }
}
