//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueHigh) | Traditional "high" poker hands

use std::collections::HashMap;
use crate::errors::*;
use crate::utils::*;
use crate::cards::*;
use crate::poker::hand_value::*;

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

    fn ordered_for_display(&self, h: &Hand) -> aResult<Hand> {
        oj_default_ordered_for_display(h, &self.ranks[..])
    }
}

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/HandEvaluatorHigh) | Traditional "high" poker hand evaluator
/// Data for high-hand evaluator
#[allow(dead_code)] // TODO
pub struct HandEvaluatorHigh {
    /// Tables loaded from disk
    tables: Option<HandEvaluationTables>,
}

impl HandEvaluatorHigh {
    /// Create a new [HandEvaluatorHigh] object.
    pub fn new() -> HandEvaluatorHigh {
        HandEvaluatorHigh {
            tables: HandEvaluatorHigh::load_tables(),
        }
    }
    /// Load tables from disk.
    fn load_tables() -> Option<HandEvaluationTables> {
        None
    }
}

impl HandEvaluatorTrait<HandValueHigh> for HandEvaluatorHigh {
    /// Evaluate traditional high poker hands.
    fn reference_evaluator(&self, hand: &Hand) -> aResult<HandValueHigh> {
        assert!(is_valid_five_cards(hand));
        let mut st = EvaluatorState::new(hand);

        st.check_flush();
        st.check_straight();
        
        if st.straight == Some(true) && st.flush == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::StraightFlush, &st.ranks[..]))
        }

        if st.flush == Some(true) {
            debug_assert!(st.straight == Some(false));
            return aOk(HandValueHigh::new(
                HandLevelHigh::Flush, &st.ranks[..]));
        }

        if st.straight == Some(true) {
            debug_assert!(st.flush == Some(false));
            return aOk(HandValueHigh::new(
                HandLevelHigh::Straight, &st.ranks[..]));
        }
        st.check_quads();

        if st.quads == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::Quads, &st.ranks[..]));
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::FullHouse, &st.ranks[..]));
        }
        st.check_trips();

        if st.trips == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::Trips, &st.ranks[..]));
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::TwoPair, &st.ranks[..]));
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return aOk(HandValueHigh::new(
                HandLevelHigh::Pair, &st.ranks[..]));
        }
        debug_assert!(st.all_checks_complete());
        debug_assert!(st.validate_no_pair());

        aOk(HandValueHigh::new(
            HandLevelHigh::NoPair, &st.ranks[..]))
    }

    fn lookup_evaluator(&self, h: &Hand) -> aResult<HandValue<HandLevelHigh>> {
        self.reference_evaluator(h)
    }
}

