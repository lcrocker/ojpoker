//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HighHand) | Traditional "high" poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full english name of hand e.g. "king-high straight"
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_full_name) | Full english name of hand
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let v = ojp_high_eval_full(&hand).unwrap();
/// println!("{}", ojp_high_full_name(&v));
/// // Output: "aces and nines with a king"
/// ```
pub fn ojp_high_full_name(v: &HandValue) -> String {
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
        HandLevel::FiveOfAKind => {
            format!("five {}", plr!(0))
        },
        HandLevel::StraightFlush => {
            if v.hand[0].rank() == Rank::Ace {
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

fn curried_evaluator_full(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::HighHand)
}

fn curried_evaluator_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::HighHand)
}

#[cfg(feature = "high-hand-tables")]
use crate::poker::high_hand_tables::*;

#[cfg(feature = "high-hand-tables")]
/// Quick lookup table evaluator
fn lookup_high(h: &Hand) -> u32 {
    let h = ojh_mp5_english(ojh_bitfield_64co(h.as_slice()).
        expect("should have been checked by this time"));
    HIGH_HAND_TABLE_1[h as usize] as u32
}

#[cfg(feature = "high-hand-tables")]
/// Full high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_full) | Full high poker hand evaluator
pub fn ojp_high_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_high(h)
    } else {
        ojp_best_value_of(h, HandScale::HighHand, lookup_high)
    };
    let vv = HIGH_HAND_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::HighHand,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "high-hand-tables")]
/// Value-only high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_quick) | Value-only high poker hand evaluator
pub fn ojp_high_eval_quick(h: &Hand) -> u32 {
    if 5 == h.len() {
        return lookup_high(h);
    }
    if h.len() < 5 {
        return curried_evaluator_quick(h);
    }
    ojp_best_value_of(h, HandScale::HighHand, lookup_high)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Full high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_full) | Full high poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let v = ojp_high_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_high_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::HighHand,
            curried_evaluator_full);
    }
    curried_evaluator_full(h)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Value-only high poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_eval_quick) | Value-only high poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::English).init(cards!("9s","As","9d","Ks","Ah"));
/// let h2 = Hand::new(DeckType::English).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_high_eval_quick(&h1);
/// let v2 = ojp_high_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_high_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::HighHand,
            curried_evaluator_quick);
    }
    curried_evaluator_quick(h)
}

fn curried_evaluator_stripped_full(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::Stripped)
}

fn curried_evaluator_stripped_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::Stripped)
}

#[cfg(feature = "stripped-deck-tables")]
use crate::poker::stripped_deck_tables::*;

#[cfg(feature = "stripped-deck-tables")]
/// Quick lookup table evaluator
fn lookup_stripped(h: &Hand) -> u32 {
    let h = ojh_mp5_stripped(ojh_bitfield_64co(h.as_slice()).
        expect("should have been checked by this time"));
    STRIPPED_DECK_TABLE_1[h as usize] as u32
}

#[cfg(feature = "stripped-deck-tables")]
/// Full stripped deck poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_eval_full) | Full stripped deck poker hand evaluator
pub fn ojp_stripped_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 5 {
        return curried_evaluator_stripped_full(h);
    }
    let ec = if 5 == h.len() {
        lookup_stripped(h)
    } else {
        ojp_best_value_of(h, HandScale::Stripped, lookup_stripped)
    };
    let vv = STRIPPED_DECK_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::Stripped,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "stripped-deck-tables")]
/// Value-only stripped deck poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_eval_quick) | Value-only stripped deck poker hand evaluator
pub fn ojp_stripped_eval_quick(h: &Hand) -> u32 {
    if 5 == h.len() {
        return lookup_stripped(h);
    }
    if h.len() < 5 {
        return curried_evaluator_stripped_quick(h);
    }
    ojp_best_value_of(h, HandScale::Stripped, lookup_stripped)
}

#[cfg(not(feature = "stripped-deck-tables"))]
/// Full stripped deck poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_eval_full) | Full stripped deck poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Stripped).init(cards!("9s","As","9d","Ks","Ah"));
/// let v = ojp_stripped_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_stripped_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::Stripped,
            curried_evaluator_stripped_full);
    }
    curried_evaluator_stripped_full(h)
}

#[cfg(not(feature = "stripped-deck-tables"))]
/// Value-only stripped deck poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_eval_quick) | Value-only stripped deck poker hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::Stripped).init(cards!("9s","As","9d","Ks","Ah"));
/// let h2 = Hand::new(DeckType::Stripped).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_stripped_eval_quick(&h1);
/// let v2 = ojp_stripped_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_stripped_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::Stripped,
            curried_evaluator_stripped_quick);
    }
    curried_evaluator_stripped_quick(h)
}

