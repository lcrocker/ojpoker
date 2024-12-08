//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand) | Hand of cards

use std::ops::{Index, IndexMut};
use crate::utils::*;
use crate::cards::*;

const MAX_HAND_SIZE: usize = 22;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Hand) | Hand of cards
/// A simple array of card objects with some utility methods.
/// It is expected that most access will go through `push()`/`pop()`, which
/// are fast, though things like `insert()` and `remove()` are available.
/// Limited to 22 cards. If you need more, you can use `Vec<Card>`. but you
/// lose some error checking and convenience methods.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Hand {
    /// Array of [Card]s
    pub cards: [Card; MAX_HAND_SIZE],
    /// Number of cards in the hand
    pub length: u8,
    /// [DeckType] associated with this hand
    pub deck_type: u8,
}

impl Hand {
    /// Create new [Hand] associated with the given [DeckType]
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::new(DeckType::OneJoker);
    /// ```
    pub fn new(t: DeckType) -> Hand {
        Hand {
            cards: [Card::default(); MAX_HAND_SIZE],
            length: 0,
            deck_type: t as u8,
        }
    }

    /// Create new [Hand] associated with the [DeckType] of the given name
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::new_by_name("onejoker");
    /// ```
    pub fn new_by_name(dname: &str) -> Hand {
        Hand {
            cards: [Card::default(); MAX_HAND_SIZE],
            length: 0,
            deck_type: DeckType::by_name(dname) as u8,
        }
    }

    /// Initialize new hand; return self for chaining
    /// ```rust
    /// use onejoker::*;
    ///
    /// let d = Deck::new(DeckType::English);
    /// let h = d.new_hand().init(cards!("Qs", "Ac"));
    /// ```
    pub fn init<I>(mut self, iter: I) -> Self
    where I: IntoIterator<Item = Card> {
        self.clear();
        self.push_all(iter);
        self
    }

    /// Initial sort for new hand; return self for chaining
    /// ```rust
    /// use onejoker::*;
    ///
    /// let d = Deck::new(DeckType::English);
    /// let h = d.new_hand().init(cards!("Qs", "Ac")).sorted();
    /// assert_eq!(h.to_string(), "AcQs");
    /// ```
    pub fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    /// Return the [DeckType] associated with this hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let d = Deck::new_by_name("poker"); // alias for "english"
    /// let h = d.new_hand();
    /// assert_eq!(h.deck_type(), DeckType::English);
    /// ```
    #[inline]
    pub fn deck_type(&self) -> DeckType {
        DeckType::from_u8(self.deck_type)
    }

