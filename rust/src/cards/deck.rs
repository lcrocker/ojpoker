//@ cards/deck_hand.rs

use std::sync::{Arc, Mutex};

// use crate::errors::*;
use crate::cards::*;

pub trait DeckTrait {
    fn remaining(&self) -> usize;
    fn size(&self) -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn contains(&self, card: Card) -> bool;
    fn new_hand(&self) -> Hand;
    fn deal_to(&self, h: Hand) -> bool;
    fn deal_all(&self, n: usize) -> bool;
    fn clear_all(&self);
    fn refill(&self);
    fn valid_card(&self, card: Card) -> Card;
    fn push(&mut self, card: Card);
    fn pop(&mut self) -> Option<Card>;
    fn push_n<I>(&mut self, n: usize, cards: I)
        where I: IntoIterator<Item = Card>;
    fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card>;
    fn remove_card(&mut self, card: Card) -> bool;
    fn shuffle(&mut self);
    fn sort(&mut self);
    fn combinations(&self, n: usize) -> CardCombinationIter;
}

#[derive(Clone, Debug)]
pub struct Deck {
    pub master: &'static MasterDeck,
    pub cards: Arc<Mutex<OrphanHand>>,
    pub hands: Vec<Arc<Mutex<OrphanHand>>>,
}

impl Deck {
    pub fn new(dname: &str) -> Deck {
        let m = MasterDeck::by_name(dname);
        Deck {
            cards: Arc::new(Mutex::new(
                OrphanHand::from_iter(m.card_list.iter().cloned()))),
            master: m,
            hands: Vec::new(),
        }
    }
}

impl DeckTrait for Deck {
    fn len(&self) -> usize {
        let s = self.cards.lock().unwrap();
        s.len()
    }

    fn remaining(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        let s = self.cards.lock().unwrap();
        s.is_empty()
    }

    fn is_not_empty(&self) -> bool {
        let s = self.cards.lock().unwrap();
        s.is_not_empty()
    }

    fn contains(&self, card: Card) -> bool {
        let s = self.cards.lock().unwrap();
        s.contains(card)
    }

    fn size(&self) -> usize {
        self.master.size()
    }

    fn new_hand(&self) -> Hand {
        Hand {
            master: self.master,
            cards: Arc::new(Mutex::new(OrphanHand::new())),
            deck: Arc::clone(&self.cards),
        }
    }

    fn deal_to(&self, h: Hand) -> bool {
        let mut d = self.cards.lock().unwrap();
        if d.is_empty() {
            return false;
        }
        let mut t = h.cards.lock().unwrap();
        t.push(d.pop().unwrap());
        true
    }

    fn deal_all(&self, n: usize) -> bool {
        let mut d = self.cards.lock().unwrap();
        if d.len() < n * self.hands.len() {
            return false;
        }
        for h in self.hands.iter() {
            let mut t = h.lock().unwrap();
            t.push_n(n, d.pop_n(n));
        }
        true
    }

    fn clear_all(&self) {
        for h in self.hands.iter() {
            let mut t = h.lock().unwrap();
            t.clear();
        }
    }

    fn refill(&self) {
        let mut d = self.cards.lock().unwrap();
        d.clear();
        d.push_n(self.size(), self.master.card_list.iter().cloned());
    }

    fn valid_card(&self, cin: Card) -> Card {
        let cout: Card = if self.master.low_aces {
            Card::low_ace_fix(cin)
        } else {
            Card::high_ace_fix(cin)
        };
        assert!(self.master.has(cout));
        cout
    }

    fn push(&mut self, card: Card) {
        let mut s = self.cards.lock().unwrap();
        s.push(card)
    }

    fn pop(&mut self) -> Option<Card> {
        let mut s = self.cards.lock().unwrap();
        s.pop()
    }

    fn push_n<I>(&mut self, n: usize, cards: I)
    where I: IntoIterator<Item = Card> {
        let mut remaining = n;
        let mut s = self.cards.lock().unwrap();

        for c in cards {
            if remaining == 0 {
                break;
            }
            remaining -= 1;
            s.push(c);
        }
    }

    fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let mut s = self.cards.lock().unwrap();
        let count = if s.len() < n { s.len() } else { n };
        let mut v = Vec::new();

        for _ in 0..count {
            v.push(s.pop().unwrap());
        }
        v.into_iter()
    }

    fn remove_card(&mut self, card: Card) -> bool {
        let mut s = self.cards.lock().unwrap();
        s.remove_card(card)
    }

    fn shuffle(&mut self) {
        let mut s = self.cards.lock().unwrap();
        s.shuffle()
    }

    fn sort(&mut self) {
        let mut s = self.cards.lock().unwrap();
        s.sort()
    }

    fn combinations(&self, n: usize) -> CardCombinationIter {
        let s = self.cards.lock().unwrap();
        let c = s.to_vec();
        CardCombinationIter::new(c, n)
    }
}

impl Default for Deck {
    fn default() -> Self {
        let m = MasterDeck::by_name("default");
        Deck {
            master: m,
            cards: Arc::new(Mutex::new(OrphanHand::from_iter(m.card_list.iter().cloned()))),
            hands: Vec::new(),
        }
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.cards.lock().unwrap();
        let l = s.len();
        let mut max: usize = l;
        let mut tail: usize = 0;

        if l > 32 {
            max = 29;
            tail = l - 29;
        }
        write!(f, "{} [", self.master.name)?;
        for i in ((l - max)..l).rev() {
            write!(f, "{}", s.card_at(i).unwrap())?;
        }
        if tail > 0 {
            write!(f, "...+{}", tail)?;
        }
        write!(f, "]")
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
    
        h1.push_n(3, [ACE_OF_CLUBS, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS]);
        assert!(h1.contains(DEUCE_OF_DIAMONDS));
        h1.insert_at(1, FOUR_OF_SPADES);
        assert_eq!(DEUCE_OF_DIAMONDS, h1.card_at(2).unwrap());
        h1.remove_at(0);
    
        let v = h1.to_vec();
        assert_eq!(vec![FOUR_OF_SPADES, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS], v);
        h2.clear();
        h2.push_n(3, cards_from_text("4s2d3h"));
        assert!(h1.equals(&h2));
        h1.push_n(2, [JACK_OF_CLUBS, QUEEN_OF_SPADES]);
        h2.push_n(2, cards_from_text("JcQs"));
        assert!(h1.equals(&h2));

        h1.push(KING_OF_HEARTS);
        assert_eq!(KING_OF_HEARTS, h1.pop().unwrap());
        assert_eq!(5, h1.len());
    
        let c = h2.remove_at(2).unwrap();
        assert_eq!(TREY_OF_HEARTS, c);


        assert_eq!(JACK_OF_CLUBS, h2.card_at(2).unwrap());
        assert!(h2.remove_card(DEUCE_OF_DIAMONDS));
        assert_eq!(JACK_OF_CLUBS, h2.card_at(1).unwrap());
    
        h2.push(h1.remove_at(2).unwrap());
        assert_eq!(FOUR_OF_SPADES, h2.card_at(0).unwrap());
        if h1.remove_card(DEUCE_OF_DIAMONDS) { h2.push(DEUCE_OF_DIAMONDS); }
        assert_eq!(FOUR_OF_SPADES, h1.card_at(0).unwrap());
        assert_eq!(h2.len(), 5);
    }
}
