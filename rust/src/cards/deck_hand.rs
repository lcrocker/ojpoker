//@ cards/deck_hand.rs

use std::sync::{Arc, Mutex};

// use crate::errors::*;
use crate::cards::*;

#[derive(Clone, Debug)]
pub struct Deck {
    pub master: &'static MasterDeck,
    pub stack: Arc<Mutex<CardStack>>,
    pub hands: Vec<Arc<Mutex<CardStack>>>,
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub master: &'static MasterDeck,
    pub stack: Arc<Mutex<CardStack>>,
    pub deck: Arc<Mutex<CardStack>>,
}

impl Deck {
    pub fn new(dname: &str) -> Deck {
        let m = MasterDeck::by_name(dname);
        Deck {
            stack: Arc::new(Mutex::new(CardStack::from_slice(&m.card_list))),
            master: &m,
            hands: Vec::new(),
        }
    }

    pub fn new_hand(&self) -> Hand {
        Hand {
            master: self.master,
            stack: Arc::new(Mutex::new(CardStack::new())),
            deck: Arc::clone(&self.stack),
        }
    }

    pub fn deal_to(&self, h: Hand) -> bool {
        let mut d = self.stack.lock().unwrap();
        if d.is_empty() {
            return false;
        }
        let mut t = h.stack.lock().unwrap();
        t.push(d.pop().unwrap());
        true
    }

    pub fn deal_all(&self, n: usize) -> bool {
        let mut d = self.stack.lock().unwrap();
        if d.len() < n * self.hands.len() {
            return false;
        }
        for h in self.hands.iter() {
            let mut t = h.lock().unwrap();
            t.push_n(&d.pop_n(n)[..]);
        }
        true
    }

    pub fn clear_all(&self) {
        for h in self.hands.iter() {
            let mut t = h.lock().unwrap();
            t.clear();
        }
    }

    pub fn refill(&self) {
        let mut d = self.stack.lock().unwrap();
        d.clear();
        d.push_n(&self.master.card_list[..]);
    }

    pub fn valid_card(&self, cin: Card) -> Card {
        let cout: Card;

        if self.master.low_aces {
            cout = Card::low_ace_fix(cin);
        } else {
            cout = Card::high_ace_fix(cin);
        }
        assert!(self.master.has(cout));
        cout
    }

    pub fn ace_fix(&self) {
        let mut s = self.stack.lock().unwrap();
        if self.master.low_aces {
            s.low_ace_fix();
        } else {
            s.high_ace_fix();
        }
    }

    pub fn remaining(&self) -> usize {
        self.len()
    }

    pub fn size(&self) -> usize {
        self.master.size()
    }
}

impl CardStackTrait for Deck {
    fn to_vec(&self) -> Vec<Card> {
        let s = self.stack.lock().unwrap();
        s.to_vec()
    }

    fn len(&self) -> usize {
        let s = self.stack.lock().unwrap();
        s.len()
    }

    fn is_empty(&self) -> bool {
        let s = self.stack.lock().unwrap();
        s.is_empty()
    }

    fn is_not_empty(&self) -> bool {
        let s = self.stack.lock().unwrap();
        s.is_not_empty()
    }

    fn contains(&self, card: Card) -> bool {
        let s = self.stack.lock().unwrap();
        s.contains(card)
    }

    fn clear(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.clear();
    }

    fn card_at(&self, index: usize) -> Option<Card> {
        let s = self.stack.lock().unwrap();
        s.card_at(index)
    }

    fn set_card_at(&mut self, index: usize, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.set_card_at(index, card)
    }

    fn push(&mut self, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.push(card)
    }

    fn pop(&mut self) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.pop()
    }

    fn push_n(&mut self, cards: &[Card]) {
        let mut s = self.stack.lock().unwrap();
        s.push_n(cards)
    }

    fn pop_n(&mut self, n: usize) -> Vec<Card>{
        let mut s = self.stack.lock().unwrap();
        s.pop_n(n)
    }

    fn insert_at(&mut self, index: usize, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.insert_at(index, card)
    }

    fn insert_at_end(&mut self, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.insert_at_end(card)
    }

    fn remove_at(&mut self, index: usize) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.remove_at(index)
    }

    fn remove_at_end(&mut self) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.remove_at_end()
    }

    fn remove_card(&mut self, card: Card) -> bool {
        let mut s = self.stack.lock().unwrap();
        s.remove_card(card)
    }

    fn shuffle(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.shuffle()
    }

    fn sort(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.sort()
    }
}

impl Default for Deck {
    fn default() -> Self {
        let m = MasterDeck::by_name("default");
        Deck {
            master: m,
            stack: Arc::new(Mutex::new(CardStack::from_slice(m.card_list))),
            hands: Vec::new(),
        }
    }
}

