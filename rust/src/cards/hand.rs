//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand) | Hand of cards

use std::ops::{Index, IndexMut};
use crate::utils::*;
use crate::cards::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand) | Hand of cards
/// A simple array of card objects with some utility methods.
/// It is expected that most access will go through `push`/`pop``, which
/// are fast, though things line `insert` and `remove` are available.
#[derive(Clone, Debug)]
pub struct Hand {
    /// Associated [MasterDeck]
    pub master: &'static MasterDeck,
    /// Current contents
    pub cards: Vec<Card>,
}

impl Hand {
    /// Create new [Hand] associated with the named [MasterDeck].
    pub fn new(m: &'static MasterDeck) -> Hand {
        Hand {
            master: m,
            cards: Vec::new(),
        }
    }

    /// Initialize new hand
    pub fn init<I>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = Card> {
        self.clear();
        self.push_n(iter);
        self
    }

    /// Export Vec of [Card]s.
    pub fn to_vec(&self) -> Vec<Card> {
        self.cards.clone()
    }

    /// Export Vec of [Rank]s.
    pub fn ranks(&self) -> Vec<Rank> {
        self.cards.iter().map(|c| c.rank()).collect()
    }

    /// Point to a slice of the underlying [Card] array.
    pub fn as_slice(&self) -> &[Card] {
        &self.cards[..]
    }

    /// Point to a slice of the underlying [Card] array.
    pub fn as_mut_slice(&mut self) -> &mut [Card] {
        &mut self.cards[..]
    }

    /// How many cards in the hand?
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Is the hand empty?
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Is the hand not empty?
    pub fn is_not_empty(&self) -> bool {
        ! self.cards.is_empty()
    }

    /// Does the hand contain the given [Card]?
    pub fn contains(&self, card: Card) -> bool {
        if let Some(c) = self.valid_card(card) {
            return self.cards.contains(&c)
        }
        false
    }

    /// Empty the hand
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Given a [Card], return the same card if valid for our [MasterDeck]
    /// or panic. Correct ace values if needed.
    pub fn valid_card(&self, cin: Card) -> Option<Card> {
        let cout: Card = if self.master.low_aces {
            Card::low_ace_fix(cin)
        } else {
            Card::high_ace_fix(cin)
        };
        if self.master.has(cout) {
            return Some(cout);
        }
        None
    }

