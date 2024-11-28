//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HighHand) | Traditional "high" poker hands

use crate::error::Result;
use crate::cards::*;
use crate::poker::*;

/// Full english name of hand e.g. "king-high straight"
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_full_name) | Full english name of hand
/// ```rust
/// use onejoker::prelude::*;
/// use onejoker::poker::{ojp_high_eval_full,ojp_high_full_name};
///
/// let hand = Hand::new(DeckType::English).init(hand!("9s","As","9d","Ks","Ah"));
/// let v = ojp_high_eval_full(&hand).unwrap();
/// println!("{}", ojp_high_full_name(&v));
/// // Output: "aces and nines with a king"
/// ```
pub fn ojp_high_full_name(d: &HandDescription) -> String {
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

#[cfg(not(feature = "high-hand-tables"))]
fn high_eval_5_full(h: &Hand) -> Result<HandDescription> {
    ojp_reference_evaluator_full(h, Scale::HighHand)
}

#[cfg(not(feature = "high-hand-tables"))]
fn high_eval_5_quick(h: &Hand) -> HandValue {
    ojp_reference_evaluator_quick(h, Scale::HighHand)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_full) | Full high poker hand evaluator
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_high_eval_full(h: &Hand) -> Result<HandDescription> {
    debug_assert!(Scale::HighHand.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_full(h, Scale::HighHand),
        5 => high_eval_5_full(h),
        6.. => ojp_best_of(h, Scale::HighHand, high_eval_5_full),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_quick) | Value-only high poker hand evaluator
#[cfg(not(feature = "high-hand-tables"))]
pub fn ojp_high_eval_quick(h: &Hand) -> HandValue {
    debug_assert!(Scale::HighHand.valid_hand(h));

    match h.len() {
        ..5 => ojp_reference_evaluator_quick(h, Scale::HighHand),
        5 => high_eval_5_quick(h),
        6.. => ojp_best_value_of(h, Scale::HighHand, high_eval_5_quick),
    }
}

#[cfg(feature = "high-hand-tables")]
use crate::poker::high_hand_tables::*;

// Map from equivalence class to hand level
#[cfg(feature = "high-hand-tables")]
fn high_hand_level_from_value(ec: HandValue) -> HandLevel {
    if ec > 6185 { return HandLevel::NoPair; }
    if ec > 3325 { return HandLevel::Pair; }
    if ec > 2467 { return HandLevel::TwoPair; }
    if ec > 1609 { return HandLevel::Trips; }
    if ec > 1599 { return HandLevel::Straight; }
    if ec > 322 { return HandLevel::Flush; }
    if ec > 166 { return HandLevel::FullHouse; }
    if ec > 10 { return HandLevel::Quads; }
    HandLevel::StraightFlush
}

// Since we didn't go through the evaluation process, the hand is not ordered
// for display, so we look up the ranks in table 3 and reorder them.
#[cfg(feature = "high-hand-tables")]
fn reorder_from_lookup(desc: &mut HandDescription) {
    let d = desc.as_mut();
    let ranks = OJP_HIGH_MP5_TABLE_3[d.value() as usize];

    if d.hand().len() == 7 &&
        (HandLevel::Flush == d.level() || HandLevel::StraightFlush == d.level()) {
        let mut counts = [0; 4];
        let mut flush_suit = Suit::None;
        let h = d.mut_hand();

        for i in 0..h.len() {
            counts[h[i].suit() as usize + 1] += 1;
            if counts[h[i].suit() as usize + 1] > 2 {
                flush_suit = h[i].suit();
                break;
            }
        }
        for i in 0..5 {
            let c = Card::from_rank_suit(Rank::from_u8(
                ((ranks >> (4 * (4 - i))) & 0x0F) as u8), flush_suit);

            for j in i..h.len() {
                if h[j] == c {
                    if i != j { h[..].swap(i, j); }
                    break;
                }
            }
        }
        return;
    } else {
        debug_assert!(d.hand().len() == 5);
    }
    for i in 0..5 {
        let r = Rank::from_u8(((ranks >> (4 * (4 - i))) & 0x0F) as u8);
        let h = d.mut_hand();

        for j in i..h.len() {
            if h[j].rank() == r {
                if i == j { continue; }
                if h[i].rank() != r || h[j] > h[i] {
                    h[..].swap(i, j);
                }
            }
        }
    }
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_5_full) | Full 5-card high poker hand evaluator
pub fn ojp_high_eval_5_full(hand: &Hand) -> Result<HandDescription> {
    let hash = ojh_mp5_english(&hand[..5]);
    let ec = OJP_HIGH_MP5_TABLE_1[hash as usize] as u32;
    let level = high_hand_level_from_value(ec);

    let mut hd =
        HandDescriptionBuilder::new(hand, Scale::HighHand)
            .with_level(level).with_value(ec).complete().unwrap();
    reorder_from_lookup(&mut hd);
    Ok(hd)
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_5_quick) | Quick 5-card high poker hand evaluator
pub fn ojp_high_eval_5_quick(hand: &Hand) -> HandValue {
    let hash = ojh_mp5_english(&hand[..5]);
    OJP_HIGH_MP5_TABLE_1[hash as usize] as HandValue
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_7_full) | Full 7-card high poker hand evaluator
pub fn ojp_high_eval_7_full(hand: &Hand) -> Result<HandDescription> {
    let hash = ojh_mp7_english(&hand[..7]);
    let ec = OJP_HIGH_MP7_TABLE_1[hash as usize] as u32;
    let level = high_hand_level_from_value(ec);

    let mut hd =
        HandDescriptionBuilder::new(hand, Scale::HighHand)
        .with_level(level).with_value(ec).complete().unwrap();
    reorder_from_lookup(&mut hd);
    Ok(hd)
}

#[cfg(feature = "high-hand-tables")]
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_7_quick) | Quick 7-card high poker hand evaluator
pub fn ojp_high_eval_7_quick(hand: &Hand) -> HandValue {
    let hash = ojh_mp7_english(&hand[..7]);
    OJP_HIGH_MP7_TABLE_1[hash as usize] as HandValue
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_full) | Full high poker hand evaluator
/// Full high poker hand evaluator
#[cfg(feature = "high-hand-tables")]
pub fn ojp_high_eval_full(h: &Hand) -> Result<HandDescription> {
    match h.len() {
        8..=13 => ojp_best_of(h, Scale::HighHand, ojp_high_eval_7_full),
        7 => ojp_high_eval_7_full(h),
        6 => ojp_best_of(h, Scale::HighHand, ojp_high_eval_5_full),
        5 => ojp_high_eval_5_full(h),
        _ => {
            if h.len() > 5 {
                return Err(Error::BadHand("too many cards".to_string()));
            }
            ojp_reference_evaluator_full(h, Scale::HighHand)
        },
    }
}

/// Value-only high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_quick) | Value-only high poker hand evaluator
#[cfg(feature = "high-hand-tables")]
pub fn ojp_high_eval_quick(h: &Hand) -> HandValue {
    match h.len() {
        8..=13 => ojp_best_value_of(h, Scale::HighHand, ojp_high_eval_7_quick),
        7 => ojp_high_eval_7_quick(h),
        6 => ojp_best_value_of(h, Scale::HighHand, ojp_high_eval_5_quick),
        5 => ojp_high_eval_5_quick(h),
        _ => {
            assert!(h.len() < 5);
            ojp_reference_evaluator_quick(h, Scale::HighHand)
        },
    }
}

/*
 * HIGH HANDS WITH BUG
 */

// fn curried_evaluator_high_bug_full(h: &Hand) -> Result<HandDescription> {
//     let Some(repl) = ojp_bug_replace_high(h) else {
//         return ojp_high_eval_full(h);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     let mut v = ojp_high_eval_full(&dup)?;
//     v.bug_is = repl.replacement;
//     Ok(v)
// }

// fn curried_evaluator_high_bug_quick(h: &Hand) -> u32 {
//     let Some(repl) = ojp_bug_replace_high(h) else {
//         return ojp_high_eval_quick(h);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     ojp_high_eval_quick(&dup)
// }

/// Full high with bug poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_bug_eval_full) | Full high hand with bug evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::OneJoker).init(hand!("9s","Jk","9d","Ks","Ah"));
/// let v = ojp_high_bug_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
// pub fn ojp_high_bug_eval_full(h: &Hand) -> Result<HandDescription> {
//     if h.len() > 5 {
//         return ojp_best_of(h, Scale::HighHand,
//             curried_evaluator_high_bug_full);
//     }
//     curried_evaluator_high_bug_full(h)
// }

/// Value-only high hand with bug evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_bug_eval_quick) | Value-only high hand with bug evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::OneJoker).init(hand!("9s","As","9d","Ks","Jk"));
/// let h2 = Hand::new(DeckType::OneJoker).init(hand!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_high_bug_eval_quick(&h1);
/// let v2 = ojp_high_bug_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
// pub fn ojp_high_bug_eval_quick(h: &Hand) -> u32 {
//     if h.len() > 5 {
//         return ojp_best_value_of(h, Scale::HighHand,
//             curried_evaluator_high_bug_quick);
//     }
//     curried_evaluator_high_bug_quick(h)
// }

// fn curried_evaluator_stripped_bug_full(h: &Hand) -> Result<HandDescription> {
//     let Some(repl) = ojp_bug_replace_high(h) else {
//         return ojp_default_eval_full(h, Scale::Stripped);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     let mut v = ojp_default_eval_full(&dup, Scale::Stripped)?;
//     v.bug_is = repl.replacement;
//     Ok(v)
// }

// fn curried_evaluator_stripped_bug_quick(h: &Hand) -> u32 {
//     let Some(repl) = ojp_bug_replace_high(h) else {
//         return ojp_default_eval_quick(h, Scale::Stripped);
//     };
//     let mut dup = *h;
//     dup[repl.index as usize] = repl.replacement;
//     ojp_default_eval_quick(&dup, Scale::Stripped)
// }

// #[cfg(not(feature = "stripped-deck-tables"))]
// /// Full stripped deck with bug poker hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_bug_eval_full) | Full stripped deck with bug evaluator
// /// ```rust
// /// use onejoker::prelude::*;
// ///
// /// let hand = Hand::new(DeckType::OneJoker).init(hand!("9s","Jk","9d","Ks","Ah"));
// /// let v = ojp_stripped_bug_eval_full(&hand).unwrap();
// /// println!("[{}]: {}", v.hand, v.full_name());
// /// // Output: "[AsAh9s9dKs]: aces and nines with a king"
// /// ```
// pub fn ojp_stripped_bug_eval_full(h: &Hand) -> Result<HandDescription> {
//     if h.len() > 5 {
//         return ojp_best_of(h, Scale::HighHand,
//             curried_evaluator_stripped_bug_full);
//     }
//     curried_evaluator_stripped_bug_full(h)
// }

// #[cfg(not(feature = "stripped-deck-tables"))]
// /// Value-only stripped deck with bug evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_bug_eval_quick) | Value-only stripped deck with bug evaluator
// /// ```rust
// /// use onejoker::prelude::*;
// ///
// /// let h1 = Hand::new(DeckType::OneJoker).init(hand!("9s","As","9d","Ks","Jk"));
// /// let h2 = Hand::new(DeckType::OneJoker).init(hand!("9c","Ac","9h","Td","Ad"));
// /// let v1 = ojp_stripped_bug_eval_quick(&h1);
// /// let v2 = ojp_stripped_bug_eval_quick(&h2);
// /// assert!(v1 < v2);   // king kicker beats ten kicker
// /// ```
// pub fn ojp_stripped_bug_eval_quick(h: &Hand) -> u32 {
//     if h.len() > 5 {
//         return ojp_best_value_of(h, Scale::HighHand,
//             curried_evaluator_stripped_bug_quick);
//     }
//     curried_evaluator_stripped_bug_quick(h)
// }

// /// If there's a bug in the hand, figure out what card it should
// /// be, return it along with its index.
// pub fn ojp_bug_replace_high(h: &Hand) -> Option<BugReplacement> {
//     let scan = ojp_bug_scan(h);
//     let index = scan.index?;

//     if h.len() < 5 {    // partial hand, just ace
//         return Some(BugReplacement::new(index as usize,
//             ojp_ace_not_present(scan.ace_mask)));
//     }
//     let mut suit = Suit::None;
//     let mut rank = Rank::None;

//     if let Some(s) = FLUSH_PATTERNS.get(&scan.suit_mask) {
//         suit = Suit::from_u8(*s);
//     };
//     if let Some(r) = STRAIGHT_PATTERNS.get(&scan.rank_mask) {
//         rank = Rank::from_u8(*r);
//     };
//     Some(ojp_bug_replacement(rank, suit, &scan))
// }

// use lazy_static::lazy_static;

// lazy_static! {
//     /// After setting a bit for each suit in the hand, these are the patterns
//     /// that indicate the rank needed for the bug to complete a flush
//     pub static ref FLUSH_PATTERNS: std::collections::HashMap<u8, u8> = {
//         let mut m = std::collections::HashMap::new();
//         m.insert(0b00010, 1);
//         m.insert(0b00100, 2);
//         m.insert(0b01000, 3);
//         m.insert(0b10000, 4);
//         m
//     };
// }

// lazy_static! {
//     /// After setting a bit for each rank in the hand, these are the patterns
//     /// that indicate the rank needed for the bug to complete a straight.
//     pub static ref STRAIGHT_PATTERNS: std::collections::HashMap<u16, u8> = {
//         let mut m = std::collections::HashMap::new();
//         m.insert(0b0000000000111101, 6);
//         m.insert(0b0000000010111001, 6);
//         m.insert(0b0000000110110001, 6);
//         m.insert(0b0000001110100001, 6);
//         m.insert(0b0000000001111001, 7);
//         m.insert(0b0000000101110001, 7);
//         m.insert(0b0000001101100001, 7);
//         m.insert(0b0000011101000001, 7);
//         m.insert(0b0000000011110001, 8);
//         m.insert(0b0000001011100001, 8);
//         m.insert(0b0000011011000001, 8);
//         m.insert(0b0000111010000001, 8);
//         m.insert(0b0000000111100001, 9);
//         m.insert(0b0000010111000001, 9);
//         m.insert(0b0000110110000001, 9);
//         m.insert(0b0010110100000001, 9);
//         m.insert(0b0000001111000001, 10);
//         m.insert(0b0000101110000001, 10);
//         m.insert(0b0010101100000001, 10);
//         m.insert(0b0110101000000001, 10);
//         m.insert(0b1110100000000001, 10);
//         m.insert(0b0000011110000001, 11);
//         m.insert(0b0010011100000001, 11);
//         m.insert(0b0110011000000001, 11);
//         m.insert(0b1110010000000001, 11);
//         m.insert(0b0000111100000001, 13);
//         m.insert(0b0100111000000001, 13);
//         m.insert(0b1100110000000001, 13);
//         m.insert(0b0010111000000001, 14);
//         m.insert(0b1010110000000001, 14);
//         m.insert(0b0110110000000001, 15);
//         m
//     };
// }

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_high() -> Result<()> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(hand!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_high_eval_full(&hand)?;
        let mut q1 = ojp_high_eval_quick(&hand);
        assert_eq!(v1.level, HandLevel::NoPair);

        hand.set(hand!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_high_eval_full(&hand)?;
        let mut q2 = ojp_high_eval_quick(&hand);
        assert_eq!(v1, v2);
        assert_eq!(q1, q2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("9d","3d","Qc","Kc","Th"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair);

        hand.set(hand!("Qc","9s","Ks","Td","3h"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("6h","2d","9c","6d","Ts"));
        v1 = ojp_high_eval_full(&hand)?;
        q1 = ojp_high_eval_quick(&hand);
        assert_eq!(v1.level, HandLevel::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4h","8c","8d","Ad","4c"));
        v1 = ojp_high_eval_full(&hand)?;
        q2 = ojp_high_eval_quick(&hand);
        assert_eq!(v1.level, HandLevel::TwoPair);
        assert!(v1.value < best);
        assert!(q2 < q1);
        best = v1.value;

        hand.set(hand!("5h","7d","5c","5s","Kd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ah","5s","3s","4s","2d"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7d","9h","8d","Ts","6s"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight);

        hand.set(hand!("9c","7d","Tc","6c","8h"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Kd","As","Js","Th","Qh"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("5d","Td","8d","4d","Qd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ac","As","7d","7h","Ah"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("3c","3s","3d","3h","Kd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ad","5d","3d","2d","4d"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush);

        hand.set(hand!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }

    // #[test]
    // fn test_hand_evaluator_stripped() -> Result<()> {
    //     let deck = Deck::new_by_name("manila");
    //     let mut hand= deck.new_hand();
    //     let mut best: u32 = HAND_VALUE_WORST;

    //     hand.set(hand!("8c","9h","7c","Jd","Kd"));
    //     let mut v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

    //     hand.set(hand!("9h","8s","7c","Kh","Jd"));
    //     let mut v2 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("9d","7d","Qc","Kc","Th"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

    //     hand.set(hand!("Qc","9s","Ks","Td","7h"));
    //     v2 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7h","Qd","9c","7d","Ts"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Pair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Th","8c","8d","Ad","Tc"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::TwoPair as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("9h","7d","9c","9s","Kd"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Trips as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7d","9h","8d","Ts","Js"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Straight as u8);

    //     hand.set(hand!("9c","7d","Tc","Jc","8h"));
    //     v2 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Kd","As","Js","Th","Qh"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Straight as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7s","7h","Ac","7d","Ad"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ac","As","7d","7h","Ah"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FullHouse as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("7d","Td","8d","Ad","Qd"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Flush as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("8c","8s","8d","8h","Kd"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::Quads as u8);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ts","Qs","9s","Js","Ks"));
    //     v1 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::StraightFlush as u8);

    //     hand.set(hand!("Qh","9h","Kh","Th","Jh"));
    //     v2 = ojp_stripped_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert!(v1.value < best);

    //     Ok(())
    // }
}