impl PartialEq for Deck {
    fn eq(&self, other: &Deck) -> bool {
        let s = self.stack.lock().unwrap();
        let o = other.stack.lock().unwrap();
        s.eq(&o)
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.stack.lock().unwrap();
        let l = s.len();
        let mut max: usize = l;
        let mut tail: usize = 0;

        if l > 30 {
            max = 27;
            tail = l - 27;
        }
        write!(f, "{} [", self.master.name)?;
        for i in ((l - max)..l).rev() {
            write!(f, "{}", s.card_at(i).unwrap())?;
        }
        if tail > 0 {
            write!(f, "+{}", tail)?;
        }
        write!(f, "]")
    }
}

impl Hand {
    pub fn new(d: Deck) -> Hand {
        Hand {
            master: d.master,
            stack: Arc::new(Mutex::new(CardStack::new())),
            deck: Arc::clone(&d.stack),
        }
    }

    pub fn draw(&self, n: usize) -> bool {
        let mut d = self.deck.lock().unwrap();
        if d.len() < n {
            return false;
        }
        let mut t = self.stack.lock().unwrap();
        t.push_n(&d.pop_n(n)[..]);
        true
    }

    pub fn draw_card(&self, c: Card) -> bool {
        let mut d = self.deck.lock().unwrap();
        if !d.remove_card(c) {
            return false;
        }
        let mut t = self.stack.lock().unwrap();
        t.push(c);
        true
    }

    pub fn draw_hand(&self, cl: &[Card]) -> bool {
        let mut d = self.deck.lock().unwrap();
        let mut t = self.stack.lock().unwrap();

        for c in cl.iter() {
            if !d.remove_card(*c) {
                return false;
            }
            t.push(*c);
        }
        true
    }

    pub fn ace_fix(&self) {
        let mut s = self.stack.lock().unwrap();
        if self.master.low_aces {
            s.low_ace_fix();
        } else {
            s.high_ace_fix();
        }
    }

    pub fn is_equivalent_to(&self, other: &Hand) -> bool {
        let s = self.stack.lock().unwrap();
        let o = other.stack.lock().unwrap();
        if s.len() != o.len() {
            return false;
        }
    
        if self.master.dups_allowed {
            let mut sl = s.to_vec();
            let mut ol = o.to_vec();
            oj_sort(&mut sl[..]);
            oj_sort(&mut ol[..]);

            for i in 0..s.len() {
                if sl[i] != ol[i] {
                    return false;
                }
            }
        } else {
            let mut mask1: u64 = 0;
            let mut mask2: u64 = 0;
        
            for i in 0..s.len() {
                mask1 |= 1 << s.card_at(i).unwrap().0;
                mask2 |= 1 << o.card_at(i).unwrap().0;
            }
            return mask1 == mask2;
        }
        true
    }
}

impl CardStackTrait for Hand {
    fn to_vec(&self) -> Vec<Card> {
        let s = self.stack.lock().unwrap();
        s.to_vec()
    }

    fn len(&self) -> usize {
        let s = self.stack.lock().unwrap();
        s.len()
    }

    fn is_empty(&self) -> bool {
        let s = self.stack.lock().unwrap();
        s.is_empty()
    }

    fn is_not_empty(&self) -> bool {
        let s = self.stack.lock().unwrap();
        s.is_not_empty()
    }

    fn contains(&self, card: Card) -> bool {
        let s = self.stack.lock().unwrap();
        s.contains(card)
    }

    fn clear(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.clear();
    }

    fn card_at(&self, index: usize) -> Option<Card> {
        let s = self.stack.lock().unwrap();
        s.card_at(index)
    }

    fn set_card_at(&mut self, index: usize, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.set_card_at(index, card)
    }

    fn push(&mut self, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.push(card)
    }

