//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Badugi) | Badugi hand values

use crate::error::Result;
use crate::cards::*;
use crate::utils::oj_sort;
use crate::poker::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_full_name) | Describe Badugi hand
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::Low).init(hand!("9s","7c","5h","8d"));
/// let v = Scale::Badugi.eval(&hand).unwrap();
/// println!("{}", v.full_name());
/// // Output: "four-card nine, eight, seven, five"
/// ```
pub fn ojp_badugi_full_name(d: &HandDescription) -> String {
    macro_rules! sng {
        ($x:literal) => { d.hand[$x as usize].rank().name() }
    }
    match d.level {
        HandLevel::FourCard => {
            if Rank::Four == d.hand[0].rank() {
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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_4_full) | Full 4-card Badugi hand evaluator
#[cfg(not(feature = "badugi-tables"))]
fn badugi_eval_4_full(h: &Hand) -> Result<HandDescription> {
    ojp_badugi_reference_evaluator_full_common(h, Scale::Badugi)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_4_quick) | Value-only  4-card Badugi evaluator
#[cfg(not(feature = "badugi-tables"))]
fn badugi_eval_4_quick(hand: &Hand) -> HandValue {
    ojp_badugi_reference_evaluator_quick(hand)
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_full) | Full Badugi hand evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let hand = Hand::new(DeckType::Low).init(hand!("9s","7c","5h","8d"));
/// let v = Scale::Badugi.eval(&hand).unwrap();
/// println!("[{}]: {}", v, v.full_name());
/// ```
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_badugi_eval_full(h: &Hand) -> Result<HandDescription> {
    debug_assert!(Scale::Badugi.valid_hand(h));

    match h.len() {
        ..4 => ojp_badugi_reference_evaluator_full_common(h, Scale::Badugi),
        4 => badugi_eval_4_full(h),
        5.. => ojp_best_of(h, Scale::Badugi, badugi_eval_4_full),
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_quick) | Quick Badugi hand evaluator
/// ```rust
/// use onejoker::prelude::*;
///
/// let h1 = Hand::new(DeckType::Low).init(hand!("9s","7c","5h","8d"));
/// let h2 = Hand::new(DeckType::Low).init(hand!("2s","Ac","3h","4c"));
/// let v1 = Scale::Badugi.eval_quick(&h1);
/// let v2 = Scale::Badugi.eval_quick(&h2);
/// assert!(v1 < v2);   // four-card nine beats three-card trey
/// ```
#[cfg(not(feature = "badugi-tables"))]
pub fn ojp_badugi_eval_quick(h: &Hand) -> u32 {
    match h.len() {
        ..4 => ojp_badugi_reference_evaluator_quick(h),
        4 => badugi_eval_4_quick(h),
        5.. => ojp_best_value_of(h, Scale::Badugi, badugi_eval_4_quick),
    }
}

// fn badeucy_evaluator_full(hand: &Hand) -> Result<HandDescription> {
//     ojp_badugi_reference_evaluator_full_common(hand, Scale::Badeucy)
// }

// fn badeucy_evaluator_quick(hand: &Hand) -> HandValue {
//     ojp_badugi_reference_evaluator_quick_common(hand)
// }

// #[cfg(feature = "badugi-tables")]
// use crate::poker::badugi_tables::*;

// #[cfg(feature = "badugi-tables")]
// /// Quick lookup table evaluator
// fn lookup_badugi(h: &Hand) -> u32 {
//     let hash = ojh_mp4_low(ojh_bitfield_64co(h.as_slice()).
//     expect("should have been checked by this time"));
//     BADUGI_TABLE_1[hash as usize] as u32
// }

// #[cfg(feature = "badugi-tables")]
// /// Quick lookup table evaluator
// fn lookup_badeucy(h: &Hand) -> u32 {
//     let hash = ojh_mp4_english(ojh_bitfield_64co(h.as_slice()).
//     expect("should have been checked by this time"));
//     BADUGI_TABLE_1[hash as usize] as u32
// }

// #[cfg(feature = "badugi-tables")]
// /// Full badugi poker hand evaluator using tables
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_full) | Full Badugi hand evaluator
// pub fn ojp_badugi_eval_full(h: &Hand) -> Result<HandValue, OjError> {
//     if h.len() < 4 {
//         return badugi_evaluator_full(h);
//     }
//     let ec = if 4 == h.len() {
//         lookup_badugi(h)
//     } else {
//         ojp_best_value_of(h, Scale::Badugi, lookup_badugi)
//     };
//     let vv = BADUGI_TABLE_2[ec as usize];
//     let mut v = HandValue::new_with_value(*h, Scale::Badugi,
//         vv.0, ec as u32);
//     v.order_for_display(&vv.1);
//     Ok(v)
// }

// #[cfg(feature = "badugi-tables")]
// /// Value-only badugi poker hand evaluator using tables
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_eval_quick) | Quick Badugi hand evaluator
// pub fn ojp_badugi_eval_quick(h: &Hand) -> u32 {
//     if 4 == h.len(){
//         return lookup_badugi(h);
//     }
//     if h.len() < 4 {
//         return badugi_evaluator_quick(h);
//     }
//     ojp_best_value_of(h, Scale::Badugi, lookup_badugi)
// }

// #[cfg(feature = "badugi-tables")]
// /// Full badeucy poker hand evaluator using tables
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_full) | Full Badeucy hand evaluator
// pub fn ojp_badeucy_eval_full(h: &Hand) -> Result<HandValue, OjError> {
//     if h.len() < 4 {
//         return badeucy_evaluator_full(h);
//     }
//     let ec = if 4 == h.len() {
//         lookup_badeucy(h)
//     } else {
//         ojp_best_value_of(h, Scale::Badeucy, lookup_badeucy)
//     };
//     let vv = BADEUCY_TABLE_2[ec as usize];
//     let mut v = HandValue::new_with_value(*h, Scale::Badeucy,
//         vv.0, ec as u32);
//     v.order_for_display(&vv.1);
//     Ok(v)
// }

// #[cfg(feature = "badugi-tables")]
// /// Value-only badeucy poker hand evaluator using tables
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_quick) | Quick Badeucy hand evaluator
// pub fn ojp_badeucy_eval_quick(h: &Hand) -> u32 {
//     if 4 == h.len(){
//         return lookup_badeucy(h);
//     }
//     if h.len() < 4 {
//         return badeucy_evaluator_quick(h);
//     }
//     ojp_best_value_of(h, Scale::Badeucy, lookup_badeucy)
// }

