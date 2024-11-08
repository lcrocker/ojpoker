//! [wiki](https://github.com/lcrocker/ojpoker/wiki/DeuceToSeven) | Deuce-to-seven "Kansas City" low poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_full_name) | Describe deuce-to-seven low hands
pub fn ojp_deuce_to_seven_full_name(v: &HandValue) -> String {
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
            format!("{}, {}, {}, {}, {}", sng!(0), sng!(1), sng!(2), sng!(3), sng!(4))
        },
        _ => String::from("unknown hand"),
    }
}

fn curried_evaluator_full(h: &Hand) -> Result<HandValue, OjError> {
    ojp_default_eval_full(h, HandScale::DeuceToSeven)
}

fn curried_evaluator_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::DeuceToSeven)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "deuce-to-seven-tables")] {
        use crate::poker::deuce_to_seven_tables::*;

        /// Quick lookup table evaluator
        fn lookup_deuce_to_seven(h: &Hand) -> u32 {
            let h = ojh_mp5_english(ojh_bitfield_64co(h.as_slice()).
                expect("should have been checked by this time"));
            DEUCE_TO_SEVEN_TABLE_1[h as usize] as u32
        }

        /// Full high poker hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_full) | Deuce-to-seven full evaluator
        pub fn ojp_deuce_to_seven_eval_full(h: &Hand) -> Result<HandValue, OjError> {
            if h.len() < 5 {
                return curried_evaluator_full(h);
            }
            let ec = if 5 == h.len() {
                lookup_deuce_to_seven(h)
            } else {
                ojp_best_value_of(h, HandScale::DeuceToSeven, lookup_deuce_to_seven)
            };
            let vv = DEUCE_TO_SEVEN_TABLE_2[ec as usize];
            let mut v = HandValue::new_with_value(*h, HandScale::DeuceToSeven,
                vv.0, ec as u32);
            v.order_for_display(&vv.1);
            Ok(v)
        }

        /// Value-only high poker hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_quick) | Deuce-to-seven quick evaluator
        pub fn ojp_deuce_to_seven_eval_quick(h: &Hand) -> u32 {
            if 5 == h.len(){
                return lookup_deuce_to_seven(h)
            }
            if h.len() < 5 {
                return curried_evaluator_quick(h);
            }
            ojp_best_value_of(h, HandScale::DeuceToSeven, lookup_deuce_to_seven)
        }
    } else {
        /// Full high poker hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_full) | Deuce-to-seven full evaluator
        pub fn ojp_deuce_to_seven_eval_full(h: &Hand) -> Result<HandValue, OjError> {
            if h.len() > 5 {
                return ojp_best_of(h, HandScale::DeuceToSeven,
                    curried_evaluator_full);
            }
            curried_evaluator_full(h)
        }

        /// Value-only high poker hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_deuce_to_seven_eval_quick) | Deuce-to-seven quick evaluator
        pub fn ojp_deuce_to_seven_eval_quick(h: &Hand) -> u32 {
            if h.len() > 5 {
                return ojp_best_value_of(h, HandScale::DeuceToSeven,
                    curried_evaluator_quick);
            }
            curried_evaluator_quick(h)
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
    fn test_hand_evaluator_deuce_to_seven() -> Result<(), OjError> {
        let deck = Deck::new_by_name("poker");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFFFFFF;

        hand.set(cards!("8c","Jc","9c","7c","Tc"));
        let mut v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::StraightFlush as u8);

        hand.set(cards!("Td","7d","8d","9d","Jd"));
        let mut v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4c","Jc","4s","4d","4h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Quads as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Kc","Kd","8d","8c","Kh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("8c","8h","Kc","8d","Ks"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::FullHouse as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("4s","As","5s","3s","2s"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","Th","7h","4h","Jh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Flush as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("Qd","Kh","Ac","Jc","Td"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("7d","9s","8d","Ts","Jh"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Straight as u8);

        hand.set(cards!("9h","7d","Ts","Jc","8h"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2h","5s","2c","2d","9c"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::Trips as u8);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5s","Ts","Ks","Td","5c"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("3h","2s","9c","3s","8h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("2d","Ah","5s","3s","4s"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(HandLevel::from_u8(v1.level), HandLevel::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("8d","3d","4c","Kc","Th"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);

        hand.set(cards!("4c","8s","Ks","Td","3h"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(cards!("5h","2s","3h","7s","4h"));
        v1 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1.level, HandLevel::NoPair as u8);

        hand.set(cards!("3s","4h","7d","2d","5d"));
        v2 = ojp_deuce_to_seven_eval_full(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }
}

