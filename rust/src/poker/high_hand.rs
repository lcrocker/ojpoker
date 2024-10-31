//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueHigh) | Traditional "high" poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::hand_value::*;
use crate::poker::eval_state::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueHigh) | Representing traditional "high" poker hands.
/// `HandValue` subclass for traditional "high" poker hands.
pub type HandValueHigh = HandValue<HandLevelHigh>;

impl HandValueHigh {
    /// Create a new `HandValueHigh`] object.
    pub fn new(level: HandLevelHigh, ranks: &[Rank]) -> HandValueHigh {
        let ranks = ranks.to_vec();
        let value = oj_high_hand_value_function(level.index(), &ranks[..]);

        HandValueHigh {
            level,
            ranks,
            value,
        }
    }
}

impl HandValueTrait for HandValueHigh {
    /// Final numeric comparator
    fn value(&self) -> u64 { self.value }

    /// Best hand for this game
    fn best() -> HandValueHigh {
        HandValueHigh {
            level: HandLevelHigh::StraightFlush,
            ranks: vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten],
            value: 0,
        }
    }

    /// Worst hand for this game
    fn worst() -> HandValueHigh {
        HandValueHigh {
            level: HandLevelHigh::NoPair,
            ranks: vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce],
            value: 0x7FFFFFFFFFFFFFFF,
        }
    }

    /// Full English name of hand, e.g. "aces and fours with a jack".
    fn full_name(&self) -> String {
        let r1: Vec<&str> = self.ranks.iter().map(|r| r.name()).collect();
        let r2: Vec<&str> = self.ranks.iter().map(|r| r.plural()).collect();
        let r3: Vec<&str> = self.ranks.iter().map(|r| r.article()).collect();

        match self.level {
            HandLevelHigh::StraightFlush => {
                if self.ranks[0] == Rank::Ace {
                    String::from("royal flush")
                } else {
                    format!("{}-high straight flush", r1[0])
                }
            },
            HandLevelHigh::Quads => {
                format!("four {} with {} {}", r2[0], r3[4], r1[4])
            },
            HandLevelHigh::FullHouse => {
                format!("{} full of {}", r2[0], r2[3])
            },
            HandLevelHigh::Flush => {
                format!("flush: {}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
            HandLevelHigh::Straight => {
                format!("{}-high straight", r1[0])
            },
            HandLevelHigh::Trips => {
                format!("three {}, {}, {}", r2[0], r1[3], r1[4])
            },
            HandLevelHigh::TwoPair => {
                format!("{} and {} with {} {}", r2[0], r2[2], r3[4], r1[4])
            },
            HandLevelHigh::Pair => {
                format!("pair of {}, {}, {}, {}", r2[0], r1[2], r1[3], r1[4])
            },
            HandLevelHigh::NoPair => {
                format!("no pair: {}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
            _ => String::from("unknown hand"),
        }
    }

    fn ordered_for_display(&self, h: &Hand) -> Result<Hand, OjError> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorHigh) | Traditional "high" poker hand evaluator
/// Data for high-hand evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorHigh {
}

impl HandEvaluatorHigh {
    /// Create a new `HandEvaluatorHigh` object.
    pub fn new() -> HandEvaluatorHigh {
        HandEvaluatorHigh {
        }
    }
}

impl HandEvaluatorTrait<HandValueHigh> for HandEvaluatorHigh {
    /// Evaluate traditional high poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> Result<HandValueHigh, OjError> {
        assert!(all_valid_cards(hand) > 0);
        assert!(all_valid_cards(hand) <= Self::COMPLETE_HAND);
        let mut st = EvaluatorState::new(hand);

        st.check_flush();
        st.wheel_is_straight = true;
        st.check_straight();
        
        if st.straight == Some(true) && st.flush == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::StraightFlush, &st.ranks[..]))
        }

        if st.flush == Some(true) {
            debug_assert!(st.straight == Some(false));
            return Ok(HandValueHigh::new(
                HandLevelHigh::Flush, &st.ranks[..]));
        }

        if st.straight == Some(true) {
            debug_assert!(st.flush == Some(false));
            return Ok(HandValueHigh::new(
                HandLevelHigh::Straight, &st.ranks[..]));
        }
        st.check_quads();

        if st.quads == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::Quads, &st.ranks[..]));
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::FullHouse, &st.ranks[..]));
        }
        st.check_trips();

        if st.trips == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::Trips, &st.ranks[..]));
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::TwoPair, &st.ranks[..]));
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return Ok(HandValueHigh::new(
                HandLevelHigh::Pair, &st.ranks[..]));
        }
        debug_assert!(st.all_checks_complete());
        debug_assert!(st.verify_no_pair());

        Ok(HandValueHigh::new(
            HandLevelHigh::NoPair, &st.ranks[..]))
    }
}

impl Default for HandEvaluatorHigh {
    fn default() -> Self {
        Self::new()
    }
}

fn all_valid_cards(hand: &Hand) -> usize {
    let mut count = 0;

    for c in hand {
        if c.suit() == Suit::None { return 0; }
        let r = c.rank();
        if r == Rank::None || r == Rank::LowAce || r == Rank::Knight {
            return 0;
        }
        count += 1;
    }
    count
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_evaluator_high() -> Result<(), OjError> {
        let eval = HandEvaluatorHigh::new();
        let deck = Deck::new("poker");
        let mut hand= deck.new_hand();
        let mut best: u64 = 0x7FFFFFFFFFFFFFFF;

        hand.push_n(parse_cards("2c3h7c4d5d"));
        let mut v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::NoPair);

        hand.clear();
        hand.push_n(parse_cards("3h4s7c2h5d"));
        let mut v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("9d3dQcKcTh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::NoPair);

        hand.clear();
        hand.push_n(parse_cards("Qc9sKsTd3h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("6h2d9c6dTs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4h8c8dAd4c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5h7d5c5sKd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("Ah5s3s4s2d"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("7d9h8dTs6s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Straight);

        hand.clear();
        hand.push_n(parse_cards("9c7dTc6c8h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("KdAsJsThQh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5dTd8d4dQd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("7s7hAc7dAd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("AcAs7d7hAh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("3c3s3d3hKd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::Quads);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("Ad5d3d2d4d"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::StraightFlush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("TsQs9sJsKs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelHigh::StraightFlush);

        hand.clear();
        hand.push_n(parse_cards("Qh9hKhThJh"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }
}