    /// Return the [Card] at the given index, or `None` if out of range.
    pub fn card_at(&self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards[index])
    }

    /// Set the [Card] at the given index, or return `false` if out of range.
    pub fn set_card_at(&mut self, index: usize, card: Card) -> bool {
        if index >= self.cards.len() {
            return false;
        }
        if let Some(c) = self.valid_card(card) {
            self.cards[index] = c;
            return true;
        }
        false
    }

    /// Set all cards in hand
    pub fn set<I>(&mut self, iter: I) -> bool
    where I: IntoIterator<Item = Card> {
        self.clear();
        self.push_n(iter)
    }

    /// Push a [Card] onto the end of the hand.
    pub fn push(&mut self, card: Card) -> bool {
        if let Some(c) = self.valid_card(card) {
            self.cards.push(c);
            return true;
        }
        false
    }

    /// Pop a [Card] from the end of the hand.
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Push a collection of [Card]s onto the end of the hand.
    pub fn push_n<I>(&mut self, iter: I) -> bool
    where I: IntoIterator<Item = Card> {
        let mut all_ok = true;

        for c in iter {
            if !self.push(c) {
                all_ok = false;
            }
        }
        all_ok
    }

    /// Pop `n` [Card]s from the end of the hand.
    pub fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let count = if self.cards.len() < n { self.cards.len() } else { n };
        let mut v: Vec<Card> = Vec::new();

        for _ in 0..count {
            v.push(self.cards.pop().expect("already checked length"));
        }
        v.into_iter()
    }

    /// Insert a [Card] at the given index.
    pub fn insert_at(&mut self, index: usize, card: Card) -> bool {
        if index <= self.cards.len() {
            if let Some(c) = self.valid_card(card) {
                self.cards.insert(index, c);
                return true;
            }
        }
        false
    }

    /// Remove the [Card] at the given index.
    pub fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index >= self.cards.len() {
            return None;
        }
        Some(self.cards.remove(index))
    }

    /// Remove the given [Card] from the hand if present.
    pub fn remove_card(&mut self, card: Card) -> bool {
        for i in 0..self.cards.len() {
            if self.cards[i] == card {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    /// Truncate the [Hand] to the given length.
    pub fn truncate(&mut self, n: usize) {
        self.cards.truncate(n);
    }

    /// Shuffle the [Hand] in place.
    pub fn shuffle(&mut self) {
        oj_shuffle(&mut self.cards[..]);
    }

    /// Sort the [Hand] in place.
    pub fn sort(&mut self) {
        oj_sort(&mut self.cards[..]);
    }

    /// Return an iterator over all `n`-card combinations of the hand.
    pub fn combinations(&self, k: usize) -> impl Iterator<Item = Hand> {
        CardCombinationIter::new(self, k)
    }

    /// Return true if the hands are identical: i.e., same cards in same order.
    pub fn equals(&self, other: &Self) -> bool {
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

    /// Return true if the hands are equivalent: i.e., same cards in any order.
    pub fn is_equivalent_to(&self, other: &Self) -> bool {
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

    /// Fix the ace values in the [Hand] to match the [MasterDeck].
    pub fn ace_fix(&mut self) {
        if self.master.low_aces {
            for i in 0..self.cards.len() {
                self.cards[i] = Card::low_ace_fix(self.cards[i]);
            }
        } else {
            for i in 0..self.cards.len() {
                self.cards[i] = Card::high_ace_fix(self.cards[i]);
            }
        }
    }

    /// Discard the cards at the given indices.
    pub fn discard(&mut self, indices: &mut [usize]) -> bool {
        let mut ok = true;
        oj_sort(indices);   // descending is important!

        for i in indices {
            if *i > self.len() {
                ok = false;
            } else {
                self.remove_at(*i);
            }
        }
        ok
    }
}

impl Index<usize> for Hand {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for Hand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}

impl std::fmt::Display for Hand {
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

/// Iterator over [Card]s
pub struct CardIter {
    cards: Vec<Card>,
    index: isize,
}

impl CardIter {
    /// Create a new iterator
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

impl Hand {
    /// Return an iterator over the [Card]s in the hand.
    pub fn iter(&self) -> CardIter {
        CardIter::new(self.to_vec())
    }
}

/// Iterator over [Card]s
pub struct CardIntoIter {
    /// Copy of cards from hand to interate over.
    cards: Vec<Card>,
    /// Current index minus one.
    index: isize,
}

impl CardIntoIter {
    /// Create a new iterator
    pub fn new(cards: Vec<Card>) -> CardIntoIter {
        CardIntoIter { cards, index: -1 }
    }
}

impl Iterator for CardIntoIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.index as usize >= self.cards.len() {
            return None;
        }
        Some(self.cards[self.index as usize])
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

struct CardCombinationIter {
    source: Vec<Card>,
    dest: Hand,
    indices: Vec<usize>,
    done: bool,
}

impl CardCombinationIter {
    pub fn new(hand: &Hand, k: usize) -> CardCombinationIter {
        let source = hand.to_vec();
        let dest: Hand = hand.clone().init(source.iter().take(k).cloned());
        let mut indices: Vec<usize> = Vec::with_capacity(k);

        for i in 0..k {
            indices.push(i);
        }
        CardCombinationIter { source, indices, dest, done: false }
    }
}

impl Iterator for CardCombinationIter {
    type Item = Hand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        for i in 0..self.indices.len() {
            self.dest.cards[i] = self.source[self.indices[i]];
        }
        self.done = oj_next_combination(&mut self.indices, self.source.len());
        Some(self.dest.clone())
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::OjError;

    #[test]
    fn test_hand_methods() -> Result<(), OjError> {
        let d = Deck::new("default");
        let mut h = d.new_hand();
        assert_eq!(h.len(), 0);
        assert!(h.is_empty());

        h.set([FOUR_OF_SPADES,JOKER]);
        assert_eq!(h.len(), 2);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JOKER);

        assert!(h.contains(FOUR_OF_SPADES));
        assert!(! h.contains(EIGHT_OF_CLUBS));
        h.clear();
        assert!(h.is_empty());
        assert!(! h.contains(FOUR_OF_SPADES));

        h.set(parse_cards("4sJc9d"));
        assert_eq!(h.len(), 3);
        assert_eq!(h.card_at(0).unwrap(), FOUR_OF_SPADES);
        assert_eq!(h.card_at(1).unwrap(), JACK_OF_CLUBS);
        assert_eq!(h.card_at(2).unwrap(), NINE_OF_DIAMONDS);

        let mut h2 = d.new_hand().init([
            LOW_ACE_OF_DIAMONDS, SEVEN_OF_HEARTS,
            ACE_OF_HEARTS, KING_OF_CLUBS
        ]);
        assert_eq!(h2.card_at(0).unwrap(), ACE_OF_DIAMONDS);
        assert_eq!(h2.card_at(1).unwrap(), SEVEN_OF_HEARTS);
        assert_eq!(h2.card_at(2).unwrap(), ACE_OF_HEARTS);
        assert_eq!(h2.card_at(3).unwrap(), KING_OF_CLUBS);

        h2[0] = QUEEN_OF_DIAMONDS;
        h2[2] = FIVE_OF_HEARTS;
        assert_eq!(h2.to_string(), "Qd7h5hKc");

        /* Push and pop
         */
        let d2 = Deck::new("onejoker");
        h = d2.new_hand().init([FOUR_OF_SPADES]);
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

        h.set([ TEN_OF_CLUBS, JACK_OF_CLUBS ]);
        h.push_n([
            QUEEN_OF_CLUBS, KING_OF_CLUBS, ACE_OF_CLUBS
        ]);
        assert_eq!(h.to_string(), "TcJcQcKcAc");

        let list: Vec<Card> = h.pop_n(2).collect();
        assert_eq!(list[0], ACE_OF_CLUBS);
        assert_eq!(list[1], KING_OF_CLUBS);
        assert_eq!(h.to_string(), "TcJcQc");

        /* insert and remove
         */
        h = d2.new_hand().init(parse_cards("4sJc9d"));
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

        h = d2.new_hand().init(parse_cards("3h5h8dTh3c4h7sJkQs7d"));
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
        Ok(())
    }
}
