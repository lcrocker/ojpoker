//@ cards/stack.rs
//! # stack | [wiki](https://github.com/lcrocker/ojpoker/wiki/CardStack) | A simple LIFO stack for cards.

use std::ops::{Index, IndexMut};

// use crate::errors::*;
use crate::cards::*;
use super::oj_shuffle;

pub trait CardStackTrait {
    fn to_vec(&self) -> Vec<Card>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn contains(&self, card: Card) -> bool;
    fn clear(&mut self);
    fn card_at(&self, index: usize) -> Option<Card>;
    fn set_card_at(&mut self, index: usize, card: Card);
    fn push(&mut self, card: Card);
    fn pop(&mut self) -> Option<Card>;
    fn push_n(&mut self, cards: &[Card]);
    fn pop_n(&mut self, n: usize) -> Vec<Card>;
    fn insert_at(&mut self, index: usize, card: Card);
    fn insert_at_end(&mut self, card: Card);
    fn remove_at(&mut self, index: usize) -> Option<Card>;
    fn remove_at_end(&mut self) -> Option<Card>;
    fn remove_card(&mut self, card: Card) -> bool;
    fn shuffle(&mut self);
    fn sort(&mut self);
    // Index and IndexMut traits are implemented below.
}

/// # CardStack | [wiki](https://github.com/lcrocker/tspoker/wiki/CardStack)
/// A `CardStack` is the basic card collection type for the library, used to
/// implement whole decks, player hands, discard piles, Texas Hold'em boards,
/// active tricks, solitaire tableaux, etc. It is a simple LIFO stack of
/// cards, with a subset of the sorts of methods of native arrays or stacks
/// or queues in many languages, but tuned with cards and simulations in mind.
/// The generic `CardStack` should be used sparingly: its enclosing classes,
/// `Hand` and `Deck`, are to be preferred for most uses as they have more
/// error checking and specialized shortcuts.
///
/// Cards are moved between stacks with `pop()` (which removes the top card
/// of the stack) and `push()` (which adds a card to the top of the stack).
/// Stacks are indexed and displayed top-down, so, for example:
/// ```
/// use onejoker::cards::*;
/// 
/// let mut hand = CardStack::new();
/// hand.push(FOUR_OF_SPADES);
/// hand.push(JOKER);
///
/// println!("{}, {}", hand, hand.card_at(0).unwrap());
/// ```
/// will print `Jk4s, Joker`.
/// Cards added to the stack as a list will be added as a unit, so:
/// ```
/// use onejoker::cards::*;
/// 
/// let mut hand = CardStack::from_text("5sJc9d");
/// hand.push_n(&[ QUEEN_OF_CLUBS, KING_OF_CLUBS ]);
/// println!("{}", hand);
/// ```
/// will print `QcKc5sJc9d`. There are also `insertX()` and `removeX()`
/// methods, but these are less efficient than `push()` and `pop()`.
#[derive(Clone, Debug)]
pub struct CardStack {
    pub cards: Vec<Card>,
}

impl CardStack {
    /// Create a new empty stack.
    pub fn new() -> CardStack {
        CardStack { cards: Vec::new() }
    }

    /// Create a new stack from a vector of cards.
    pub fn from_slice(cs: &[Card]) -> CardStack {
        let mut newv = Vec::from(cs);
        newv.reverse();
        CardStack { cards: newv }
    }

    /// Create a new stack from a string of card text, e.g. "AsKdQhJc".
    pub fn from_text(s: &str) -> CardStack {
        CardStack::from_slice(&cards_from_text(s)[..])
    }

    /// For all cards in stack, convert any high aces to low aces.
    pub fn low_ace_fix(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i] = Card::low_ace_fix(self.cards[i]);
        }
    }

    /// For all cards in stack, convert any high aces to low aces.
    pub fn high_ace_fix(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i] = Card::high_ace_fix(self.cards[i]);
        }
    }
}

