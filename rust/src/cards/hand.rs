//@ cards/deck_hand.rs

use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

// use crate::errors::*;
use crate::cards::*;

pub trait HandTrait: IntoIterator<Item = Card> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn clear(&mut self);
    fn contains(&self, card: Card) -> bool;
    fn to_vec(&self) -> Vec<Card>;
    fn card_at(&self, index: usize) -> Option<Card>;
    fn set_card_at(&mut self, index: usize, card: Card) -> bool;
    fn push(&mut self, card: Card);
    fn pop(&mut self) -> Option<Card>;
    fn push_n<I>(&mut self, n: usize, cards: I)
        where I: IntoIterator<Item = Card>;
    fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card>;
    fn insert_at(&mut self, index: usize, card: Card);
    fn remove_at(&mut self, index: usize) -> Option<Card>;
    fn remove_card(&mut self, card: Card) -> bool;
    fn shuffle(&mut self);
    fn sort(&mut self);
    fn combinations(&self, n: usize) -> CardCombinationIter;
    fn equals(&self, other: &Self) -> bool;
    fn is_equivalent_to(&self, other: &Self) -> bool;
}

pub struct CardIter {
    cards: Vec<Card>,
    index: isize,
}

impl CardIter {
    pub fn new(cards: Vec<Card>) -> CardIter {
        CardIter { cards, index: -1 }
    }
}

impl Iterator for CardIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index as usize >= self.cards.len() {
            return None;
        }
        Some(self.cards[self.index as usize])
    }
}

impl OrphanHand {
    pub fn iter(&self) -> CardIter {
        CardIter::new(self.cards.to_vec())
    }
}

pub struct CardIntoIter {
    cards: Vec<Card>,
}

impl CardIntoIter {
    pub fn new(cards: Vec<Card>) -> CardIntoIter {
        CardIntoIter { cards }
    }
}

impl Iterator for CardIntoIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cards.is_empty() {
            return None;
        }
        Some(self.cards.remove(0))
    }
}

impl IntoIterator for OrphanHand {
    type Item = Card;
    type IntoIter = CardIntoIter;

    fn into_iter(self) -> CardIntoIter {
        CardIntoIter::new(self.to_vec())
    }
}

impl<'a> IntoIterator for &'a OrphanHand {
    type Item = Card;
    type IntoIter = CardIter;

    fn into_iter(self) -> CardIter {
        CardIter::new(self.to_vec())
    }
}

pub struct CardCombinationIter {
    source: Vec<Card>,
    indices: Vec<usize>,
    done: bool,
}

impl CardCombinationIter {
    pub fn new(source: Vec<Card>, n: usize) -> CardCombinationIter {
        let mut indices: Vec<usize> = Vec::new();
        for i in 0..n {
            indices.push(i + 1);
        }
        CardCombinationIter { source, indices, done: false }
    }
}

impl Iterator for CardCombinationIter {
    type Item = Vec<Card>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut v: Vec<Card> = Vec::new();
        for i in self.indices.iter() {
            v.push(self.source[*i]);
        }
        self.done = !oj_next_combination(&mut self.indices, self.source.len());
        Some(v)
    }
}

#[derive(Clone, Debug)]
pub struct OrphanHand {
    cards: Vec<Card>,
}

impl OrphanHand {
    pub fn new() -> OrphanHand {
        OrphanHand { cards: Vec::new() }
    }

    pub fn from_text(text: &str) -> OrphanHand {
        OrphanHand { cards: cards_from_text(text) }
    }
    pub fn low_ace_fix(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i] = Card::low_ace_fix(self.cards[i]);
        }
    }

    pub fn high_ace_fix(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i] = Card::high_ace_fix(self.cards[i]);
        }
    }
}

impl Default for OrphanHand {
    fn default() -> Self {
        OrphanHand::new()
    }
}

impl std::iter::FromIterator<Card> for OrphanHand {
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item = Card> {
        OrphanHand { cards: iter.into_iter().collect() }
    }
}

impl HandTrait for OrphanHand {
    fn len(&self) -> usize {
        self.cards.len()
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn is_not_empty(&self) -> bool {
        !self.cards.is_empty()
    }

    fn clear(&mut self) {
        self.cards.clear();
    }

    fn contains(&self, card: Card) -> bool {
        self.cards.contains(&card)
    }

    fn to_vec(&self) -> Vec<Card> {
        self.cards.clone()
    }

    fn card_at(&self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards[index])
    }

    fn set_card_at(&mut self, index: usize, card: Card) -> bool {
        if index >= self.cards.len() {
            return false;
        }
        self.cards[index] = card;
        true
    }

    fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn push_n<I>(&mut self, n: usize, cards: I)
    where I: IntoIterator<Item = Card> {
        let mut remaining = n;

        for c in cards {
            if remaining == 0 {
                break;
            }
            remaining -= 1;
            self.cards.push(c);
        }
    }

    fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let count = if self.cards.len() < n { self.cards.len() } else { n };
        let mut v: Vec<Card> = Vec::new();

        for _ in 0..count {
            v.push(self.cards.pop().unwrap());
        }
        v.into_iter()
    }

    fn insert_at(&mut self, index: usize, card: Card) {
        if index <= self.cards.len() {
            self.cards.insert(index, card);
        }
    }

    fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards.remove(index))
    }

    fn remove_card(&mut self, card: Card) -> bool {
        for i in 0..self.cards.len() {
            if self.cards[i] == card {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    fn shuffle(&mut self) {
        oj_shuffle(&mut self.cards[..]);
    }

    fn sort(&mut self) {
        oj_sort(&mut self.cards[..]);
    }

    fn combinations(&self, n: usize) -> CardCombinationIter {
        let c = self.cards.clone();
        CardCombinationIter::new(c, n)
    }

    fn equals(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }
        for i in 0..self.cards.len() {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }

    fn is_equivalent_to(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }
        let mut ss: Vec<Card> = self.cards.clone();
        let mut os: Vec<Card> = other.cards.clone();
        oj_sort(&mut ss[..]);
        oj_sort(&mut os[..]);

        for i in 0..self.cards.len() {
            if ss[i] != os[i] {
                return false;
            }
        }
        true
    }
}

impl std::str::FromStr for OrphanHand {
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrphanHand::from_text(s))
    }
}

impl Index<usize> for OrphanHand {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for OrphanHand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}

impl std::fmt::Display for OrphanHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        if !self.cards.is_empty() {
            let mut v: Vec<String> = Vec::new();
            for c in self.cards.iter() {
                v.push(c.to_string());
            }
            s = v.join("");
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    pub master: &'static MasterDeck,
    pub cards: Arc<Mutex<OrphanHand>>,
    pub deck: Arc<Mutex<OrphanHand>>,
}

impl Hand {
    pub fn new(d: Deck) -> Hand {
        Hand {
            master: d.master,
            cards: Arc::new(Mutex::new(OrphanHand::new())),
            deck: Arc::clone(&d.cards),
        }
    }

    pub fn draw(&self, n: usize) -> bool {
        let mut d = self.deck.lock().unwrap();
        if d.len() < n {
            return false;
        }
        let mut t = self.cards.lock().unwrap();
        t.push_n(n, d.pop_n(n));
        true
    }

    pub fn draw_card(&self, c: Card) -> bool {
        let mut d = self.deck.lock().unwrap();
        if !d.remove_card(c) {
            return false;
        }
        let mut t = self.cards.lock().unwrap();
        t.push(c);
        true
    }

    pub fn draw_hand(&self, cl: &[Card]) -> bool {
        let mut d = self.deck.lock().unwrap();
        let mut t = self.cards.lock().unwrap();

        for c in cl.iter() {
            if !d.remove_card(*c) {
                return false;
            }
            t.push(*c);
        }
        true
    }

    pub fn ace_fix(&self) {
        let mut t = self.cards.lock().unwrap();
        if self.master.low_aces {
            t.low_ace_fix();
        } else {
            t.high_ace_fix();
        }
    }

    pub fn valid_card(&self, cin: Card) -> Card {
        let cout: Card = if self.master.low_aces {
            Card::low_ace_fix(cin)
        } else {
            Card::high_ace_fix(cin)
        };
        assert!(self.master.has(cout));
        cout
    }
}

