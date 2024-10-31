//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueAceToFive) | Ace-to-five low hand values

use crate::errors::*;
use crate::cards::*;
use crate::poker::hand_value::*;
use crate::poker::eval_state::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueAceToFive) | Ace-to-five low poker hands
/// `HandValue` subclass for low poker hands
pub type HandValueAceToFive = HandValue<HandLevelAceToFive>;

impl HandValueAceToFive {
    /// Create a new `HandValueHigh`] object.
    pub fn new(level: HandLevelAceToFive, ranks: &[Rank]) -> HandValueAceToFive {
        let ranks = ranks.to_vec();
        let value = oj_low_hand_value_function(level.index(), &ranks[..]);

        HandValueAceToFive {
            level,
            ranks,
            value,
        }
    }
}

impl HandValueTrait for HandValueAceToFive {
    /// Final numeric comparator
    fn value(&self) -> u64 { self.value }

    /// Best hand for this game
    fn best() -> HandValueAceToFive {
        HandValueAceToFive {
            level: HandLevelAceToFive::NoPair,
            ranks: vec![Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce, Rank::LowAce],
            value: 0,
        }
    }

    /// Worst hand for this game
    fn worst() -> HandValueAceToFive {
        HandValueAceToFive {
            level: HandLevelAceToFive::Quads,
            ranks: vec![Rank::King, Rank::King, Rank::King, Rank::King, Rank::Queen],
            value: 0x7FFFFFFFFFFFFFFF,
        }
    }

    /// Full English name of hand, e.g. "aces and fours with a jack".
    fn full_name(&self) -> String {
        let r1: Vec<&str> = self.ranks.iter().map(|r| r.name()).collect();
        let r2: Vec<&str> = self.ranks.iter().map(|r| r.plural()).collect();
        let r3: Vec<&str> = self.ranks.iter().map(|r| r.article()).collect();

        match self.level {
            HandLevelAceToFive::Quads => {
                format!("four {} with {} {}", r2[0], r3[4], r1[4])
            },
            HandLevelAceToFive::FullHouse => {
                format!("{} full of {}", r2[0], r2[3])
            },
            HandLevelAceToFive::Trips => {
                format!("three {}, {}, {}", r2[0], r1[3], r1[4])
            },
            HandLevelAceToFive::TwoPair => {
                format!("{} and {} with {} {}", r2[0], r2[2], r3[4], r1[4])
            },
            HandLevelAceToFive::Pair => {
                format!("pair of {}, {}, {}, {}", r2[0], r1[2], r1[3], r1[4])
            },
            HandLevelAceToFive::NoPair => {
                format!("{}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            }
        }
    }

    fn ordered_for_display(&self, h: &Hand) -> Result<Hand, OjError> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorHigh) | Traditional "high" poker hand evaluator
/// Data for high-hand evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorAceToFive {
}

impl HandEvaluatorAceToFive {
    /// Create a new `HandEvaluatorHigh` object.
    pub fn new() -> HandEvaluatorAceToFive {
        HandEvaluatorAceToFive {
        }
    }
}

impl HandEvaluatorTrait<HandValueAceToFive> for HandEvaluatorAceToFive {
    /// Evaluate traditional high poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> Result<HandValueAceToFive, OjError> {
        assert!(all_valid_cards(hand) > 0);
        assert!(all_valid_cards(hand) <= Self::COMPLETE_HAND);
        let mut st = EvaluatorState::new(hand);

        st.straight = Some(false);
        st.flush = Some(false);
        st.check_quads();

        if st.quads == Some(true) {
            return Ok(HandValueAceToFive::new(
                HandLevelAceToFive::Quads, &st.ranks[..]));
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return Ok(HandValueAceToFive::new(
                HandLevelAceToFive::FullHouse, &st.ranks[..]));
        }
        st.check_trips();

        if st.trips == Some(true) {
            return Ok(HandValueAceToFive::new(
                HandLevelAceToFive::Trips, &st.ranks[..]));
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return Ok(HandValueAceToFive::new(
                HandLevelAceToFive::TwoPair, &st.ranks[..]));
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return Ok(HandValueAceToFive::new(
                HandLevelAceToFive::Pair, &st.ranks[..]));
        }
        debug_assert!(st.all_checks_complete());
        debug_assert!(st.verify_no_pair());

        Ok(HandValueAceToFive::new(
            HandLevelAceToFive::NoPair, &st.ranks[..]))
    }
}

impl Default for HandEvaluatorAceToFive {
    fn default() -> Self {
        Self::new()
    }
}

fn all_valid_cards(hand: &Hand) -> usize {
    let mut count = 0;

    for c in hand {
        if c.suit() == Suit::None { return 0; }
        let r = c.rank();
        if r == Rank::None || r == Rank::Ace || r == Rank::Knight {
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
    fn test_hand_evaluator_ace_to_five() -> Result<(), OjError> {
        let eval = HandEvaluatorAceToFive::new();
        let deck = Deck::new("low");
        let mut hand= deck.new_hand();
        let mut best: u64 = 0x7FFFFFFFFFFFFFFF;

        hand.push_n(parse_cards("KsKhKdKcQs"));
        let mut v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::Quads);

        hand.clear();
        hand.push_n(parse_cards("KdQcKcKhKs"));
        let mut v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("Jd5cJcJh5s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5d5cJs5hJc"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("7d4c7s7hKc"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("2dTc2s3hTs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4s3c9d9hQc"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("TsJsKs9sQs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("KhTd9sQcJc"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);

        hand.clear();
        hand.push_n(parse_cards("Kc3d9d6h2d"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("6c9c3cKc2c"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);

        hand.clear();
        hand.push_n(parse_cards("5c3d4s7s2d"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("Ah2c4s5d3d"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToFive::NoPair);
        assert!(v1.value < best);

        Ok(())
    }

}