fn curried_evaluator_bug_full(h: &Hand) -> Result<HandValue, OjError> {
    let Some(repl) = ojp_bug_replace_high(h) else {
        return ojp_default_eval_full(h, HandScale::HighHand);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    let mut v = ojp_default_eval_full(&dup, HandScale::HighHand)?;
    v.bug_is = repl.replacement;
    Ok(v)
}

fn curried_evaluator_bug_quick(h: &Hand) -> u32 {
    let Some(repl) = ojp_bug_replace_high(h) else {
        return ojp_default_eval_quick(h, HandScale::HighHand);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    ojp_default_eval_quick(&dup, HandScale::HighHand)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Full high with bug poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_bug_eval_full) | Full high hand with bug evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::OneJoker).init(cards!("9s","Jk","9d","Ks","Ah"));
/// let v = ojp_high_bug_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_high_bug_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::HighHand,
            curried_evaluator_bug_full);
    }
    curried_evaluator_bug_full(h)
}

#[cfg(not(feature = "high-hand-tables"))]
/// Value-only high hand with bug evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_high_bug_eval_quick) | Value-only high hand with bug evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::OneJoker).init(cards!("9s","As","9d","Ks","Jk"));
/// let h2 = Hand::new(DeckType::OneJoker).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_high_bug_eval_quick(&h1);
/// let v2 = ojp_high_bug_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_high_bug_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::HighHand,
            curried_evaluator_bug_quick);
    }
    curried_evaluator_bug_quick(h)
}

fn curried_evaluator_stripped_bug_full(h: &Hand) -> Result<HandValue, OjError> {
    let Some(repl) = ojp_bug_replace_high(h) else {
        return ojp_default_eval_full(h, HandScale::Stripped);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    let mut v = ojp_default_eval_full(&dup, HandScale::Stripped)?;
    v.bug_is = repl.replacement;
    Ok(v)
}

fn curried_evaluator_stripped_bug_quick(h: &Hand) -> u32 {
    let Some(repl) = ojp_bug_replace_high(h) else {
        return ojp_default_eval_quick(h, HandScale::Stripped);
    };
    let mut dup = *h;
    dup[repl.index as usize] = repl.replacement;
    ojp_default_eval_quick(&dup, HandScale::Stripped)
}

#[cfg(not(feature = "stripped-deck-tables"))]
/// Full stripped deck with bug poker hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_bug_eval_full) | Full stripped deck with bug evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::OneJoker).init(cards!("9s","Jk","9d","Ks","Ah"));
/// let v = ojp_stripped_bug_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// // Output: "[AsAh9s9dKs]: aces and nines with a king"
/// ```
pub fn ojp_stripped_bug_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 5 {
        return ojp_best_of(h, HandScale::HighHand,
            curried_evaluator_stripped_bug_full);
    }
    curried_evaluator_stripped_bug_full(h)
}

#[cfg(not(feature = "stripped-deck-tables"))]
/// Value-only stripped deck with bug evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_stripped_bug_eval_quick) | Value-only stripped deck with bug evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::OneJoker).init(cards!("9s","As","9d","Ks","Jk"));
/// let h2 = Hand::new(DeckType::OneJoker).init(cards!("9c","Ac","9h","Td","Ad"));
/// let v1 = ojp_stripped_bug_eval_quick(&h1);
/// let v2 = ojp_stripped_bug_eval_quick(&h2);
/// assert!(v1 < v2);   // king kicker beats ten kicker
/// ```
pub fn ojp_stripped_bug_eval_quick(h: &Hand) -> u32 {
    if h.len() > 5 {
        return ojp_best_value_of(h, HandScale::HighHand,
            curried_evaluator_stripped_bug_quick);
    }
    curried_evaluator_stripped_bug_quick(h)
}

/// If there's a bug in the hand, figure out what card it should
/// be, return it along with its index.
pub fn ojp_bug_replace_high(h: &Hand) -> Option<BugReplacement> {
    let scan = ojp_bug_scan(h);
    let index = scan.index?;

    if h.len() < 5 {    // partial hand, just ace
        return Some(BugReplacement::new(index as usize,
            ojp_ace_not_present(scan.ace_mask)));
    }
    let mut suit = Suit::None;
    let mut rank = Rank::None;

    if let Some(s) = FLUSH_PATTERNS.get(&scan.suit_mask) {
        suit = Suit::from_u8(*s);
    };
    if let Some(r) = STRAIGHT_PATTERNS.get(&scan.rank_mask) {
        rank = Rank::from_u8(*r);
    };
    Some(ojp_bug_replacement(rank, suit, &scan))
}