    fn pop(&mut self) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.pop()
    }

    fn push_n(&mut self, cards: &[Card]) {
        let mut s = self.stack.lock().unwrap();
        s.push_n(cards)
    }

    fn pop_n(&mut self, n: usize) -> Vec<Card> {
        let mut s = self.stack.lock().unwrap();
        s.pop_n(n)
    }

    fn insert_at(&mut self, index: usize, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.insert_at(index, card)
    }

    fn insert_at_end(&mut self, card: Card) {
        let mut s = self.stack.lock().unwrap();
        s.insert_at_end(card)
    }

    fn remove_at(&mut self, index: usize) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.remove_at(index)
    }

    fn remove_at_end(&mut self) -> Option<Card> {
        let mut s = self.stack.lock().unwrap();
        s.remove_at_end()
    }

    fn remove_card(&mut self, card: Card) -> bool {
        let mut s = self.stack.lock().unwrap();
        s.remove_card(card)
    }

    fn shuffle(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.shuffle()
    }

    fn sort(&mut self) {
        let mut s = self.stack.lock().unwrap();
        s.sort()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        let cs = self.stack.lock().unwrap();
        if cs.is_not_empty() {
            let mut v: Vec<String> = Vec::new();
            for c in cs.iter() {
                v.push(c.to_string());
            }
            s = v.join("");
        }
        write!(f, "[{}]", s)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        let s = self.stack.lock().unwrap();
        let o = other.stack.lock().unwrap();
        s.eq(&o)
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_deck() {
        let mut d1 = Deck::new("english");
        let mut d2 = Deck::new("52");
        assert_eq!(d1, d2);

        d2.shuffle();
        assert_ne!(d1, d2);
        d1.sort();
        d2.sort();
        assert_eq!(d1, d2);

        let mut prev = ACE_OF_SPADES;
        for _ in 0..52 {
            let c = d1.pop().unwrap();
            assert!(c <= prev);
            prev = c;
        }   
        assert!(d1.is_empty());
        assert!(d1.pop().is_none());
        d1.push(NINE_OF_DIAMONDS);
        assert_eq!(NINE_OF_DIAMONDS, d1.pop().unwrap());
        d1.refill();
    
        assert_eq!(d1.len(), 52);
        assert_eq!(d2.remaining(), 52);
        assert_eq!(d1.size(), 52);

        assert!(d1.contains(FOUR_OF_HEARTS));
        assert!(! d2.contains(JOKER));
        d2.clear();
        assert!(d2.is_empty());

        d2.push(ACE_OF_SPADES);
        d2.push(DEUCE_OF_HEARTS);
        assert!(ACE_OF_SPADES == d2.card_at(1).unwrap());
        assert!(DEUCE_OF_HEARTS == d2.card_at(0).unwrap());

        d2.push_n(&[TREY_OF_CLUBS, FOUR_OF_DIAMONDS, FIVE_OF_SPADES]);
        assert_eq!(d2.len(), 5);
        assert_eq!(d2.pop().unwrap(), TREY_OF_CLUBS);
        assert_eq!(d2.len(), 4);

        d2.remove_at(1);
        assert!(FOUR_OF_DIAMONDS == d2.card_at(0).unwrap());
        assert!(DEUCE_OF_HEARTS == d2.card_at(1).unwrap());
        assert!(d1.card_at(60).is_none());

        assert!(! d2.remove_card(TREY_OF_CLUBS));
        assert!(d2.remove_card(FOUR_OF_DIAMONDS));
        assert!(DEUCE_OF_HEARTS == d2.card_at(0).unwrap());
        assert!(ACE_OF_SPADES == d2.card_at(1).unwrap());
        d2.insert_at(0, KING_OF_CLUBS);
        // d2.insert_at(9, QUEEN_OF_DIAMONDS);
        assert!(DEUCE_OF_HEARTS == d2.card_at(1).unwrap());

        d2.refill();
        d1.shuffle();
        d2.shuffle();
        let _ = format!("{} {:?} {} {:?})", d1, d1, d2, d2);

        let mut h1 = d1.new_hand();
        let mut h2 = d2.new_hand();
    
        h1.draw(5);
        assert_eq!(47, d1.len());
        assert_eq!(5, h1.len());
        h2.draw(7);
        assert_eq!(45, d2.len());
        assert_eq!(7, h2.len());
    
        h1.clear();
        assert_eq!(0, h1.len());
        assert!(h1.is_empty());
        let c = h1.card_at(0);
        assert!(c.is_none());
    
        h1.push_n(&[ACE_OF_CLUBS, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS]);
        assert!(h1.contains(DEUCE_OF_DIAMONDS));
        h1.insert_at(1, FOUR_OF_SPADES);
        assert_eq!(DEUCE_OF_DIAMONDS, h1.card_at(2).unwrap());
        h1.remove_at(0);
    
        let v = h1.to_vec();
        assert_eq!(vec![FOUR_OF_SPADES, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS], v);
        h2.clear();
        h2.push_n(&cards_from_text("4s2d3h"));
        assert_eq!(h1, h2);
        h1.push_n(&[JACK_OF_CLUBS, QUEEN_OF_SPADES]);
        h2.push_n(&cards_from_text("JcQs"));
        assert_eq!(h1, h2);

        h1.push(KING_OF_HEARTS);
        assert_eq!(KING_OF_HEARTS, h1.pop().unwrap());
        assert_eq!(5, h1.len());
    
        h2.remove_at(2).unwrap();
        assert_eq!(TREY_OF_HEARTS, h2.card_at(3).unwrap());
        assert!(h2.remove_card(DEUCE_OF_DIAMONDS));
        assert_eq!(TREY_OF_HEARTS, h2.card_at(2).unwrap());
    
        h2.push(h1.remove_at(2).unwrap());
        assert_eq!(FOUR_OF_SPADES, h2.card_at(0).unwrap());
        if h1.remove_card(DEUCE_OF_DIAMONDS) { h2.push(DEUCE_OF_DIAMONDS); }
        assert_eq!(TREY_OF_HEARTS, h1.card_at(2).unwrap());

        assert_eq!(5, h2.len());
        assert_eq!(FOUR_OF_SPADES, h2.card_at(1).unwrap());
    }
}
