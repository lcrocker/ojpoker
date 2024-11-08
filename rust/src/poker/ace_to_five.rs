//! [wiki](https://github.com/lcrocker/ojpoker/wiki/AceToFive) | Ace-to-five low hand values

use crate::errors::*;
use crate::cards::*;
use crate::poker::*;

/// Full English name of hand, e.g. "aces and fours with a jack".
/// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_full_name) | Describe ace-to-five hand
pub fn ojp_ace_to_five_full_name(v: &HandValue) -> String {
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
        HandLevel::Quads => {
            format!("four {} with {} {}", plr!(0), art!(4), sng!(4))
        },
        HandLevel::FullHouse => {
            format!("{} full of {}", plr!(0), plr!(3))
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
    ojp_default_eval_full(h, HandScale::AceToFive)
}

fn curried_evaluator_quick(h: &Hand) -> u32 {
    ojp_default_eval_quick(h, HandScale::AceToFive)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ace-to-five-tables")] {
        use crate::poker::ace_to_five_tables::*;

        /// Quick lookup table evaluator
        fn lookup_ace_to_five(h: &Hand) -> u32 {
            let h = ojh_positional_32cs_mp5_low(h.as_slice()).
            expect("should have been checked by this time");
            ACE_TO_FIVE_TABLE_1[h as usize] as u32
        }

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

        /// Value-only high poker hand evaluator
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
    } else {
        /// Full ace-to-five hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_full) | Ace-to-five full evaluator
        pub fn ojp_ace_to_five_eval_full(h: &Hand) -> Result<HandValue, OjError> {
            if h.len() > 5 {
                return ojp_best_of(h, HandScale::AceToFive,
                    curried_evaluator_full);
            }
            curried_evaluator_full(h)
        }

        /// Value-only ace-to-five hand evaluator
        /// [wiki](https://github.com/lcrocker/ojpoker/wiki/ojp_ace_to_five_eval_quick) | Ace-to-five quick evaluator
        pub fn ojp_ace_to_five_eval_quick(h: &Hand) -> u32 {
            if h.len() > 5 {
                return ojp_best_value_of(h, HandScale::AceToFive,
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
    fn test_hand_evaluator_ace_to_five() -> Result<(), OjError> {
        let deck = Deck::new_by_name("low");
        let mut hand= deck.new_hand();
        let mut best: u32 = 0xFFFFFFFF;

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

}
