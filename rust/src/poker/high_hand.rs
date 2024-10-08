
use crate::cards::*;
use crate::poker::*;

// Handle knight gap
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
        let ranks: Vec<Rank> = cards.iter().map(|c| c.rank().unwrap()).collect();

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
        let suit = self.cards[0].suit().unwrap();
        for i in 1..4 {
            if self.cards[i].suit().unwrap() != suit {
                self.flush = Some(false);
                return self;
            }
        }
        self.flush = Some(true);
        self
    }
    
    fn check_straight(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
    
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
        for i in 1..4 {
            if self.ranks[i] != next_lower_rank(self.ranks[i - 1]) {
                self.straight = Some(false);
                return self;
            }
        }
        self.straight = Some(true);
        self
    }
    
    fn check_quads(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
    
        // AAAAB
        if self.ranks[0] == self.ranks[1] &&
            self.ranks[0] == self.ranks[2] &&
            self.ranks[0] == self.ranks[3] {
    
            self.quads = Some(true);
            return self;
        }
        // ABBBB
        if self.ranks[1] == self.ranks[2] &&
            self.ranks[1] == self.ranks[3] &&
            self.ranks[1] == self.ranks[4] {
    
            self.ranks[4] = self.ranks[0];
            self.ranks[0] = self.ranks[1];
    
            self.sorted = Some(false);
            self.quads = Some(true);
            return self;
        }
        self.quads = Some(false);
        self
    }
    
    fn check_full_house(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
        assert!(self.quads == Some(false));
        assert_ne!(self.ranks[0], self.ranks[4]);
    
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

            self.ranks[4] = self.ranks[0];
            self.ranks[3] = self.ranks[0];
            self.ranks[0] = self.ranks[2];
            self.ranks[1] = self.ranks[2];
    
            self.sorted = Some(false);
            self.full_house = Some(true);
            return self;
        }
        self.full_house = Some(false);
        self
    }
    
    fn check_trips(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
        assert!(self.quads == Some(false));
        assert!(self.full_house == Some(false));
    
        // AAABC
        if self.ranks[0] == self.ranks[1] && self.ranks[0] == self.ranks[2] {
            assert_ne!(self.ranks[3], self.ranks[4]);
            self.trips = Some(true);
            return self;
        }
        // ABBBC
        if self.ranks[0] == self.ranks[1] && self.ranks[2] == self.ranks[3] {
            self.ranks[4] = self.ranks[0];
            self.ranks[3] = self.ranks[0];
            self.ranks[0] = self.ranks[2];
            self.ranks[1] = self.ranks[2];
    
            assert_ne!(self.ranks[3], self.ranks[4]);
            self.sorted = Some(false);
            self.trips = Some(true);
            return self;
        }
        // ABCCC
        if self.ranks[1] == self.ranks[2] && self.ranks[1] == self.ranks[3] {
            self.ranks[4] = self.ranks[0];
            self.ranks[0] = self.ranks[1];
    
            assert_ne!(self.ranks[3], self.ranks[4]);
            self.sorted = Some(false);
            self.trips = Some(true);
            return self;
        }
        self.trips = Some(false);
        self
    }
    
    fn check_two_pair(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
        assert!(self.quads == Some(false));
        assert!(self.full_house == Some(false));
        assert!(self.trips == Some(false));
    
        // AABBC
        if self.ranks[0] == self.ranks[1] && self.ranks[2] == self.ranks[3] {
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            assert_ne!(self.ranks[2], self.ranks[4]);
            self.two_pair = Some(true);
            return self;
        }
        // ABBCC
        if self.ranks[1] == self.ranks[2] && self.ranks[3] == self.ranks[4] {
            self.ranks[4] = self.ranks[0];
            self.ranks[0] = self.ranks[1];
            self.ranks[2] = self.ranks[3];
    
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            assert_ne!(self.ranks[2], self.ranks[4]);
            self.sorted = Some(false);
            self.two_pair = Some(true);
            return self;
        }
        // AABCC
        if self.ranks[0] == self.ranks[1] && self.ranks[3] == self.ranks[4] {
            self.ranks[4] = self.ranks[2];
            self.ranks[2] = self.ranks[3];
    
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            assert_ne!(self.ranks[2], self.ranks[4]);
            self.sorted = Some(false);
            self.two_pair = Some(true);
            return self;
        }
        self.two_pair = Some(false);
        self
    }
    
    fn check_one_pair(&mut self) -> &mut Self {
        assert!(self.sorted == Some(true));
        assert!(self.quads == Some(false));
        assert!(self.full_house == Some(false));
        assert!(self.trips == Some(false));
        assert!(self.two_pair == Some(false));
    
        // AABCD
        if self.ranks[0] == self.ranks[1] {
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[3]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            self.pair = Some(true);
            return self;
        }
        // ABBCD
        if self.ranks[1] == self.ranks[2] {
            self.ranks[2] = self.ranks[0];
            self.ranks[0] = self.ranks[1];
    
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[3]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        // ABCCD
        if self.ranks[2] == self.ranks[3] {
            self.ranks[3] = self.ranks[1];
            self.ranks[1] = self.ranks[2];
            self.ranks[2] = self.ranks[0];
            self.ranks[0] = self.ranks[1];
    
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[3]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        // ABCDD
        if self.ranks[3] == self.ranks[4] {
            self.ranks[4] = self.ranks[2];
            self.ranks[2] = self.ranks[0];
            self.ranks[0] = self.ranks[3];
            self.ranks[3] = self.ranks[1];
            self.ranks[1] = self.ranks[0];
    
            assert_ne!(self.ranks[0], self.ranks[2]);
            assert_ne!(self.ranks[0], self.ranks[3]);
            assert_ne!(self.ranks[0], self.ranks[4]);
            self.sorted = Some(false);
            self.pair = Some(true);
            return self;
        }
        self.pair = Some(false);
        self
    }
}

fn is_valid_five_cards(hand: &Hand) -> bool {
    if hand.len() != 5 {
        return false;
    }
    for i in 0..5 {
        if hand.card_at(i).is_none() {
            return false;
        }
        if hand.card_at(i).unwrap().suit().is_none() {
            return false;
        }
        if hand.card_at(i).unwrap().rank().unwrap() == Rank::LowAce {
            return false;
        }
        if hand.card_at(i).unwrap().rank().unwrap() == Rank::Knight {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq)]
pub struct HandValueHigh {
    base: HandValueBase<HandLevelHigh>,
}

impl HandValueHigh {
    fn new(hand: Hand, level: HandLevelHigh, ranks: &[Rank], v: Option<u64>) -> HandValueHigh {
        HandValueHigh {
            base: HandValueBase::new(hand, HandScale::HighHand, level, ranks, v),
        }
    }

    pub fn reference_evaluator(hand: Hand) -> HandValueHigh {
        assert!(is_valid_five_cards(&hand));
        let mut st = EvaluatorState::new(&hand);

        st.check_flush();
        st.check_straight();
        
        if st.straight == Some(true) && st.flush == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::StraightFlush, &st.ranks[..], None);
        }

        if st.flush == Some(true) {
            assert!(st.straight == Some(false));
            return HandValueHigh::new(hand,
                HandLevelHigh::Flush, &st.ranks[..], None);
        }

        if st.straight == Some(true) {
            assert!(st.flush == Some(false));
            return HandValueHigh::new(hand,
                HandLevelHigh::Straight, &st.ranks[..], None);
        }
        st.check_quads();

        if st.quads == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::Quads, &st.ranks[..], None);
        }
        st.check_full_house();

        if st.full_house == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::FullHouse, &st.ranks[..], None);
        }
        st.check_trips();

        if st.trips == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::Trips, &st.ranks[..], None);
        }
        st.check_two_pair();

        if st.two_pair == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::TwoPair, &st.ranks[..], None);
        }
        st.check_one_pair();

        if st.pair == Some(true) {
            return HandValueHigh::new(hand,
                HandLevelHigh::Pair, &st.ranks[..], None);
        }
        assert!(st.all_checks_complete());

        HandValueHigh::new(hand,
            HandLevelHigh::NoPair, &st.ranks[..], None)
    }

    pub fn lookup_evaluator(_hand: Hand) -> HandValueHigh {
        todo!()
    }

    pub fn fast_evaluator(_hand: Hand) -> u64 {
        todo!()
    }
}