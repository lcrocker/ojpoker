//! [wiki](https://github.com/lcrocker/ojpoker/wiki/HandValueHigh) | Traditional "high" poker hands

use crate::utils::*;
use crate::cards::*;

/// Work around knight gap
pub const POKER_RANK_ORDER: [i8; 16] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, -1, 12, 13, 14
];

/// The straightforward way to evaluate poker hands requires making a lot
/// of checks that are dependent on each other and their order of execution.
/// This struct encapsulates the state of those checks and enforces the
/// proper order. This code handles partial hands (fewer than 5 cards),
/// but not hands > 5 cards.
pub struct EvaluatorState {
    /// The cards in the hand.
    pub cards: Vec<Card>,
    /// The ranks of the cards in the hand, maybe in different order
    pub ranks: Vec<Rank>,
    /// Does wheel count as a straight in this game?
    pub wheel_is_straight: bool,
    /// Are the ranks currently sorted (descending)?
    pub sorted: Option<bool>,
    /// Is the hand a flush?
    pub flush: Option<bool>,
    /// Is the hand a straight?
    pub straight: Option<bool>,
    /// Is the hand four of a kind?
    pub quads: Option<bool>,
    /// Is the hand a full house?
    pub full_house: Option<bool>,
    /// Is the hand three of a kind?
    pub trips: Option<bool>,
    /// Is the hand two pair?
    pub two_pair: Option<bool>,
    /// Is the hand one pair?
    pub pair: Option<bool>,
}

impl EvaluatorState {
    /// Create a new `EvaluatorState` object.
    pub fn new(hand: &Hand) -> Self {
        // Caller should have already verified this
        debug_assert!(hand.len() <= 5);

        let mut cards = hand.to_vec();
        oj_sort(&mut cards);
        let ranks: Vec<Rank> = cards.iter().map(|c| c.rank()).collect();
        assert!(ranks.len() <= 5);
        
        Self {
            cards,
            ranks,
            wheel_is_straight: true,
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

    /// Check for flush
    pub fn check_flush(&mut self) -> &mut Self {
        if self.cards.len() < 5 {
            self.flush = Some(false);
            return self;
        }

        let suit = self.cards[0].suit();
        if suit == Suit::None {
            self.flush = Some(false);
            return self;
        } 
        for i in 1..=4 {
            let s2 = self.cards[i].suit();
            if s2 != suit {
                self.flush = Some(false);
                return self;
            }
        }
        self.flush = Some(true);
        self
    }

    /// Check for straight
    pub fn check_straight(&mut self) -> &mut Self {
        debug_assert!(self.sorted == Some(true));
        if self.cards.len() < 5 {
            self.flush = Some(false);
            return self;
        }
        if self.wheel_is_straight &&
            self.ranks[0] == Rank::Ace &&
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
            if POKER_RANK_ORDER[self.ranks[i] as usize] + 1 !=
                POKER_RANK_ORDER[self.ranks[i - 1] as usize] {

                self.straight = Some(false);
                return self;
            }
        }
        self.straight = Some(true);
        self
    }

    /// Check for four of a kind
    pub fn check_quads(&mut self) -> &mut Self {
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

    /// Check for full house
    pub fn check_full_house(&mut self) -> &mut Self {
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
    
    /// Check for three of a kind
    pub fn check_trips(&mut self) -> &mut Self {
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
        if self.ranks[1] == self.ranks[2] && self.ranks[1] == self.ranks[3] {
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
    
    /// Check for two pair
    pub fn check_two_pair(&mut self) -> &mut Self {
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
    
    /// Check for one pair
    pub fn check_one_pair(&mut self) -> &mut Self {
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

    /// Verify that all the checks have been made.
    pub fn all_checks_complete(&self) -> bool {
        self.flush.is_some() &&
        self.straight.is_some() &&
        self.quads.is_some() &&
        self.full_house.is_some() &&
        self.trips.is_some() &&
        self.two_pair.is_some() &&
        self.pair.is_some()
    }
    
    /// Verify that there are no pairs in the hand.
    pub fn verify_no_pair(&self) -> bool {
        if self.cards.len() < 2 {
            return true;
        }
        for i in 1..self.cards.len() {
            for j in 0..i {
                let r = self.cards[i].rank();

                if r == self.cards[j].rank() {
                    return false;
                }
            }
        }
        true
    }
}
