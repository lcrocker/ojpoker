//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueDeuceToSeven) | Deuce-to-seven low poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::hand_value::*;
use crate::poker::eval_state::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueDeuceToSeven) | Representing deuce-to-seven low poker hands.
/// `HandValue` subclass for deuce-to-seven low hands.
pub type HandValueDeuceToSeven = HandValue<HandLevelDeuceToSeven>;

impl HandValueDeuceToSeven {
    /// Create a new `HandValueDeuceToSeven`] object.
    pub fn new(level: HandLevelDeuceToSeven, ranks: &[Rank]) -> HandValueDeuceToSeven {
        let ranks = ranks.to_vec();
        let value = oj_low_hand_value_function(level.index(), &ranks[..]);

        HandValueDeuceToSeven {
            level,
            ranks,
            value,
        }
    }
}

impl HandValueTrait for HandValueDeuceToSeven {
    /// Final numeric comparator
    fn value(&self) -> u64 { self.value }

    /// Best hand for this game
    fn best() -> HandValueDeuceToSeven {
        HandValueDeuceToSeven {
            level: HandLevelDeuceToSeven::NoPair,
            ranks: vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce],
            value: 0,
        }
    }

    /// Worst hand for this game
    fn worst() -> HandValueDeuceToSeven {
        HandValueDeuceToSeven {
            level: HandLevelDeuceToSeven::StraightFlush,
            ranks: vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten],
            value: 0x7FFFFFFFFFFFFFFF,
        }
    }

    /// Full English name of hand, e.g. "aces and fours with a jack".
    fn full_name(&self) -> String {
        let r1: Vec<&str> = self.ranks.iter().map(|r| r.name()).collect();
        let r2: Vec<&str> = self.ranks.iter().map(|r| r.plural()).collect();
        let r3: Vec<&str> = self.ranks.iter().map(|r| r.article()).collect();

        match self.level {
            HandLevelDeuceToSeven::StraightFlush => {
                if self.ranks[0] == Rank::Ace {
                    String::from("royal flush")
                } else {
                    format!("{}-high straight flush", r1[0])
                }
            },
            HandLevelDeuceToSeven::Quads => {
                format!("four {} with {} {}", r2[0], r3[4], r1[4])
            },
            HandLevelDeuceToSeven::FullHouse => {
                format!("{} full of {}", r2[0], r2[3])
            },
            HandLevelDeuceToSeven::Flush => {
                format!("flush: {}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
            HandLevelDeuceToSeven::Straight => {
                format!("{}-high straight", r1[0])
            },
            HandLevelDeuceToSeven::Trips => {
                format!("three {}, {}, {}", r2[0], r1[3], r1[4])
            },
            HandLevelDeuceToSeven::TwoPair => {
                format!("{} and {} with {} {}", r2[0], r2[2], r3[4], r1[4])
            },
            HandLevelDeuceToSeven::Pair => {
                format!("pair of {}, {}, {}, {}", r2[0], r1[2], r1[3], r1[4])
            },
            HandLevelDeuceToSeven::NoPair => {
                format!("{}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
        }
    }

    fn ordered_for_display(&self, h: &Hand) -> Result<Hand, OjError> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorDeuceToSeven) | Traditional "DeuceToSeven" poker hand evaluator
/// Data for DeuceToSeven-hand evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorDeuceToSeven {
}

impl HandEvaluatorDeuceToSeven {
    /// Create a new `HandEvaluatorDeuceToSeven` object.
    pub fn new() -> HandEvaluatorDeuceToSeven {
        HandEvaluatorDeuceToSeven {
        }
    }
}

impl HandEvaluatorTrait<HandValueDeuceToSeven> for HandEvaluatorDeuceToSeven {
    /// Evaluate traditional DeuceToSeven poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> Result<HandValueDeuceToSeven, OjError> {
        assert!(all_valid_cards(hand) > 0);
        assert!(all_valid_cards(hand) <= Self::COMPLETE_HAND);
        let mut st = EvaluatorState::new(hand);

        st.check_flush();
        st.wheel_is_straight = false;
        st.check_straight();
        
        if st.straight == Some(true) && st.flush == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::StraightFlush, &st.ranks[..]))
        }

        if st.flush == Some(true) {
            debug_assert!(st.straight == Some(false));
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::Flush, &st.ranks[..]));
        }

        if st.straight == Some(true) {
            debug_assert!(st.flush == Some(false));
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::Straight, &st.ranks[..]));
        }
        st.check_quads();

        if st.quads == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::Quads, &st.ranks[..]));
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::FullHouse, &st.ranks[..]));
        }
        st.check_trips();

        if st.trips == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::Trips, &st.ranks[..]));
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::TwoPair, &st.ranks[..]));
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return Ok(HandValueDeuceToSeven::new(
                HandLevelDeuceToSeven::Pair, &st.ranks[..]));
        }
        debug_assert!(st.all_checks_complete());
        debug_assert!(st.verify_no_pair());

        Ok(HandValueDeuceToSeven::new(
            HandLevelDeuceToSeven::NoPair, &st.ranks[..]))
    }
}

impl Default for HandEvaluatorDeuceToSeven {
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
    fn test_hand_evaluator_deuce_to_seven() -> Result<(), OjError> {
        let eval = HandEvaluatorDeuceToSeven::new();
        let deck = Deck::new("poker");
        let mut hand= deck.new_hand();
        let mut best: u64 = 0x7FFFFFFFFFFFFFFF;

        hand.push_n(parse_cards("8cJc9c7cTc"));
        let mut v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::StraightFlush);

        hand.clear();
        hand.push_n(parse_cards("Td7d8d9dJd"));
        let mut v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4cJc4s4d4h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Quads);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("KcKd8d8cKh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("8c8hKc8dKs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4sAs5s3s2s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5hTh7h4hJh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("QdKhAcJcTd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("7d9s8dTsJh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Straight);

        hand.clear();
        hand.push_n(parse_cards("9h7dTsJc8h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("2h5s2c2d9c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5sTsKsTd5c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("3h2s9c3s8h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("2dAh5s3s4s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("8d3d4cKcTh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::NoPair);

        hand.clear();
        hand.push_n(parse_cards("4c8sKsTd3h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5h2s3h7s4h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelDeuceToSeven::NoPair);

        hand.clear();
        hand.push_n(parse_cards("3s4h7d2d5d"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);

        Ok(())
    }
}

