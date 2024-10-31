//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueAceToSix) | Ace-to-six "London" low poker hands

use crate::errors::*;
use crate::cards::*;
use crate::poker::hand_value::*;
use crate::poker::eval_state::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueAceToSix) | Representing ace-to-six low poker hands.
/// `HandValue` subclass for deuce-to-seven low hands.
pub type HandValueAceToSix = HandValue<HandLevelAceToSix>;

impl HandValueAceToSix {
    /// Create a new `HandValueAceToSix`] object.
    pub fn new(level: HandLevelAceToSix, ranks: &[Rank]) -> HandValueAceToSix {
        let ranks = ranks.to_vec();
        let value = oj_low_hand_value_function(level.index(), &ranks[..]);

        HandValueAceToSix {
            level,
            ranks,
            value,
        }
    }
}

impl HandValueTrait for HandValueAceToSix {
    /// Final numeric comparator
    fn value(&self) -> u64 { self.value }

    /// Best hand for this game
    fn best() -> HandValueAceToSix {
        HandValueAceToSix {
            level: HandLevelAceToSix::NoPair,
            ranks: vec![Rank::Six, Rank::Four, Rank::Trey, Rank::Deuce, Rank::LowAce],
            value: 0,
        }
    }

    /// Worst hand for this game
    fn worst() -> HandValueAceToSix {
        HandValueAceToSix {
            level: HandLevelAceToSix::StraightFlush,
            ranks: vec![Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine],
            value: 0x7FFFFFFFFFFFFFFF,
        }
    }

    /// Full English name of hand, e.g. "aces and fours with a jack".
    fn full_name(&self) -> String {
        let r1: Vec<&str> = self.ranks.iter().map(|r| r.name()).collect();
        let r2: Vec<&str> = self.ranks.iter().map(|r| r.plural()).collect();
        let r3: Vec<&str> = self.ranks.iter().map(|r| r.article()).collect();

        match self.level {
            HandLevelAceToSix::StraightFlush => {
                format!("{}-high straight flush", r1[0])
            },
            HandLevelAceToSix::Quads => {
                format!("four {} with {} {}", r2[0], r3[4], r1[4])
            },
            HandLevelAceToSix::FullHouse => {
                format!("{} full of {}", r2[0], r2[3])
            },
            HandLevelAceToSix::Flush => {
                format!("flush: {}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
            HandLevelAceToSix::Straight => {
                format!("{}-high straight", r1[0])
            },
            HandLevelAceToSix::Trips => {
                format!("three {}, {}, {}", r2[0], r1[3], r1[4])
            },
            HandLevelAceToSix::TwoPair => {
                format!("{} and {} with {} {}", r2[0], r2[2], r3[4], r1[4])
            },
            HandLevelAceToSix::Pair => {
                format!("pair of {}, {}, {}, {}", r2[0], r1[2], r1[3], r1[4])
            },
            HandLevelAceToSix::NoPair => {
                format!("{}, {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3], r1[4])
            },
        }
    }

    fn ordered_for_display(&self, h: &Hand) -> Result<Hand, OjError> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorAceToSix) | Traditional "AceToSix" poker hand evaluator
/// Data for AceToSix-hand evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorAceToSix {
}

impl HandEvaluatorAceToSix {
    /// Create a new `HandEvaluatorAceToSix` object.
    pub fn new() -> HandEvaluatorAceToSix {
        HandEvaluatorAceToSix {
        }
    }
}

impl HandEvaluatorTrait<HandValueAceToSix> for HandEvaluatorAceToSix {
    /// Evaluate traditional AceToSix poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> Result<HandValueAceToSix, OjError> {
        assert!(all_valid_cards(hand) > 0);
        assert!(all_valid_cards(hand) <= Self::COMPLETE_HAND);
        let mut st = EvaluatorState::new(hand);

        st.check_flush();
        st.wheel_is_straight = true;
        st.check_straight();

        if st.straight == Some(true) && st.flush == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::StraightFlush, &st.ranks[..]))
        }

        if st.flush == Some(true) {
            debug_assert!(st.straight == Some(false));
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::Flush, &st.ranks[..]));
        }

        if st.straight == Some(true) {
            debug_assert!(st.flush == Some(false));
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::Straight, &st.ranks[..]));
        }
        st.check_quads();

        if st.quads == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::Quads, &st.ranks[..]));
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::FullHouse, &st.ranks[..]));
        }
        st.check_trips();

        if st.trips == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::Trips, &st.ranks[..]));
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::TwoPair, &st.ranks[..]));
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return Ok(HandValueAceToSix::new(
                HandLevelAceToSix::Pair, &st.ranks[..]));
        }
        debug_assert!(st.all_checks_complete());
        debug_assert!(st.verify_no_pair());

        Ok(HandValueAceToSix::new(
            HandLevelAceToSix::NoPair, &st.ranks[..]))
    }
}

impl Default for HandEvaluatorAceToSix {
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
    fn test_hand_evaluator_deuce_to_seven() -> Result<(), OjError> {
        let eval = HandEvaluatorAceToSix::new();
        let deck = Deck::new("low");
        let mut hand= deck.new_hand();
        let mut best: u64 = 0x7FFFFFFFFFFFFFFF;

        hand.push_n(parse_cards("8cJc9cQcTc"));
        let mut v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::StraightFlush);

        hand.clear();
        hand.push_n(parse_cards("TdQd8d9dJd"));
        let mut v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4sAs5s3s2s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::StraightFlush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("6cTc6s6d6h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Quads);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("QcQd6d6cQh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("6c6hQc6dQs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::FullHouse);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("QsAsTsKsJs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5hJh6h4h9h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Flush);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("Qd9s8dTsJh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Straight);

        hand.clear();
        hand.push_n(parse_cards("9hQdTsJc8h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("2dAh5s3s4s"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Straight);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("3h5s3c3d9c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Trips);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("4sJsKsJd4c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::TwoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("6h2s9c6s8h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::Pair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("QdKhAcJcTd"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("8d3d4cKcTh"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::NoPair);

        hand.clear();
        hand.push_n(parse_cards("4c8sKsTd3h"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("5h2s3h7s4h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelAceToSix::NoPair);
        assert!(v1.value < best);
        best = v1.value;

        hand.clear();
        hand.push_n(parse_cards("3s4h6d2dAd"));
        v1 = eval.value_of(&hand)?;
        assert!(v1.value < best);

        Ok(())
    }
}