impl Default for HandEvaluatorHigh {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard structure for holding hashtable for poker hand evaluation.
#[allow(dead_code)] // TODO
struct HandEvaluationTables {
    /// Total number of hash values
    hash_count: usize,
    /// Total number of equivalence classes
    ec_count: usize,
    /// Map hashes to equivalence classes
    hashes: HashMap<u64, u16>,
    /// Hand level for each ec.
    ec_levels: Vec<u8>,
    /// Ranks in order for each ec.
    ec_ranks: Vec<Vec<u8>>,
}

#[allow(dead_code)] // TODO
impl HandEvaluationTables {
    /// Create a new [HandEvaluationTables] object.
    fn new(h: usize, e: usize) -> HandEvaluationTables {
        HandEvaluationTables {
            hash_count: h,
            ec_count: e,
            hashes: HashMap::with_capacity(h),
            ec_levels: Vec::with_capacity(e),
            ec_ranks: Vec::with_capacity(e),
        }
    }
}

/// Handle knight gap (see [Knight](https://github.com/lcrocker/ojpoker/wiki/Knight)).
fn next_lower_rank(r: Rank) -> Rank {
    match r {
        Rank::Deuce => Rank::LowAce,
        Rank::Trey => Rank::Deuce,
        Rank::Four => Rank::Trey,
        Rank::Five => Rank::Four,
        Rank::Six => Rank::Five,
        Rank::Seven => Rank::Six,
        Rank::Eight => Rank::Seven,
        Rank::Nine => Rank::Eight,
        Rank::Ten => Rank::Nine,
        Rank::Jack => Rank::Ten,
        Rank::Queen => Rank::Jack,
        Rank::King => Rank::Queen,
        Rank::Ace => Rank::King,
        _ => panic!("Invalid rank"),
    }
}

struct EvaluatorState {
    cards: Vec<Card>,
    ranks: Vec<Rank>,
    sorted: Option<bool>,
    flush: Option<bool>,
    straight: Option<bool>,
    quads: Option<bool>,
    full_house: Option<bool>,
    trips: Option<bool>,
    two_pair: Option<bool>,
    pair: Option<bool>,
}

impl EvaluatorState {
    fn new(hand: &Hand) -> Self {
        let mut cards = hand.to_vec();
        oj_sort(&mut cards);
        let ranks: Vec<Rank> = cards.iter().map(|c|
            c.rank().unwrap_or(Rank::None)).collect();

        assert!(ranks.len() == 5);
        assert!(ranks[0] >= ranks[1]);
        assert!(ranks[1] >= ranks[2]);
        assert!(ranks[2] >= ranks[3]);
        assert!(ranks[3] >= ranks[4]);
        
        Self {
            cards,
            ranks,
            sorted: Some(true),
            flush: None,
            straight: None,
            quads: None,
            full_house: None,
            trips: None,
            two_pair: None,
            pair: None,
        }
    }

    pub fn all_checks_complete(&self) -> bool {
        self.flush.is_some() &&
        self.straight.is_some() &&
        self.quads.is_some() &&
        self.full_house.is_some() &&
        self.trips.is_some() &&
        self.two_pair.is_some() &&
        self.pair.is_some()
    }

    fn check_flush(&mut self) -> &mut Self {
        if self.cards.len() < 5 {
            self.flush = Some(false);
            return self;
        }

        let Ok(suit) = self.cards[0].suit() else {
            self.flush = Some(false);
            return self
        };
        for i in 1..=4 {
            let Ok(s2) = self.cards[i].suit() else {
                self.flush = Some(false);
                return self;
            };
            if s2 != suit {
                self.flush = Some(false);
                return self;
            }
        }
        self.flush = Some(true);
        self
    }
    
    fn check_straight(&mut self) -> &mut Self {
        debug_assert!(self.sorted == Some(true));
        if self.cards.len() < 5 {
            self.flush = Some(false);
            return self;
        }
        if self.ranks[0] == Rank::Ace &&
            self.ranks[1] == Rank::Five &&
            self.ranks[2] == Rank::Four &&
            self.ranks[3] == Rank::Trey &&
            self.ranks[4] == Rank::Deuce {
            
            self.ranks[0] = Rank::Five;
            self.ranks[1] = Rank::Four;
            self.ranks[2] = Rank::Trey;
            self.ranks[3] = Rank::Deuce;
            self.ranks[4] = Rank::Ace;
    
            self.sorted = Some(false);
            self.straight = Some(true);
            return self;
        }
        for i in 1..=4 {
            if self.ranks[i] != next_lower_rank(self.ranks[i - 1]) {
                self.straight = Some(false);
                return self;
            }
        }
        self.straight = Some(true);
        self
    }
    
    fn check_quads(&mut self) -> &mut Self {
        if self.cards.len() < 4 {
            self.flush = Some(false);
            return self;
        }
        debug_assert!(self.sorted == Some(true));
    
        // AAAAB
        if self.ranks[0] == self.ranks[1] &&
            self.ranks[0] == self.ranks[2] &&
            self.ranks[0] == self.ranks[3] {
    
            self.quads = Some(true);
            return self;
        }
        if self.cards.len() < 5 {
            self.quads = Some(false);
            return self;
        }
        // ABBBB
        if self.ranks[1] == self.ranks[2] &&
            self.ranks[1] == self.ranks[3] &&
            self.ranks[1] == self.ranks[4] {
    
            self.ranks.swap(0, 4);
            self.sorted = Some(false);
            self.quads = Some(true);
            return self;
        }
        self.quads = Some(false);
        self
    }
    
