//@ cards/stack.rs

//! # stack | [wiki](https://github.com/lcrocker/tspoker/wiki/CardStack) | A simple LIFO stack for cards.

// use crate::errors::*;
use crate::cards::*;

use super::oj_shuffle;

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

    /// Return contents of stack as a vector of cards.
    pub fn to_vec(&self) -> Vec<Card> {
        let mut rv = self.cards.clone();
        rv.reverse();
        rv
    }

    /// For all cards in stack, convert any high aces to low aces.
    pub fn low_ace_fix(&mut self) {
        for i in 0..self.cards.len() {
            self.cards[i] = Card::low_ace_fix(self.cards[i]);
        }
    }

    /// How many cards are in the stack? 
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Is the stack empty?
    pub fn is_empty(&self) -> bool {
        0 == self.cards.len()
    }

    /// Clear the stack of all cards.
    pub fn clear(&mut self) {
        self.cards.clear()
    }

    /// Is card `c` in the stack?
    pub fn contains(&self, c: Card) -> bool {
        self.cards.contains(&c)
    }

    /// Return the card at given index from the top.
    pub fn card_at(&self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards[self.cards.len() - 1 - index])
    }

    /// Push a card onto the top of the stack.
    pub fn push(&mut self, c: Card) {
        self.cards.push(c)
    }

    /// Pop a card from the top of the stack.
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Push a vector of cards onto the top of the stack as a unit.
    /// When done, the first card of the vec will be on top of the stack.
    pub fn push_n(&mut self, cv: &[Card]) {
        for i in (0..=(cv.len() - 1)).rev() {
            self.cards.push(cv[i]);
        }
    }

    /// Return a vector of the top N cards, or an empty vec if we
    /// can't fulfill the request (never a partial vec).
    pub fn pop_n(&mut self, n: usize) -> Vec<Card> {
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
    pub fn insert_at(&mut self, n: usize, c: Card) {
        debug_assert!(n <= self.cards.len());
        self.cards.insert(self.cards.len() - n, c);
    }

    /// Insert a card at the bottom of the stack.
    pub fn insert_at_end(&mut self, c: Card) {
        self.cards.insert(0, c);
    }

    /// Remove the card at a given index from the top.
    pub fn remove_at(&mut self, n: usize) -> Option<Card> {
        if n >= self.cards.len() {
            return None;
        }
        Some(self.cards.remove(self.cards.len() - 1 - n))
    }

    /// Remove the card from the bottom of the stack.
    pub fn remove_at_end(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            return None;
        }
        Some(self.cards.remove(0))
    }

    /// Remove the first instance of the given card from the stack.
    /// Return true if the card was found and removed, false otherwise.
    pub fn remove_card(&mut self, c: Card) -> bool {
        for i in (0..self.cards.len()).rev() {
            if self.cards[i] == c {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    /// Quick-and dirty FNV hash function for hands. Likely to have collisions.
    pub fn quick_hash(&self) -> u32 {
        let mut h: u32 = 0x811c9dc5;
        for c in self.cards.iter().rev() {
            h ^= c.0 as u32;
            h = h.wrapping_mul(0x01000193);
        }
        h
    }   

    /// Randomize the order of the cards in the stack.
    pub fn shuffle(&mut self) {
        oj_shuffle(self.cards.as_mut_slice());
    }

    /// Sort the cards in the stack in descending from the stack top.
    pub fn sort(&mut self) {
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

impl std::cmp::PartialEq for CardStack {
    fn eq(&self, other: &Self) -> bool {
        let sc = &self.cards;
        let oc = &other.cards;
        if sc.len() != oc.len() { return false; }

        let mut v1 = sc.clone();
        oj_sort(&mut v1[..]);
        let mut v2 = oc.clone();
        oj_sort(&mut v2[..]);
        v1 == v2
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::BufReader;
    use rmp_serde::decode::from_read;

    #[derive(Debug, Deserialize)]
    struct HandData {
        deck: i32,
        text: String,
        len: usize,
        hash: u32,
    }

    #[derive(Debug, Deserialize)]
    struct HandDataList {
        count: usize,
        deck_names: Vec<String>,
        hands: Vec<HandData>,
    }

    #[test]
    fn test_hand_data_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("../data/bin/hands_text.msgpack");
        if file.is_err() {
            eprintln!("No test data file found; skipping test.");
            return Ok(());
        }
        let reader = BufReader::new(file.unwrap());
        let data: HandDataList = from_read(reader)?;

        for i in 0..data.count as usize {
            let deck = MasterDeck::by_name(&data.deck_names[data.hands[i].deck as usize - 1]);
            let mut h = CardStack::from_text(&data.hands[i].text);
            if deck.low_aces {
                h.low_ace_fix();
            }
            assert_eq!(h.len(), data.hands[i].len as usize);
            for j in 0..h.len() {
                assert_eq!(true, deck.has(h.card_at(j).unwrap()));
            }
            assert_eq!(h.to_string(), data.hands[i].text);
            assert_eq!(h.quick_hash(), data.hands[i].hash);
        }
        Ok(())
    }
}