impl CardStackTrait for CardStack {
    /// Return contents of stack as a vector of cards.
    fn to_vec(&self) -> Vec<Card> {
        let mut rv = self.cards.clone();
        rv.reverse();
        rv
    }

    /// How many cards are in the stack? 
    fn len(&self) -> usize {
        self.cards.len()
    }

    /// Is the stack empty?
    fn is_empty(&self) -> bool {
        0 == self.cards.len()
    }

    /// Is the stack not empty?
    fn is_not_empty(&self) -> bool {
        ! self.is_empty()
    }

    /// Is card `c` in the stack?
    fn contains(&self, c: Card) -> bool {
        self.cards.contains(&c)
    }

    /// Clear the stack of all cards.
    fn clear(&mut self) {
        self.cards.clear()
    }

    /// Return the card at given index from the top.
    fn card_at(&self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards[self.cards.len() - 1 - index])
    }

    fn set_card_at(&mut self, index: usize, card: Card) {
        if index >= self.cards.len() {
            return;
        }
        let len = self.cards.len();
        self.cards[len - 1 - index] = card;
    }

    /// Push a card onto the top of the stack.
    fn push(&mut self, c: Card) {
        self.cards.push(c)
    }

    /// Pop a card from the top of the stack.
    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Push a vector of cards onto the top of the stack as a unit.
    /// When done, the first card of the vec will be on top of the stack.
    fn push_n(&mut self, cv: &[Card]) {
        for i in (0..=(cv.len() - 1)).rev() {
            self.cards.push(cv[i]);
        }
    }

    /// Return a vector of the top N cards, or an empty vec if we
    /// can't fulfill the request (never a partial vec).
    fn pop_n(&mut self, n: usize) -> Vec<Card> {
        let mut rv: Vec<Card> = Vec::new();
        if n > self.cards.len() {
            return rv;
        }
        for _ in 0..n {
            rv.push(self.pop().unwrap());
        }
        rv
    }

    /// Insert a card at a given index from the top.
    fn insert_at(&mut self, n: usize, c: Card) {
        debug_assert!(n <= self.cards.len());
        self.cards.insert(self.cards.len() - n, c);
    }

    /// Insert a card at the bottom of the stack.
    fn insert_at_end(&mut self, c: Card) {
        self.cards.insert(0, c);
    }

    /// Remove the card at a given index from the top.
    fn remove_at(&mut self, n: usize) -> Option<Card> {
        if n >= self.cards.len() {
            return None;
        }
        Some(self.cards.remove(self.cards.len() - 1 - n))
    }

    /// Remove the card from the bottom of the stack.
    fn remove_at_end(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        Some(self.cards.remove(0))
    }

    /// Remove the first instance of the given card from the stack.
    /// Return true if the card was found and removed, false otherwise.
    fn remove_card(&mut self, c: Card) -> bool {
        for i in (0..self.cards.len()).rev() {
            if self.cards[i] == c {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    /// Randomize the order of the cards in the stack.
    fn shuffle(&mut self) {
        oj_shuffle(self.cards.as_mut_slice());
    }

    /// Sort the cards in the stack in descending from the stack top.
    fn sort(&mut self) {
        oj_sort(self.cards.as_mut_slice());
    }
}

impl Default for CardStack {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CardStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        if ! self.cards.is_empty() {
            let mut v: Vec<String> = Vec::new();
            for c in self.cards.iter().rev() {
                v.push(c.to_string());
            }
            s = v.join("");
        }
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for CardStack {
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CardStack::from_slice(&cards_from_text(s)[..]))
    }
}

impl PartialEq for CardStack {
    fn eq(&self, other: &Self) -> bool {
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
}

impl Index<usize> for CardStack {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[self.cards.len() - 1 - index]
    }
}

impl IndexMut<usize> for CardStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.cards.len();
        &mut self.cards[len - 1 - index]
    }
}

pub struct CardStackIter<'a> {
    cards: &'a Vec<Card>,
    index: usize,
}

impl CardStackIter<'_> {
    pub fn new(cards: &Vec<Card>) -> CardStackIter {
        CardStackIter { cards, index: cards.len() }
    }
}