    fn check_full_house(&mut self) -> &mut Self {
        if self.cards.len() < 5 {
            self.flush = Some(false);
            return self;
        }
        debug_assert!(self.sorted == Some(true));
        debug_assert!(self.quads == Some(false));
        debug_assert_ne!(self.ranks[0], self.ranks[4]);
    
        // AAABB
        if self.ranks[0] == self.ranks[1] &&
            self.ranks[0] == self.ranks[2] &&
            self.ranks[3] == self.ranks[4] {
    
            self.full_house = Some(true);
            return self;
        }
        // AABBB
        if self.ranks[0] == self.ranks[1] &&
            self.ranks[2] == self.ranks[3] &&
            self.ranks[2] == self.ranks[4] {

            self.ranks.swap(0, 3);
            self.ranks.swap(1, 4);    
            self.sorted = Some(false);
            self.full_house = Some(true);
            return self;
        }
        self.full_house = Some(false);
        self
    }
    
    fn check_trips(&mut self) -> &mut Self {
        if self.cards.len() < 3 {
            self.flush = Some(false);
            return self;
        }
        debug_assert!(self.sorted == Some(true));
        debug_assert!(self.quads == Some(false));
        debug_assert!(self.full_house == Some(false));
    
        // AAABC
        if self.ranks[0] == self.ranks[1] && self.ranks[0] == self.ranks[2] {
            self.trips = Some(true);
            return self;
        }
        if self.cards.len() < 4 {
            self.trips = Some(false);
            return self;
        }
        // ABBBC
        if self.ranks[0] == self.ranks[1] && self.ranks[2] == self.ranks[3] {
            self.ranks.swap(0, 3);
            self.sorted = Some(false);
            self.trips = Some(true);
            return self;
        }
        if self.cards.len() < 5 {
            self.trips = Some(false);
            return self;
        }
        // ABCCC
        if self.ranks[2] == self.ranks[3] && self.ranks[2] == self.ranks[4] {
            self.ranks.swap(0, 3);
            self.ranks.swap(1, 4);   
            self.sorted = Some(false);
            self.trips = Some(true);
            return self;
        }
        self.trips = Some(false);
        self
    }
    
    fn check_two_pair(&mut self) -> &mut Self {
        if self.cards.len() < 4 {
            self.flush = Some(false);
            return self;
        }
        debug_assert!(self.sorted == Some(true));
        debug_assert!(self.quads == Some(false));
        debug_assert!(self.full_house == Some(false));
        debug_assert!(self.trips == Some(false));
    
        // AABBC
        if self.ranks[0] == self.ranks[1] && self.ranks[2] == self.ranks[3] {
            self.two_pair = Some(true);
            return self;
        }
        if self.cards.len() < 5 {
            self.two_pair = Some(false);
            return self;
        }
        // ABBCC
        if self.ranks[1] == self.ranks[2] && self.ranks[3] == self.ranks[4] {
            self.ranks.swap(0, 2);
            self.ranks.swap(2, 4);    
            self.sorted = Some(false);
            self.two_pair = Some(true);
            return self;
        }
        // AABCC
        if self.ranks[0] == self.ranks[1] && self.ranks[3] == self.ranks[4] {
            self.ranks.swap(2, 4);
            self.sorted = Some(false);
            self.two_pair = Some(true);
            return self;
        }
        self.two_pair = Some(false);
        self
    }
    