    /// How many cards in the hand?
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let h = d.new_hand().init(d.draw_hand(cards!("Qs", "Ac")));
    /// assert_eq!(h.len(), 2);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.length as usize
    }

    #[inline]
    /// Is the hand empty?
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let mut h = d.new_hand();
    /// assert!(h.is_empty());
    /// d.deal_to(&mut h, 5);
    /// assert!(h.is_not_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        0 == self.length
    }

    #[inline]
    /// Is the hand not empty?
    pub fn is_not_empty(&self) -> bool {
        0 != self.length
    }

    #[inline]
    /// Empty the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("As", "Qc"));
    /// assert_eq!(h.len(), 2);
    /// h.clear();
    /// assert!(h.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.length = 0;
    }

    /// Export Vec of [Card]s (as copy)
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::default().init(cards!("As", "Qc"));
    /// let v = h.to_vec();
    /// assert_eq!(v.len(), 2);
    /// assert_eq!(h.len(), 2);
    /// ```
    pub fn to_vec(&self) -> Vec<Card> {
        self.cards[..(self.length as usize)].to_vec()
    }

    #[inline]
    /// Point to a slice of the underlying [Card] array
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::default().init(cards!("As", "Qc"));
    /// let c: &[Card] = h.as_slice();
    /// ```
    pub fn as_slice(&self) -> &[Card] {
        &self.cards[..(self.length as usize)]
    }

    #[inline]
    /// Point to a mutable slice of the underlying [Card] array
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("As", "Qc"));
    /// let c: &mut [Card] = h.as_mut_slice();
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [Card] {
        &mut self.cards[..(self.length as usize)]
    }

    /// Find given card in the hand, return index
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::default().init(cards!("Ac", "Kc", "Qc", "Jc", "Tc"));
    /// assert_eq!(h.index_of(QUEEN_OF_CLUBS).unwrap(), 2);
    /// assert_eq!(h.index_of(FOUR_OF_CLUBS), None);
    /// ```
    pub fn index_of(&self, card: Card) -> Option<usize> {
        if let Some(c) = self.deck_type().valid_card(card) {
            for i in 0..(self.length as usize) {
                if c == self.cards[i] {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Does the hand contain the given [Card]?
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::default().init(cards!("Ac", "Kc", "Qc", "Jc", "Tc"));
    /// assert!(h.contains(QUEEN_OF_CLUBS));
    /// assert!(! h.contains(FOUR_OF_CLUBS));
    /// ```
    pub fn contains(&self, card: Card) -> bool {
        if let Some(c) = self.deck_type().valid_card(card) {
            for i in 0..(self.length as usize) {
                if c == self.cards[i] {
                    return true;
                }
            }
        }
        false
    }

    /// Return the [Card] at the given index, or `None` if out of range
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h = Hand::default().init(cards!("Ac", "Kc", "Qc", "Jc", "Tc"));
    /// assert_eq!(h.card_at(2).unwrap(), QUEEN_OF_CLUBS);
    /// assert_eq!(h.card_at(7), None);
    /// ```
    pub fn card_at(&self, index: usize) -> Option<Card> {
        if index >= (self.length as usize) {
            return None;
        }
        Some(self.cards[index])
    }

    /// Set the [Card] at the given index, or return `false` if out of range
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ac", "Kc", "Qc", "Jc", "Tc"));
    /// assert_eq!(h.set_card_at(2, QUEEN_OF_DIAMONDS), true);
    /// assert_eq!(h[2], QUEEN_OF_DIAMONDS);
    /// ```
    pub fn set_card_at(&mut self, index: usize, card: Card) -> bool {
        if index >= (self.length as usize) {
            return false;
        }
        if let Some(c) = self.deck_type().valid_card(card) {
            self.cards[index] = c;
            return true;
        }
        false
    }

    /// Push a [Card] onto the end of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh"));
    /// h.push(QUEEN_OF_HEARTS);
    /// assert_eq!(h.to_string(), "AhKhQh");
    /// ```
    pub fn push(&mut self, card: Card) -> bool {
        if (self.length as usize) >= MAX_HAND_SIZE {
            return false;
        }
        if let Some(c) = self.deck_type().valid_card(card) {
            self.cards[self.length as usize] = c;
            self.length += 1;
            return true;
        }
        false
    }

    /// Pop a [Card] from the end of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh"));
    /// assert_eq!(h.pop().unwrap(), QUEEN_OF_HEARTS);
    /// assert_eq!(h.to_string(), "AhKh");
    /// ```
    pub fn pop(&mut self) -> Option<Card> {
        if self.is_empty() {
            return None;
        }
        self.length -= 1;
        Some(self.cards[self.length as usize])
    }

    /// Push a collection of [Card]s onto the end of the hand, at most `n`,
    /// returning the number actually pushed.
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let mut h = d.new_hand().init(cards!("Ah", "Kh"));
    /// assert_eq!(3, h.push_n(3, d.pop_n(3)));
    /// ```
    pub fn push_n<I>(&mut self, n: usize, iter: I) -> usize
    where I: IntoIterator<Item = Card> {
        let mut pushed: usize = 0;

        for c in iter {
            if (self.length as usize) >= MAX_HAND_SIZE {
                break;
            }
            if let Some(cout) = self.deck_type().valid_card(c) {
                self.cards[self.length as usize] = cout;
                self.length += 1;
                pushed += 1;

                if pushed >= n {
                    break;
                }
            }
        }
        pushed
    }

    /// Push a collection of [Card]s onto the end of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let mut h = d.new_hand().init(cards!("Ah", "Kh"));
    /// h.push_all(d.pop_n(3));
    /// assert_eq!(5, h.len());
    /// ```
    pub fn push_all<I>(&mut self, iter: I) -> usize
    where I: IntoIterator<Item = Card> {
        let mut pushed: usize = 0;

        for c in iter {
            if (self.length as usize) >= MAX_HAND_SIZE {
                break;
            }
            if let Some(cout) = self.deck_type().valid_card(c) {
                self.cards[self.length as usize] = cout;
                self.length += 1;
                pushed += 1;
            }
        }
        pushed
    }

    /// Pop `n` [Card]s from the end of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// let v: Vec<Card> = h.pop_n(3).collect();
    /// assert_eq!(v.len(), 3);
    /// ```
    pub fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let count =
            if (self.length as usize) < n {
                self.length as usize
            } else { n };
        let mut v: Vec<Card> = Vec::new();

        for _ in 0..count {
            v.push(self.pop().expect("already checked length"));
        }
        v.into_iter()
    }

    /// Pop all [Card]s from the end of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// let v: Vec<Card> = h.pop_all().collect();
    /// assert_eq!(v.len(), 5);
    /// ```
    pub fn pop_all(&mut self) -> impl Iterator<Item = Card> {
        let v = self.cards[..(self.length as usize)].to_vec();
        self.length = 0;
        v.into_iter()
    }

    /// Replace the [Card]s in the hand with the given collection
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh"));
    /// h.set(cards!("Qh", "Jh"));
    /// assert_eq!(h.to_string(), "QhJh");
    /// ```
    pub fn set<I>(&mut self, iter: I) -> bool
    where I: IntoIterator<Item = Card> {
        self.length = 0;
        0 != self.push_all(iter)
    }

    /// Insert a [Card] at the given index
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Jh", "Th"));
    /// h.insert_at(2, QUEEN_OF_HEARTS);
    /// assert_eq!(h.to_string(), "AhKhQhJhTh");
    /// ```
    pub fn insert_at(&mut self, index: usize, card: Card) -> bool {
        if index <= (self.length as usize) &&
            (self.length as usize) < MAX_HAND_SIZE {

            if let Some(c) = self.deck_type().valid_card(card) {
                for i in (index..(self.length as usize)).rev() {
                    self.cards[i + 1] = self.cards[i];
                }
                self.cards[index] = c;
                self.length += 1;
                return true;
            }
        }
        false
    }

    /// Remove the [Card] at the given index
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// h.remove_at(2);
    /// assert_eq!(h.to_string(), "AhKhJhTh");
    /// ```
    pub fn remove_at(&mut self, index: usize) -> Option<Card> {
        if index >= (self.length as usize) {
            return None;
        }
        let ret = self.cards[index];
        for i in index..(self.length as usize - 1) {
            self.cards[i] = self.cards[i + 1];
        }
        self.length -= 1;
        Some(ret)
    }

    /// Remove the given [Card] from the hand if present
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// h.remove_card(KING_OF_HEARTS);
    /// assert_eq!(h.to_string(), "AhQhJhTh");
    /// ```
    pub fn remove_card(&mut self, card: Card) -> bool {
        for i in 0..(self.length as usize) {
            if self.cards[i] == card {
                self.remove_at(i);
                return true;
            }
        }
        false
    }

    /// Truncate the [Hand] to the given length
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// h.truncate(3);
    /// assert_eq!(h.to_string(), "AhKhQh");
    /// ```
    pub fn truncate(&mut self, n: usize) {
        if n < (self.length as usize) {
            self.length = n as u8;
        }
    }

    /// Shuffle the [Hand] in place
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// h.shuffle();
    /// println!("{}", h); // e.g., "QhJhKhThAh"
    /// ```
    pub fn shuffle(&mut self) {
        oj_shuffle(&mut self.cards[..(self.length as usize)]);
    }

    /// Sort the [Hand] in place, descending by rank, then suit
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let mut h = d.new_hand().init(cards!("Th", "Kh", "Jh", "Ah", "Qh"));
    /// h.sort();
    /// assert_eq!(h.to_string(), "AhKhQhJhTh");
    /// ```
    pub fn sort(&mut self) {
        oj_sort(&mut self.cards[..(self.length as usize)]);
    }

    /// Return an iterator over all `n`-card combinations of the hand
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let h = d.new_hand().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// let mut count = 0;
    /// for sub in h.combinations(3) {
    ///     count += 1;
    ///     println!("{}", sub);    // "AhKhQh", "AhKhJh", ...
    /// }
    /// assert_eq!(count, 10);
    /// ```
    pub fn combinations(&self, k: usize) -> impl Iterator<Item = Hand> {
        CardCombinationIter::new(self, k)
    }

    /// Return true if the hands are identical: i.e., same cards in same order
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h1 = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// let mut h2 = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// assert!(h1.equals(&h2));
    /// h2.set(cards!("Th", "Ah", "Qh", "Jh", "Kh"));
    /// assert!(! h1.equals(&h2));
    /// ```
    pub fn equals(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }
        for i in 0..(self.length as usize) {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }

    /// Return true if the hands are equivalent: i.e., same cards in any order
    /// ```rust
    /// use onejoker::*;
    ///
    /// let h1 = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// let h2 = Hand::default().init(cards!("Th", "Ah", "Qh", "Jh", "Kh"));
    /// assert!(h1.is_equivalent_to(&h2));
    /// ```
    pub fn is_equivalent_to(&self, other: &Self) -> bool {
        if self.length as usize != other.length as usize {
            return false;
        }
        if self.deck_type().dups_allowed() {
            let mut ss: Hand = *self;
            let mut os: Hand = *other;
            oj_sort(&mut ss.cards[..(ss.length as usize)]);
            oj_sort(&mut os.cards[..(os.length as usize)]);

            for i in 0..(self.length as usize) {
                if ss.cards[i] != os.cards[i] {
                    return false;
                }
            }
            return true;
        }
        let mut ss: u64 = 0;
        let mut os: u64 = 0;

        for i in 0..(self.length as usize) {
            ss |= 1 << self.cards[i].0 as u64;
            os |= 1 << other.cards[i].0 as u64;
        }
        ss == os
    }

    /// Fix the ace values in the [Hand] to match the [DeckType].
    /// Used internally--most users should not need this.
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::new(DeckType::Low).init(cards!("Ah", "2h"));
    /// assert_eq!(h[0].rank(), Rank::LowAce);
    /// // Unlike push() and set_card_at(), this does not error check
    /// h[0] = ACE_OF_HEARTS;
    /// assert_eq!(h[0].rank(), Rank::Ace);
    /// h.ace_fix();
    /// assert_eq!(h[0].rank(), Rank::LowAce);
    /// ```
    pub fn ace_fix(&mut self) {
        if self.deck_type().low_aces() {
            for i in 0..(self.length as usize) {
                self.cards[i] = Card::low_ace_fix(self.cards[i]);
            }
        } else {
            for i in 0..(self.length as usize) {
                self.cards[i] = Card::high_ace_fix(self.cards[i]);
            }
        }
    }

    /// Remove the cards at the given indices
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut h = Hand::default().init(cards!("Ah", "Kh", "Qh", "Jh", "Th"));
    /// h.discard(&[1, 3]);
    /// assert_eq!(h.to_string(), "AhQhTh");
    /// ```
    pub fn discard(&mut self, indices: &[usize]) -> bool {
        let mut ok = true;
        let mut v = indices.to_vec();
        oj_sort(&mut v);   // descending is important!

        for i in v {
            if i > self.length as usize {
                ok = false;
            } else {
                self.remove_at(i);
            }
        }
        ok
    }
}

impl Index<usize> for Hand {
    type Output = Card;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

impl IndexMut<usize> for Hand {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cards[index]
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();
        if !self.is_empty() {
            let mut v: Vec<String> = Vec::new();
            for i in 0..(self.length as usize) {
                v.push(self.cards[i].to_string());
            }
            s = v.join("");
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s: String = "".to_string();

        if self.is_not_empty() {
            let mut v: Vec<String> = Vec::new();
            for i in 0..(self.length as usize) {
                v.push(self.cards[i].to_string());
            }
            s = v.join("");
        }
        write!(f, "[{},{}]", self.deck_type().name(), s)
    }
}

impl std::default::Default for Hand {
    fn default() -> Self {
        Hand::new(DeckType::default())
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
        let mut dest: Hand = *hand;
        dest.truncate(k);

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
        Some(self.dest)
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
        let d = Deck::default();
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

        h.set(cards!("4s", "Jc", "9d"));
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
        let d2 = Deck::new(DeckType::OneJoker);
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
        h.push_all([
            QUEEN_OF_CLUBS, KING_OF_CLUBS, ACE_OF_CLUBS
        ]);
        assert_eq!(h.to_string(), "TcJcQcKcAc");

        let list: Vec<Card> = h.pop_n(2).collect();
        assert_eq!(list[0], ACE_OF_CLUBS);
        assert_eq!(list[1], KING_OF_CLUBS);
        assert_eq!(h.to_string(), "TcJcQc");

        /* insert and remove
         */
        h = d2.new_hand().init(cards!("4s", "Jc", "9d"));
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

        h = d2.new_hand().init(ojc_parse("3h5h8dTh3c4h7sJkQs7d"));
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