impl CardStack {
    pub fn iter(&self) -> CardStackIter {
        CardStackIter::new(&self.cards)
    }
}

impl<'a> Iterator for CardStackIter<'a> {
    type Item = &'a Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            None
        } else {
            self.index -= 1;
            Some(&self.cards[self.index])
        }
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_methods() -> Result<(), OjError> {
        let mut h = CardStack::new();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());

        h = CardStack::from_slice(&[FOUR_OF_SPADES,JOKER]);
        assert_eq!(h.len(), 2);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JOKER);

        assert!(h.contains(FOUR_OF_SPADES));
        assert!(! h.contains(EIGHT_OF_CLUBS));
        h.clear();
        assert!(h.is_empty());
        assert!(! h.contains(FOUR_OF_SPADES));

        h = CardStack::from_text("4sJc9d");
        assert_eq!(h.len(), 3);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JACK_OF_CLUBS);
        assert_eq!(h.card_at(2).unwrap(), NINE_OF_DIAMONDS);

        h = CardStack::from_slice(&[
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
        h = CardStack::new();
        h.push(FOUR_OF_SPADES);
        assert_eq!(h.len(), 1);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        h.push(JOKER);
        assert_eq!(h.len(), 2);
        assert_eq!(h.card_at(0).unwrap(), JOKER);
        assert_eq!(h.card_at(1).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.to_string(), "Jk4s");
        assert_eq!(h.pop().unwrap(), JOKER);
        assert_eq!(h.len(), 1);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.pop().unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());

        h.push(NINE_OF_DIAMONDS);
        h.push(QUEEN_OF_SPADES);
        assert_eq!(h.to_string(), "Qs9d");

        h = CardStack::from_slice(&[ KING_OF_CLUBS, ACE_OF_CLUBS ]);
        h.push_n(&[
            TEN_OF_CLUBS, JACK_OF_CLUBS, QUEEN_OF_CLUBS
        ]);
        assert_eq!(h.to_string(), "TcJcQcKcAc");

        let list = h.pop_n(2);
        assert_eq!(list[0], TEN_OF_CLUBS);
        assert_eq!(list[1], JACK_OF_CLUBS);
        assert_eq!(h.to_string(), "QcKcAc");

        /* insert and remove
         */
        h = CardStack::from_text("4sJc9d");
        h.insert_at(1, JOKER);
        assert_eq!(h.to_string(), "4sJkJc9d");
        h.insert_at(0, TEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "Td4sJkJc9d");
        h.insert_at(4, QUEEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "Td4sJkJcQd9d");
        h.insert_at_end(ACE_OF_CLUBS);
        assert_eq!(h.to_string(), "Td4sJkJcQd9dAc");
        h.insert_at(7, SIX_OF_SPADES);
        assert_eq!(h.to_string(), "Td4sJkJcQd9dAc6s");

        assert_eq!(h.remove_at(0).unwrap(), TEN_OF_DIAMONDS);
        assert_eq!(h.to_string(), "4sJkJcQd9dAc6s");
        assert_eq!(h.remove_at(2).unwrap(), JACK_OF_CLUBS);
        assert_eq!(h.to_string(), "4sJkQd9dAc6s");
        assert_eq!(h.remove_card(ACE_OF_CLUBS), true);
        assert_eq!(h.to_string(), "4sJkQd9d6s");
        assert_eq!(h.remove_at_end().unwrap(), SIX_OF_SPADES);
        assert_eq!(h.to_string(), "4sJkQd9d");
        assert_eq!(h.remove_at(3).unwrap(), NINE_OF_DIAMONDS);
        assert_eq!(h.to_string(), "4sJkQd");

        /* shuffle and sort
         */

        h = CardStack::from_text("3h5h8dTh3c4h7sJkQs7d");
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
        h = CardStack::from_text("As2s3s4s5s6s7s8s9sTsAh2h3h4h5h6h7h8h9hTh");
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
