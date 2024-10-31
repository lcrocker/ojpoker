//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueBadugi) | Badugi hand values

use crate::utils::*;
use crate::errors::*;
use crate::cards::*;
use crate::poker::hand_value::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueBadugi) | Badugi hand values
pub type HandValueBadugi = HandValue<HandLevelBadugi>;

impl HandValueBadugi {
    /// Create a new `HandValueBadugi` object.
    pub fn new(level: HandLevelBadugi, ranks: &[Rank], value: u64) -> HandValueBadugi {
        HandValueBadugi {
            level,
            ranks: ranks.to_vec(),
            value,
        }
    }
}

impl HandValueTrait for HandValueBadugi {
    /// Final numeric comparator
    fn value(&self) -> u64 { self.value }

    /// Best hand for this game
    fn best() -> HandValueBadugi {
        HandValueBadugi {
            level: HandLevelBadugi::FourCard,
            ranks: vec![Rank::Four, Rank::Trey, Rank::Deuce, Rank::LowAce],
            value: 0,
        }
    }

    /// Worst hand for this game
    fn worst() -> HandValueBadugi {
        HandValueBadugi {
            level: HandLevelBadugi::OneCard,
            ranks: vec![Rank::King],
            value: 0x7FFFFFFFFFFFFFFF,
        }
    }

    /// Full English name of hand, e.g. "aces and fours with a jack".
    fn full_name(&self) -> String {
        let r1: Vec<&str> = self.ranks.iter().map(|r| r.name()).collect();

        match self.level {
            HandLevelBadugi::FourCard => {
                if self.ranks[0] == Rank::Four {
                    format!("perfect badugi")
                } else {
                    format!("four-card {}, {}, {}, {}", r1[0], r1[1], r1[2], r1[3])
                }
            },
            HandLevelBadugi::ThreeCard => {
                format!("three-card {}, {}, {}", r1[0], r1[1], r1[2])
            },
            HandLevelBadugi::TwoCard => {
                format!("two-card {}, {}", r1[0], r1[1])
            },
            HandLevelBadugi::OneCard => {
                format!("one-card {}", r1[0])
            },
        }
    }

    fn ordered_for_display(&self, h: &Hand) -> Result<Hand, OjError> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

fn badugi_rank_value(cards: &[Card]) -> (u64, Vec<Rank>) {
    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank()).collect();
    oj_sort(&mut ranks[..]);
    let value: u64 =
        oj_low_hand_value_function(5 - cards.len() as u32, &ranks);
    (value, ranks)
}

fn is_badugi(cards: &[Card]) -> bool {
    let mut rank_set: u32 = 0;
    let mut suit_set: u32 = 0;

    for c in cards {
        let r = c.rank() as u32;
        if 0 == r {
            return false;
        }
        let p_set = rank_set;
        rank_set |= 1 << r;
        if p_set == rank_set {
            return false;
        }

        let s = c.suit() as u32;
        if 0 == s {
            return false;
        }
        let p_set = suit_set;
        suit_set |= 1 << s;
        if p_set == suit_set {
            return false;
        }
    }
    true
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorBadugi) | Badugi evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorBadugi;

impl HandEvaluatorBadugi {
    /// Create a new `HandEvaluatorBadugi` object.
    pub fn new() -> HandEvaluatorBadugi {
        HandEvaluatorBadugi {
        }
    }
}

impl HandEvaluatorTrait<HandValueBadugi> for HandEvaluatorBadugi {
    const COMPLETE_HAND: usize = 4;

    /// Evaluate traditional high poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> Result<HandValueBadugi, OjError> {
        assert!(all_valid_cards(hand) > 0);
        assert!(all_valid_cards(hand) <= Self::COMPLETE_HAND);
        let h4 = hand.clone();

        if hand.len() == 4 && is_badugi(hand.as_slice()) {
            let (v, ranks) = badugi_rank_value(h4.as_slice());
            return Ok(HandValueBadugi::new(
                HandLevelBadugi::FourCard, &ranks[..], v));
        }
        let mut best_value: u64 = 0x7FFFFFFFFFFFFFFF;
        let mut best_ranks: Vec<Rank> = Vec::new();