// #[cfg(not(feature = "badugi-tables"))]
// /// Full badeucy hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_full) | Full Badeucy hand evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let hand = Hand::new(DeckType::English).init(hand!("9s","7c","5h","8d"));
// /// let v = ojp_badeucy_eval_full(&hand).unwrap();
// /// println!("[{}]: {}", v.hand, v.full_name());
// /// ```
// pub fn ojp_badeucy_eval_full(h: &Hand) -> Result<HandValue, OjError> {
//     if h.len() > 4 {
//         return ojp_best_of(h, Scale::Badeucy,
//             badeucy_evaluator_full);
//     }
//     badeucy_evaluator_full(h)
// }

// #[cfg(not(feature = "badugi-tables"))]
// /// Value-only badeucy hand evaluator
// /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badeucy_eval_quick) | Quick Badeucy hand evaluator
// /// ```rust
// /// use onejoker::*;
// ///
// /// let h1 = Hand::new(DeckType::English).init(hand!("9s","7c","5h","8d"));
// /// let h2 = Hand::new(DeckType::English).init(hand!("2s","Ac","3h","4d"));
// /// let v1 = ojp_badeucy_eval_quick(&h1);
// /// let v2 = ojp_badeucy_eval_quick(&h2);
// /// assert!(v1 < v2);   // four-card nine beats four-card ace
// /// ```
// pub fn ojp_badeucy_eval_quick(h: &Hand) -> u32 {
//     if h.len() > 4 {
//         return ojp_best_value_of(h, Scale::Badeucy,
//             badeucy_evaluator_quick);
//     }
//     badeucy_evaluator_quick(h)
// }

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

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_reference_evaluator_full_common) | Common Badugi/Badeucy full evaluator
pub fn ojp_badugi_reference_evaluator_full_common(hand: &Hand, g: Scale)
->  Result<HandDescription> {
    let mut h = *hand;
    oj_sort(&mut h[..]);
    let mut state = BadugiEvaluatorState::Initial;

    loop {
        match state {
            BadugiEvaluatorState::Initial => {
                if h.len() < 4 {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                let v= badugi_value(&h[..]);
                if HAND_VALUE_WORST == v {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                return HandDescriptionBuilder::new(&h, g)
                    .with_level(HandLevel::FourCard)
                    .with_value(v).truncate(4).complete();
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
                    v = badugi_value(&h3[..]);
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
                return HandDescriptionBuilder::new(&h, g)
                    .with_level(HandLevel::ThreeCard)
                    .with_value(best_value).truncate(3).complete();
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
                    v = badugi_value(&h2[..]);
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
                return HandDescriptionBuilder::new(&h, g)
                    .with_level(HandLevel::TwoCard)
                    .with_value(best_value).truncate(2).complete();
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
                return HandDescriptionBuilder::new(&h, g)
                    .with_level(HandLevel::OneCard)
                    .with_value(3 * HAND_LEVEL_MULTIPLIER + h[0].rank() as u32)
                    .truncate(1).complete();
            }
        }
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_badugi_reference_evaluator_quick) | Badugi/Badeucy quick evaluator
pub fn ojp_badugi_reference_evaluator_quick(hand: &Hand) -> HandValue {
    let mut h = *hand;
    oj_sort(&mut h[..]);
    let mut state = BadugiEvaluatorState::Initial;

    loop {
        match state {
            BadugiEvaluatorState::Initial => {
                if h.len() < 4 {
                    state = BadugiEvaluatorState::NotFour;
                    continue;
                }
                let v= badugi_value(&h[..]);
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
                    v = badugi_value(&h3[..]);
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
                    v = badugi_value(&h2[..]);
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

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_badugi() -> Result<()> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand().init(hand!("Ks","Kh","Kd","Kc"));
        let mut best: u32 = HAND_VALUE_WORST;

        let mut v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ad","Ac","Ah","As"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::OneCard);
        assert_eq!(v1.hand[0].rank(), Rank::LowAce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("3d","9d","Ad","Kd"));
        let mut v2 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.hand[0].rank(), Rank::LowAce);

        hand.set(hand!("Jd","5c","Jh","7c"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Kd","5d","Jc","Kc"));
        v2 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.hand[0].rank(), Rank::Jack);
        assert_eq!(v1.hand[1].rank(), Rank::Five);

        hand.set(hand!("7d","4c","7s","9c"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::TwoCard);
        assert_eq!(v1.hand[0].rank(), Rank::Seven);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("2h","Tc","2s","5h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard);
        assert_eq!(v1.hand[0].rank(), Rank::Ten);
        assert_eq!(v1.hand[1].rank(), Rank::Five);
        assert_eq!(v1.hand[2].rank(), Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("4s","3c","9d","9h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::ThreeCard);
        assert_eq!(v1.hand[0].rank(), Rank::Nine);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Tc","Jd","Kh","Qs"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard);
        assert_eq!(v1.hand[0].rank(), Rank::King);
        assert_eq!(v1.hand[1].rank(), Rank::Queen);
        assert_eq!(v1.hand[2].rank(), Rank::Jack);
        assert_eq!(v1.hand[3].rank(), Rank::Ten);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("3c","2d","4s","5h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard);
        assert_eq!(v1.hand[0].rank(), Rank::Five);
        assert_eq!(v1.hand[1].rank(), Rank::Four);
        assert_eq!(v1.hand[2].rank(), Rank::Trey);
        assert_eq!(v1.hand[3].rank(), Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(hand!("Ac","3d","4s","2h"));
        v1 = ojp_badugi_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FourCard);
        assert_eq!(v1.hand[0].rank(), Rank::Four);
        assert_eq!(v1.hand[1].rank(), Rank::Trey);
        assert_eq!(v1.hand[2].rank(), Rank::Deuce);
        assert_eq!(v1.hand[3].rank(), Rank::LowAce);
        assert!(v1.value < best);

        Ok(())
    }

    // #[test]
    // fn test_hand_evaluator_badeucy() -> Result<()> {
    //     let deck = Deck::new_by_name("english");
    //     let mut hand= deck.new_hand().init(hand!("As","Ah","Ad","Ac"));
    //     let mut best: u32 = HAND_VALUE_WORST;

    //     let mut v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::OneCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Ace);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Kd","Kc","Kh","Ks"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::OneCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::King);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("3d","9d","Ad","Kd"));
    //     let mut v2 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v2.hand[0].rank(), Rank::Trey);

    //     hand.set(hand!("Jd","5c","Jh","7c"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::TwoCard);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Kd","5d","Jc","Kc"));
    //     v2 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1, v2);
    //     assert_eq!(v1.hand[0].rank(), Rank::Jack);
    //     assert_eq!(v1.hand[1].rank(), Rank::Five);

    //     hand.set(hand!("7d","4c","7s","9c"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::TwoCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Seven);
    //     assert_eq!(v1.hand[1].rank(), Rank::Four);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("2h","Tc","2s","5h"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::ThreeCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Ten);
    //     assert_eq!(v1.hand[1].rank(), Rank::Five);
    //     assert_eq!(v1.hand[2].rank(), Rank::Deuce);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("4s","3c","9d","9h"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::ThreeCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Nine);
    //     assert_eq!(v1.hand[1].rank(), Rank::Four);
    //     assert_eq!(v1.hand[2].rank(), Rank::Trey);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("Ac","3d","4s","2h"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FourCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Ace);
    //     assert_eq!(v1.hand[1].rank(), Rank::Four);
    //     assert_eq!(v1.hand[2].rank(), Rank::Trey);
    //     assert_eq!(v1.hand[3].rank(), Rank::Deuce);
    //     assert!(v1.value < best);

    //     hand.set(hand!("Tc","Jd","Kh","Qs"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FourCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::King);
    //     assert_eq!(v1.hand[1].rank(), Rank::Queen);
    //     assert_eq!(v1.hand[2].rank(), Rank::Jack);
    //     assert_eq!(v1.hand[3].rank(), Rank::Ten);
    //     assert!(v1.value < best);
    //     best = v1.value;

    //     hand.set(hand!("3c","2d","4s","5h"));
    //     v1 = ojp_badeucy_eval_full(&hand)?;
    //     assert_eq!(v1.level, HandLevel::FourCard);
    //     assert_eq!(v1.hand[0].rank(), Rank::Five);
    //     assert_eq!(v1.hand[1].rank(), Rank::Four);
    //     assert_eq!(v1.hand[2].rank(), Rank::Trey);
    //     assert_eq!(v1.hand[3].rank(), Rank::Deuce);
    //     assert!(v1.value < best);

    //     Ok(())
    // }
}