impl Hand {
    pub fn iter(&self) -> CardIter {
        CardIter::new(self.to_vec())
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = CardIntoIter;

    fn into_iter(self) -> CardIntoIter {
        CardIntoIter::new(self.to_vec())
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = Card;
    type IntoIter = CardIter;

    fn into_iter(self) -> CardIter {
        CardIter::new(self.to_vec())
    }
}

impl HandTrait for Hand {
    fn len(&self) -> usize {
        let t = self.cards.lock().unwrap();
        t.len()
    }

    fn is_empty(&self) -> bool {
        let t = self.cards.lock().unwrap();
        t.is_empty()
    }

    fn is_not_empty(&self) -> bool {
        let t = self.cards.lock().unwrap();
        !t.is_empty()
    }

    fn clear(&mut self) {
        let mut t = self.cards.lock().unwrap();
        t.clear();
    }

    fn contains(&self, card: Card) -> bool {
        let t = self.cards.lock().unwrap();
        t.contains(card)
    }

    fn to_vec(&self) -> Vec<Card> {
        let t = self.cards.lock().unwrap();
        t.to_vec()
    }

    fn card_at(&self, index: usize) -> Option<Card> {
        let t = self.cards.lock().unwrap();
        if index >= t.len() {
            return None;
        }
        Some(t[index])
    }

    fn set_card_at(&mut self, index: usize, card: Card) -> bool {
        let mut t = self.cards.lock().unwrap();
        if index >= t.len() {
            return false;
        }
        t[index] = card;
        true
    }

    fn push(&mut self, card: Card) {
        let mut t = self.cards.lock().unwrap();
        t.push(card);
    }

    fn pop(&mut self) -> Option<Card> {
        let mut t = self.cards.lock().unwrap();
        t.pop()
    }

    fn push_n<I>(&mut self, n: usize, cards: I)
    where I: IntoIterator<Item = Card> {
        let mut remaining = n;
        let mut t = self.cards.lock().unwrap();

        for c in cards {
            if remaining == 0 {
                break;
            }
            remaining -= 1;
            t.push(c);
        }
    }

    fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let mut t = self.cards.lock().unwrap();
        let count = if t.len() < n { t.len() } else { n };
        let mut v: Vec<Card> = Vec::new();

        for _ in 0..count {
            v.push(t.pop().unwrap());
        }
        v.into_iter()
    }

    fn insert_at(&mut self, index: usize, card: Card) {
        let mut t = self.cards.lock().unwrap();
        if index <= t.len() {
            t.insert_at(index, card);
        }
    }

    fn remove_at(&mut self, index: usize) -> Option<Card> {
        let mut t = self.cards.lock().unwrap();
        if index >= t.len() {
            return None;
        }
        t.remove_at(index)
    }

    fn remove_card(&mut self, card: Card) -> bool {
        let mut t = self.cards.lock().unwrap();
        t.remove_card(card)
    }

    fn shuffle(&mut self) {
        let mut t = self.cards.lock().unwrap();
        t.shuffle();
    }

    fn sort(&mut self) {
        let mut t = self.cards.lock().unwrap();
        t.sort();
    }

    fn combinations(&self, n: usize) -> CardCombinationIter {
        let t = self.cards.lock().unwrap();
        let c = t.to_vec();
        CardCombinationIter::new(c, n)
    }

    fn equals(&self, other: &Self) -> bool {
        let t = self.cards.lock().unwrap();
        let o = other.cards.lock().unwrap();
        t.equals(&o)
    }

    fn is_equivalent_to(&self, other: &Self) -> bool {
        let t = self.cards.lock().unwrap();
        let o = other.cards.lock().unwrap();

        if self.master.dups_allowed {
            return t.is_equivalent_to(&o);
        }
        let mut mask1: u64 = 0;
        let mut mask2: u64 = 0;
    
        for i in 0..t.len() {
            mask1 |= 1 << t.card_at(i).unwrap().0;
            mask2 |= 1 << o.card_at(i).unwrap().0;
        }
        mask1 == mask2
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let t = self.cards.lock().unwrap();
        write!(f, "{}", t)
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_methods() -> Result<(), OjError> {
        let mut h = OrphanHand::new();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());

        h = OrphanHand::from_iter([FOUR_OF_SPADES,JOKER]);
        assert_eq!(h.len(), 2);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JOKER);

        assert!(h.contains(FOUR_OF_SPADES));
        assert!(! h.contains(EIGHT_OF_CLUBS));
        h.clear();
        assert!(h.is_empty());
        assert!(! h.contains(FOUR_OF_SPADES));

        h = OrphanHand::from_text("4sJc9d");
        assert_eq!(h.len(), 3);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JACK_OF_CLUBS);
        assert_eq!(h.card_at(2).unwrap(), NINE_OF_DIAMONDS);

        h = OrphanHand::from_iter([
            LOW_ACE_OF_DIAMONDS, SEVEN_OF_HEARTS,
            ACE_OF_HEARTS, KING_OF_CLUBS
        ]);
        assert_eq!(h.card_at(0).unwrap(), LOW_ACE_OF_DIAMONDS);
        assert_eq!(h.card_at(1).unwrap(), SEVEN_OF_HEARTS);
        assert_eq!(h.card_at(2).unwrap(), ACE_OF_HEARTS);
        assert_eq!(h.card_at(3).unwrap(), KING_OF_CLUBS);
        h.low_ace_fix();
        assert_eq!(h.card_at(0).unwrap(), LOW_ACE_OF_DIAMONDS);
        assert_eq!(h.card_at(1).unwrap(), SEVEN_OF_HEARTS);
        assert_eq!(h.card_at(2).unwrap(), LOW_ACE_OF_HEARTS);
        assert_eq!(h.card_at(3).unwrap(), KING_OF_CLUBS);