    fn check_one_pair(&mut self) -> &mut Self {
        if self.cards.len() < 2 {
            self.flush = Some(false);
            return self;
        }
        debug_assert!(self.sorted == Some(true));
        debug_assert!(self.quads == Some(false));
        debug_assert!(self.full_house == Some(false));
        debug_assert!(self.trips == Some(false));
        debug_assert!(self.two_pair == Some(false));
    
        // AABCD
        if self.ranks[0] == self.ranks[1] {
            self.pair = Some(true);
            return self;
        }
        if self.cards.len() < 3 {
            self.pair = Some(false);
            return self;
        }
        // ABBCD
        if self.ranks[1] == self.ranks[2] {
            self.ranks.swap(0, 2);  
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        if self.cards.len() < 4 {
            self.pair = Some(false);
            return self;
        }
        // ABCCD
        if self.ranks[2] == self.ranks[3] {
            self.ranks.swap(0, 2);
            self.ranks.swap(1, 3);   
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        if self.cards.len() < 5 {
            self.pair = Some(false);
            return self;
        }
        // ABCDD
        if self.ranks[3] == self.ranks[4] {
            self.ranks.swap(2, 4);
            self.ranks.swap(1, 3);
            self.ranks.swap(0, 2);
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        self.pair = Some(false);
        self
    }

    fn validate_no_pair(&self) -> bool {
        if self.cards.len() < 2 {
            return true;
        }
        for i in 1..self.cards.len() {
            for j in 0..i {
                let r = self.cards[i].rank().expect("already checked");

                if r == self.cards[j].rank().expect("already checked") {
                    return false;
                }
            }
        }
        true
    }
}

fn is_valid_five_cards(hand: &Hand) -> bool {
    if hand.len() != 5 {
        return false;
    }
    for i in 0..5 {
        let Ok(r) = hand[i].rank() else {
            return false;
        };
        if hand[i].suit().is_err() { return false; }
        if r == Rank::LowAce || r == Rank::Knight {
            return false;
        }
    }
    true
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_value_high() -> aResult<()> {
        let hv = HandValueHigh::new(HandLevelHigh::NoPair, &[
            Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);
        assert_eq!(hv.level, HandLevelHigh::NoPair);
        assert_eq!(hv.ranks, vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);

        let hv = HandValueHigh::best();
        assert_eq!(hv.level, HandLevelHigh::StraightFlush);
        assert_eq!(hv.ranks, vec![Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);

        let hv = HandValueHigh::worst();
        assert_eq!(hv.level, HandLevelHigh::NoPair);
        assert_eq!(hv.ranks, vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce]);

        let hv = HandValueHigh::new(HandLevelHigh::NoPair, &[
            Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten]);
        assert_eq!(hv.full_name(), "no pair: ace, king, queen, jack, ten");

        let hv = HandValueHigh::best();
        assert_eq!(hv.full_name(), "royal flush");

        let hv = HandValueHigh::worst();
        assert_eq!(hv.full_name(), "no pair: seven, five, four, trey, deuce");
        aOk(())
    }

    #[test]
    fn test_hand_evaluator_high() -> aResult<()> {
        let evaluator = HandEvaluatorHigh::new();
        let hand = Hand::from_text("2c 3c 4c 5c 6c");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::StraightFlush);
        assert_eq!(hv.ranks, vec![Rank::Six, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce]);

        let hand = Hand::from_text("2c 3c 4c 5c 7c");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::Flush);
        assert_eq!(hv.ranks, vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce]);

        let hand = Hand::from_text("2c 3c 4c 5c 6d");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::Straight);
        assert_eq!(hv.ranks, vec![Rank::Six, Rank::Five, Rank::Four, Rank::Trey, Rank::Deuce]);

        let hand = Hand::from_text("2c 2d 2h 2s 3c");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::Quads);
        assert_eq!(hv.ranks, vec![Rank::Deuce, Rank::Deuce, Rank::Deuce, Rank::Deuce, Rank::Trey]);

        let hand = Hand::from_text("2c 2d 2h 3s 3c");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::FullHouse);
        assert_eq!(hv.ranks, vec![Rank::Deuce, Rank::Deuce, Rank::Deuce, Rank::Trey, Rank::Trey]);

        let hand = Hand::from_text("2c 2d 2h 3s 4c");
        let hv = evaluator.reference_evaluator(&hand)?;
        assert_eq!(hv.level, HandLevelHigh::Trips);

        aOk(())
    }
}