use lazy_static::lazy_static;

lazy_static! {
    /// After setting a bit for each suit in the hand, these are the patterns
    /// that indicate the rank needed for the bug to complete a flush
    pub static ref FLUSH_PATTERNS: std::collections::HashMap<u8, u8> = {
        let mut m = std::collections::HashMap::new();
        m.insert(0b00010, 1);
        m.insert(0b00100, 2);
        m.insert(0b01000, 3);
        m.insert(0b10000, 4);
        m
    };
}

lazy_static! {
    /// After setting a bit for each rank in the hand, these are the patterns
    /// that indicate the rank needed for the bug to complete a straight.
    pub static ref STRAIGHT_PATTERNS: std::collections::HashMap<u16, u8> = {
        let mut m = std::collections::HashMap::new();
        m.insert(0b0000000000111101, 6);
        m.insert(0b0000000010111001, 6);
        m.insert(0b0000000110110001, 6);
        m.insert(0b0000001110100001, 6);
        m.insert(0b0000000001111001, 7);
        m.insert(0b0000000101110001, 7);
        m.insert(0b0000001101100001, 7);
        m.insert(0b0000011101000001, 7);
        m.insert(0b0000000011110001, 8);
        m.insert(0b0000001011100001, 8);
        m.insert(0b0000011011000001, 8);
        m.insert(0b0000111010000001, 8);
        m.insert(0b0000000111100001, 9);
        m.insert(0b0000010111000001, 9);
        m.insert(0b0000110110000001, 9);
        m.insert(0b0010110100000001, 9);
        m.insert(0b0000001111000001, 10);
        m.insert(0b0000101110000001, 10);
        m.insert(0b0010101100000001, 10);
        m.insert(0b0110101000000001, 10);
        m.insert(0b1110100000000001, 10);
        m.insert(0b0000011110000001, 11);
        m.insert(0b0010011100000001, 11);
        m.insert(0b0110011000000001, 11);
        m.insert(0b1110010000000001, 11);
        m.insert(0b0000111100000001, 13);
        m.insert(0b0100111000000001, 13);
        m.insert(0b1100110000000001, 13);
        m.insert(0b0010111000000001, 14);
        m.insert(0b1010110000000001, 14);
        m.insert(0b0110110000000001, 15);
        m
    };
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_high() -> Result<(), OjError> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(cards!("2c","3h","7c","4d","5d"));
        let mut v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("3h","4s","7c","2h","5d"));
        let mut v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9d","3d","Qc","Kc","Th"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("Qc","9s","Ks","Td","3h"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("6h","2d","9c","6d","Ts"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4h","8c","8d","Ad","4c"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","7d","5c","5s","Kd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ah","5s","3s","4s","2d"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","9h","8d","Ts","6s"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9c","7d","Tc","6c","8h"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","As","Js","Th","Qh"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5d","Td","8d","4d","Qd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","As","7d","7h","Ah"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3c","3s","3d","3h","Kd"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ad","5d","3d","2d","4d"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_high_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_stripped() -> Result<(), OjError> {
        let deck = Deck::new_by_name("manila");
        let mut hand= deck.new_hand();
        let mut best: u32 = HAND_VALUE_WORST;

        hand.set(cards!("8c","9h","7c","Jd","Kd"));
        let mut v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("9h","8s","7c","Kh","Jd"));
        let mut v2 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9d","7d","Qc","Kc","Th"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);

        hand.set(cards!("Qc","9s","Ks","Td","7h"));
        v2 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7h","Qd","9c","7d","Ts"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Pair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Th","8c","8d","Ad","Tc"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoPair as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("9h","7d","9c","9s","Kd"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","9h","8d","Ts","Js"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9c","7d","Tc","Jc","8h"));
        v2 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","As","Js","Th","Qh"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7s","7h","Ac","7d","Ad"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","As","7d","7h","Ah"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","Td","8d","Ad","Qd"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("8c","8s","8d","8h","Kd"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ts","Qs","9s","Js","Ks"));
        v1 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Qh","9h","Kh","Th","Jh"));
        v2 = ojp_stripped_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }
}