        h[0] = QUEEN_OF_DIAMONDS;
        h[2] = FIVE_OF_HEARTS;
        assert_eq!(h.to_string(), "Qd7h5hKc");

        /* Push and pop
         */
        h = OrphanHand::new();
        h.push(FOUR_OF_SPADES);
        assert_eq!(h.len(), 1);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        h.push(JOKER);
        assert_eq!(h.len(), 2);
        assert_eq!(h.card_at(1).unwrap(), JOKER);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.to_string(), "4sJk");
        assert_eq!(h.pop().unwrap(), JOKER);
        assert_eq!(h.len(), 1);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.pop().unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());

        h.push(NINE_OF_DIAMONDS);
        h.push(QUEEN_OF_SPADES);
        assert_eq!(h.to_string(), "9dQs");

        h = OrphanHand::from_iter([ TEN_OF_CLUBS, JACK_OF_CLUBS ]);
        h.push_n(3, [
            QUEEN_OF_CLUBS, KING_OF_CLUBS, ACE_OF_CLUBS
        ]);
        assert_eq!(h.to_string(), "TcJcQcKcAc");

        let list: Vec<Card> = h.pop_n(2).collect();
        assert_eq!(list[0], ACE_OF_CLUBS);
        assert_eq!(list[1], KING_OF_CLUBS);
        assert_eq!(h.to_string(), "TcJcQc");

        /* insert and remove
         */
        h = OrphanHand::from_text("4sJc9d");
        h.insert_at(1, JOKER);
        assert_eq!(h.to_string(), "4sJkJc9d");
        h.insert_at(0, TEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "Td4sJkJc9d");
        h.insert_at(4, QUEEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "Td4sJkJcQd9d");
        h.push(ACE_OF_CLUBS);
        assert_eq!(h.to_string(), "Td4sJkJcQd9dAc");
        h.insert_at(7, SIX_OF_SPADES);
        assert_eq!(h.to_string(), "Td4sJkJcQd9dAc6s");

        assert_eq!(h.remove_at(0).unwrap(), TEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "4sJkJcQd9dAc6s");
        assert_eq!(h.remove_at(2).unwrap(), JACK_OF_CLUBS);
        assert_eq!(h.to_string(), "4sJkQd9dAc6s");
        assert_eq!(h.remove_card(ACE_OF_CLUBS), true);
        assert_eq!(h.to_string(), "4sJkQd9d6s");
        assert_eq!(h.pop().unwrap(), SIX_OF_SPADES);
        assert_eq!(h.to_string(), "4sJkQd9d");
        assert_eq!(h.remove_at(3).unwrap(), NINE_OF_DIAMONDS);
        assert_eq!(h.to_string(), "4sJkQd");

        /* shuffle and sort
         */

        h = OrphanHand::from_text("3h5h8dTh3c4h7sJkQs7d");
        h.shuffle();
        assert_eq!(h.len(), 10);
        assert!(h.contains(FIVE_OF_HEARTS));
        assert!(h.contains(TEN_OF_HEARTS));
        assert!(h.contains(SEVEN_OF_DIAMONDS));
        assert!(! h.contains(NINE_OF_CLUBS));

        h.sort();
        assert_eq!(h.to_string(), "QsTh8d7s7d5h4h3h3cJk");

        h.remove_card(SEVEN_OF_DIAMONDS);
        h.shuffle();
        assert_eq!(h.len(), 9);
        assert!(h.contains(TREY_OF_CLUBS));
        assert!(h.contains(SEVEN_OF_SPADES));
        assert!(h.contains(JOKER));
        assert!(! h.contains(SEVEN_OF_DIAMONDS));

        h.sort();
        assert_eq!(h.to_string(), "QsTh8d7s5h4h3h3cJk");

        // Test randomness of shuffle
        let mut counts = [0; 20];
        h = OrphanHand::from_text("As2s3s4s5s6s7s8s9sTsAh2h3h4h5h6h7h8h9hTh");
        for _ in 0..1000000 {
            h.shuffle();
            for j in 0..20 {
                if h[j] == ACE_OF_SPADES {
                    counts[j] += 1;
                    break;
                }
            }
        }
        for i in 0..20 {
            assert!(counts[i] > 49000);
            assert!(counts[i] < 51000);
        }
        Ok(())
    }
}
