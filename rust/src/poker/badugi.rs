//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Badugi) | Badugi hand values

use crate::errors::*;
use crate::utils::*;
use crate::cards::*;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_full_name) | Describe Badugi hands
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("9s","7c","5h","8d"));
/// let v = ojp_badugi_eval_full(&hand).unwrap();
/// println!("{}", ojp_badugi_full_name(&v));
/// // Output: "four-card nine, eight, seven, five"
/// ```
pub fn ojp_badugi_full_name(v: &HandValue) -> String {
    macro_rules! sng {
        ($x:literal) => { v.hand[$x as usize].rank().name() }
    }

    match HandLevel::from_u8(v.level) {
        HandLevel::FourCard => {
            if Rank::Four == v.hand[0].rank() {
                String::from("perfect badugi")
            } else {
                format!("four-card {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3))
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

// Return HAND_VALUE_WORST if not a badugi, or the 32-bit hand value
fn badugi_value(cards: &[Card]) -> u32 {
    let mut suits: u32 = 0;
    let mut ranks: u32 = 0;

    let mut v: u32 = 0;
    for c in cards {
        let s = c.suit() as u32;
        let r = c.rank() as u32;

        if 0 != (suits & (1 << s)) || 0 != (ranks & (1 << r)) {
            return HAND_VALUE_WORST;
        }
        suits |= 1 << s;
        ranks |= 1 << r;

        v <<= 4;
        v += c.rank() as u32;
    }
    v + (4 - cards.len() as u32) * HAND_LEVEL_MULTIPLIER
}

enum BadugiEvaluatorState {
    Initial,
    NotFour,
    NotThree,
    NotTwo
}

fn badugi_evaluator_full_common(hand: &Hand, g: HandScale)
->  Result<HandValue, OjError> {
    let mut h = *hand;
    oj_sort(h.as_mut_slice());
    let mut state = BadugiEvaluatorState::Initial;

    loop {
        match state {
            BadugiEvaluatorState::Initial => {
                if h.len() < 4 {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                let v= badugi_value(h.as_slice());
                if HAND_VALUE_WORST == v {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                return Ok(HandValue::new_with_value(h, g,
                    HandLevel::FourCard, v));
            },
            BadugiEvaluatorState::NotFour => {
                if h.len() < 3 {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                let mut best_hand = Hand::default();
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h3 in h.combinations(3) {
                    v = badugi_value(h3.as_slice());
                    if v < best_value {
                        best_value = v;
                        best_hand = h3;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                for i in 0..3 {
                    let j = h.index_of(best_hand[i]);
                    if let Some(j) = j {
                        if i != j {
                            h.cards.swap(i, j);
                        }
                    }
                }
                h.truncate(3);
                return Ok(HandValue::new_with_value(h, g,
                    HandLevel::ThreeCard, best_value));
            },
            BadugiEvaluatorState::NotThree => {
                if h.len() < 2 {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                let mut best_hand = Hand::default();
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h2 in h.combinations(2) {
                    v = badugi_value(h2.as_slice());
                    if v < best_value {
                        best_value = v;
                        best_hand = h2;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                for i in 0..2 {
                    let j = h.index_of(best_hand[i]);
                    if let Some(j) = j {
                        if i != j {
                            h.cards.swap(i, j);
                        }
                    }
                }
                h.truncate(2);
                return Ok(HandValue::new_with_value(h, g,
                HandLevel::TwoCard, best_value));
            },
            BadugiEvaluatorState::NotTwo => {
                let mut least = 0;

                for i in 1..h.len() {
                    if h[i].rank() < h[least].rank() {
                        least = i;
                    }
                }
                if 0 != least {
                    h.cards.swap(0, least);
                }
                h.truncate(1);
                return Ok(HandValue::new_with_value(h, g,
                    HandLevel::OneCard,
                    3 * HAND_LEVEL_MULTIPLIER + h[0].rank() as u32));
            }
        }
    }
}

fn badugi_evaluator_quick_common(hand: &Hand) -> u32 {
    let mut h = *hand;
    oj_sort(h.as_mut_slice());
    let mut state = BadugiEvaluatorState::Initial;

    loop {
        match state {
            BadugiEvaluatorState::Initial => {
                if h.len() < 4 {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                let v= badugi_value(h.as_slice());
                if HAND_VALUE_WORST == v {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                return v;
            },
            BadugiEvaluatorState::NotFour => {
                if h.len() < 3 {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h3 in h.combinations(3) {
                    v = badugi_value(h3.as_slice());
                    if v < best_value {
                        best_value = v;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotThree;
                    continue;
                }
                return best_value;
            },
            BadugiEvaluatorState::NotThree => {
                if h.len() < 2 {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                let mut best_value = HAND_VALUE_WORST;
                let mut v: u32;

                for h2 in h.combinations(2) {
                    v = badugi_value(h2.as_slice());
                    if v < best_value {
                        best_value = v;
                    }
                }
                if HAND_VALUE_WORST == best_value {
                    state = BadugiEvaluatorState::NotTwo;
                    continue;
                }
                return best_value;
            },
            BadugiEvaluatorState::NotTwo => {
                let mut least = 0;

                for i in 1..h.len() {
                    if h[i].rank() < h[least].rank() {
                        least = i;
                    }
                }
                return 3 * HAND_LEVEL_MULTIPLIER + h[least].rank() as u32;
            }
        }
    }
}

fn badugi_evaluator_full(hand: &Hand) -> Result<HandValue, OjError> {
    debug_assert!(ojp_valid_hand_for_game(hand, HandScale::Badugi));
    badugi_evaluator_full_common(hand, HandScale::Badugi)
}

fn badugi_evaluator_quick(hand: &Hand) -> u32 {
    debug_assert!(ojp_valid_hand_for_game(hand, HandScale::Badugi));
    badugi_evaluator_quick_common(hand)
}

fn badeucy_evaluator_full(hand: &Hand) -> Result<HandValue, OjError> {
    debug_assert!(ojp_valid_hand_for_game(hand, HandScale::Badeucy));
    badugi_evaluator_full_common(hand, HandScale::Badeucy)
}

fn badeucy_evaluator_quick(hand: &Hand) -> u32 {
    debug_assert!(ojp_valid_hand_for_game(hand, HandScale::Badeucy));
    badugi_evaluator_quick_common(hand)
}

#[cfg(feature = "badugi-tables")]
use crate::poker::badugi_tables::*;

#[cfg(feature = "badugi-tables")]
/// Quick lookup table evaluator
fn lookup_badugi(h: &Hand) -> u32 {
    let hash = ojh_mp4_low(ojh_bitfield_64co(h.as_slice()).
    expect("should have been checked by this time"));
    BADUGI_TABLE_1[hash as usize] as u32
}

#[cfg(feature = "badugi-tables")]
/// Quick lookup table evaluator
fn lookup_badeucy(h: &Hand) -> u32 {
    let hash = ojh_mp4_english(ojh_bitfield_64co(h.as_slice()).
    expect("should have been checked by this time"));
    BADUGI_TABLE_1[hash as usize] as u32
}

#[cfg(feature = "badugi-tables")]
/// Full badugi poker hand evaluator using tables
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_full) | Full Badugi hand evaluator
pub fn ojp_badugi_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 4 {
        return badugi_evaluator_full(h);
    }
    let ec = if 4 == h.len() {
        lookup_badugi(h)
    } else {
        ojp_best_value_of(h, HandScale::Badugi, lookup_badugi)
    };
    let vv = BADUGI_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::Badugi,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "badugi-tables")]
/// Value-only badugi poker hand evaluator using tables
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_quick) | Quick Badugi hand evaluator
pub fn ojp_badugi_eval_quick(h: &Hand) -> u32 {
    if 4 == h.len(){
        return lookup_badugi(h);
    }
    if h.len() < 4 {
        return badugi_evaluator_quick(h);
    }
    ojp_best_value_of(h, HandScale::Badugi, lookup_badugi)
}

#[cfg(feature = "badugi-tables")]
/// Full badeucy poker hand evaluator using tables
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_full) | Full Badeucy hand evaluator
pub fn ojp_badeucy_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() < 4 {
        return badeucy_evaluator_full(h);
    }
    let ec = if 4 == h.len() {
        lookup_badeucy(h)
    } else {
        ojp_best_value_of(h, HandScale::Badeucy, lookup_badeucy)
    };
    let vv = BADEUCY_TABLE_2[ec as usize];
    let mut v = HandValue::new_with_value(*h, HandScale::Badeucy,
        vv.0, ec as u32);
    v.order_for_display(&vv.1);
    Ok(v)
}

#[cfg(feature = "badugi-tables")]
/// Value-only badeucy poker hand evaluator using tables
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_quick) | Quick Badeucy hand evaluator
pub fn ojp_badeucy_eval_quick(h: &Hand) -> u32 {
    if 4 == h.len(){
        return lookup_badeucy(h);
    }
    if h.len() < 4 {
        return badeucy_evaluator_quick(h);
    }
    ojp_best_value_of(h, HandScale::Badeucy, lookup_badeucy)
}

#[cfg(not(feature = "badugi-tables"))]
/// Full badugi hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_full) | Full Badugi hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::Low).init(cards!("9s","7c","5h","8d"));
/// let v = ojp_badugi_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// ```
pub fn ojp_badugi_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 4 {
        return ojp_best_of(h, HandScale::Badugi,
            badugi_evaluator_full);
    }
    badugi_evaluator_full(h)
}

#[cfg(not(feature = "badugi-tables"))]
/// Value-only badugi hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_quick) | Quick Badugi hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::Low).init(cards!("9s","7c","5h","8d"));
/// let h2 = Hand::new(DeckType::Low).init(cards!("2s","Ac","3h","4c"));
/// let v1 = ojp_badugi_eval_quick(&h1);
/// let v2 = ojp_badugi_eval_quick(&h2);
/// assert!(v1 < v2);   // four-card nine beats three-card trey
/// ```
pub fn ojp_badugi_eval_quick(h: &Hand) -> u32 {
    if h.len() > 4 {
        return ojp_best_value_of(h, HandScale::Badugi,
            badugi_evaluator_quick);
    }
    badugi_evaluator_quick(h)
}

#[cfg(not(feature = "badugi-tables"))]
/// Full badeucy hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_full) | Full Badeucy hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let hand = Hand::new(DeckType::English).init(cards!("9s","7c","5h","8d"));
/// let v = ojp_badeucy_eval_full(&hand).unwrap();
/// println!("[{}]: {}", v.hand, v.full_name());
/// ```
pub fn ojp_badeucy_eval_full(h: &Hand) -> Result<HandValue, OjError> {
    if h.len() > 4 {
        return ojp_best_of(h, HandScale::Badeucy,
            badeucy_evaluator_full);
    }
    badeucy_evaluator_full(h)
}

#[cfg(not(feature = "badugi-tables"))]
/// Value-only badeucy hand evaluator
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_quick) | Quick Badeucy hand evaluator
/// ```rust
/// use onejoker::*;
///
/// let h1 = Hand::new(DeckType::English).init(cards!("9s","7c","5h","8d"));
/// let h2 = Hand::new(DeckType::English).init(cards!("2s","Ac","3h","4d"));
/// let v1 = ojp_badeucy_eval_quick(&h1);
/// let v2 = ojp_badeucy_eval_quick(&h2);
/// assert!(v1 < v2);   // four-card nine beats four-card ace
/// ```
pub fn ojp_badeucy_eval_quick(h: &Hand) -> u32 {
    if h.len() > 4 {
        return ojp_best_value_of(h, HandScale::Badeucy,
            badeucy_evaluator_quick);
    }
    badeucy_evaluator_quick(h)
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_badugi() -> Result<(), OjError> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand().init(cards!("Ks","Kh","Kd","Kc"));
        let mut best: u32 = HAND_VALUE_WORST;

        let mut v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ad","Ac","Ah","As"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::LowAce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3d","9d","Ad","Kd"));
        let mut v2 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.hand[0].rank(), Rank::LowAce);

        hand.set(cards!("Jd","5c","Jh","7c"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","5d","Jc","Kc"));
        v2 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.hand[0].rank(), Rank::Jack);
        assert_eq!(v1.hand[1].rank(), Rank::Five);

        hand.set(cards!("7d","4c","7s","9c"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Seven);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2h","Tc","2s","5h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Ten);
        assert_eq!(v1.hand[1].rank(), Rank::Five);
        assert_eq!(v1.hand[2].rank(), Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","3c","9d","9h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Nine);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Tc","Jd","Kh","Qs"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert_eq!(v1.hand[1].rank(), Rank::Queen);
        assert_eq!(v1.hand[2].rank(), Rank::Jack);
        assert_eq!(v1.hand[3].rank(), Rank::Ten);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3c","2d","4s","5h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Five);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert_eq!(v1.hand[3].rank(), Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","3d","4s","2h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Four);
        assert_eq!(v1.hand[1].rank(), Rank::Trey);
        assert_eq!(v1.hand[2].rank(), Rank::Deuce);
        assert_eq!(v1.hand[3].rank(), Rank::LowAce);
        assert!(v1.value < best);

        Ok(())
    }

    #[test]
    fn test_hand_evaluator_badeucy() -> Result<(), OjError> {
        let deck = Deck::new_by_name("english");
        let mut hand= deck.new_hand().init(cards!("As","Ah","Ad","Ac"));
        let mut best: u32 = HAND_VALUE_WORST;

        let mut v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Ace);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","Kc","Kh","Ks"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3d","9d","Ad","Kd"));
        let mut v2 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v2.hand[0].rank(), Rank::Trey);

        hand.set(cards!("Jd","5c","Jh","7c"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kd","5d","Jc","Kc"));
        v2 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.hand[0].rank(), Rank::Jack);
        assert_eq!(v1.hand[1].rank(), Rank::Five);

        hand.set(cards!("7d","4c","7s","9c"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Seven);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2h","Tc","2s","5h"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Ten);
        assert_eq!(v1.hand[1].rank(), Rank::Five);
        assert_eq!(v1.hand[2].rank(), Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","3c","9d","9h"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Nine);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Ac","3d","4s","2h"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Ace);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert_eq!(v1.hand[3].rank(), Rank::Deuce);
        assert!(v1.value < best);

        hand.set(cards!("Tc","Jd","Kh","Qs"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert_eq!(v1.hand[1].rank(), Rank::Queen);
        assert_eq!(v1.hand[2].rank(), Rank::Jack);
        assert_eq!(v1.hand[3].rank(), Rank::Ten);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3c","2d","4s","5h"));
        v1 = ojp_badeucy_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard as u8);
        assert_eq!(v1.hand[0].rank(), Rank::Five);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert_eq!(v1.hand[3].rank(), Rank::Deuce);
        assert!(v1.value < best);

        Ok(())
    }
}