        if hand.len() >= 3 {
            for h3 in hand.combinations(3) {
                if is_badugi(h3.as_slice()) {
                    let (v, ranks) = badugi_rank_value(h3.as_slice());
                    if v < best_value {
                        best_value = v;
                        best_ranks = ranks;
                    }
                }
            }
        }
        if best_value != 0x7FFFFFFFFFFFFFFF {
            return Ok(HandValueBadugi::new(
                HandLevelBadugi::ThreeCard, &best_ranks[..], best_value));
        }

        if hand.len() >= 2 {
            for h2 in hand.combinations(2) {
                if is_badugi(h2.as_slice()) {
                    let (v, ranks) = badugi_rank_value(h2.as_slice());
                    if v < best_value {
                        best_value = v;
                        best_ranks = ranks;
                    }
                }
            }
        }
        if best_value != 0x7FFFFFFFFFFFFFFF {
            return Ok(HandValueBadugi::new(
                HandLevelBadugi::TwoCard, &best_ranks[..], best_value));
        }

        let mut least_card= hand[0];
        for i in 1..hand.len() {
            if hand[i] < least_card {
                least_card = hand[i];
            }
        }
        return Ok(HandValueBadugi::new(
            HandLevelBadugi::OneCard, &[least_card.rank()],
            40000000 + least_card.rank() as u64));
    }
}

impl Default for HandEvaluatorBadugi {
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
    fn test_hand_evaluator_badugi() -> Result<(), OjError> {
        let eval = HandEvaluatorBadugi::new();
        let deck = Deck::new("low");
        let mut hand= deck.new_hand().init(parse_cards("KsKhKdKc"));
        let mut best: u64 = 0x7FFFFFFFFFFFFFFF;

        let mut v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::OneCard);
        assert_eq!(v1.ranks[0], Rank::King);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("AdAcAhAs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::OneCard);
        assert_eq!(v1.ranks[0], Rank::LowAce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("3d9dAdKd"));
        let mut v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.ranks[0], Rank::LowAce);

        hand.set(parse_cards("Jd5cJh7c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::TwoCard);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("Kd5dJcKc"));
        v2 = eval.value_of(&hand)?;
        assert_eq!(v1, v2);
        assert_eq!(v1.ranks[0], Rank::Jack);
        assert_eq!(v1.ranks[1], Rank::Five);

        hand.set(parse_cards("7d4c7s9c"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::TwoCard);
        assert_eq!(v1.ranks[0], Rank::Seven);
        assert_eq!(v1.ranks[1], Rank::Four);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("2hTc2s5h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::ThreeCard);
        assert_eq!(v1.ranks[0], Rank::Ten);
        assert_eq!(v1.ranks[1], Rank::Five);
        assert_eq!(v1.ranks[2], Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("4s3c9d9h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::ThreeCard);
        assert_eq!(v1.ranks[0], Rank::Nine);
        assert_eq!(v1.ranks[1], Rank::Four);
        assert_eq!(v1.ranks[2], Rank::Trey);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("TcJdKhQs"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::FourCard);
        assert_eq!(v1.ranks[0], Rank::King);
        assert_eq!(v1.ranks[1], Rank::Queen);
        assert_eq!(v1.ranks[2], Rank::Jack);
        assert_eq!(v1.ranks[3], Rank::Ten);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("3c2d4s5h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::FourCard);
        assert_eq!(v1.ranks[0], Rank::Five);
        assert_eq!(v1.ranks[1], Rank::Four);
        assert_eq!(v1.ranks[2], Rank::Trey);
        assert_eq!(v1.ranks[3], Rank::Deuce);
        assert!(v1.value < best);
        best = v1.value;

        hand.set(parse_cards("Ac3d4s2h"));
        v1 = eval.value_of(&hand)?;
        assert_eq!(v1.level, HandLevelBadugi::FourCard);
        assert_eq!(v1.ranks[0], Rank::Four);
        assert_eq!(v1.ranks[1], Rank::Trey);
        assert_eq!(v1.ranks[2], Rank::Deuce);
        assert_eq!(v1.ranks[3], Rank::LowAce);
        assert!(v1.value < best);

        Ok(())
    }

}

