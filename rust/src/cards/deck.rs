//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck) | "Live" deck of cards for play

use crate::error::{Error, Result};
use crate::cards::*;
use crate::utils::{Random, oj_sort, oj_next_combination};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck) | "Live" deck of cards for play
///
/// An array of [Card] objects with methods appropriate for a deck of cards.
/// Note that cards are `pop()`'d from end of the array for speed, making
/// that notionally the "top" of the deck. We show the [Deck] reversed
/// when printing for this reason to bake debugging easier. Cards in the
/// deck are not accessed randomly by index, though they can be removed
/// by value.
#[repr(C)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deck {
    /// Current contents of the deck
    cards: Vec<Card>,
    /// PRNG
    #[serde(skip)]
    rng: Random,
    /// Associated [DeckType]
    deck_type: DeckType,
}

impl Deck {
    /// Create a new deck from the given [DeckType], e.g.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::English);
    /// ```
    pub fn new(t: DeckType) -> Deck {
        Deck {
            cards: t.card_list().to_vec(),
            rng: Random::new(),
            deck_type: t,
        }
    }

    /// Create a new deck from a [DeckType] by name, e.g.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new_by_name("canasta");
    /// ```
    pub fn new_by_name(dname: &str) -> Deck {
        let t = DeckType::by_name(dname);

        Deck {
            cards: t.card_list().to_vec(),
            rng: Random::new(),
            deck_type: t,
        }
    }

    /// Return the [DeckType] associated with this deck
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new_by_name("lowball");
    /// assert_eq!(DeckType::LowJoker, d.deck_type());
    /// ```
    pub fn deck_type(&self) -> DeckType {
        self.deck_type
    }

    /// Set the PRNG seed for this deck
    pub fn reproducible(mut self, seed: u64) -> Self {
        self.rng = Random::new().seeded(seed);
        self
    }

    /// Initial shuffle for new deck
    ///
    /// Returns `self` for chaining.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::English).shuffled();
    /// ```
    pub fn shuffled(mut self) -> Self {
        self.shuffle();
        self
    }

    /// Create a new [Hand] associated with this deck
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::English);
    /// let h = d.new_hand();
    /// ```
    pub fn new_hand(&self) -> Hand {
        Hand::new(self.deck_type)
    }

    /// Export the current contents of the deck
    ///
    /// Returns a new vector of [Card], leaving the deck itself unchanged.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::English).shuffled();
    /// let saved_copy: Vec<Card> = d.to_vec();
    /// ```
    pub fn to_vec(&self) -> Vec<Card> {
        self.cards.to_vec()
    }

    /// Shuffle the deck in place
    ///
    /// Does not refill the deck, but just shuffles whatever cards are
    /// currently in it. There is a separate `refill_and_shuffle` method
    /// for doing both.
    pub fn shuffle(&mut self) {
        for i in (1..self.cards.len()).rev() {
            let j = self.rng.uniform16(i + 1);
            if i != j { self.cards.swap(i, j); }
        }
    }

    /// Refill the deck to its original contents
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::Pinochle).shuffled();
    /// let mut h = d.new_hand().init(d.draw(12));
    /// println!("{}", d.remaining());  // 36
    /// // . . .
    /// d.refill();
    /// println!("{}", d.remaining());  // 48
    /// ```
    pub fn refill(&mut self) {
        self.cards = self.deck_type.card_list().to_vec();
    }

    /// Refill the deck and shuffle
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new_by_name("bridge").shuffled();
    /// let mut h = d.new_hand().init(d.draw(13));
    /// // . . .
    /// d.refill_and_shuffle();
    /// ```
    pub fn refill_and_shuffle(&mut self) {
        self.cards = self.deck_type.card_list().to_vec();
        self.shuffle();
    }

    /// Number of cards currently in the deck
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new_by_name("panguinge").shuffled();
    /// println!("{}, {}", d.size(), d.len());  // 320, 320
    /// let mut h = d.new_hand().init(d.draw(10));
    /// println!("{}, {}", d.size(), d.len());  // 320, 310
    /// ```
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Alias for `len()`
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Return the total number of cards in the full deck
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::Swiss);
    /// println!("{}", d.size());  // 36
    /// ```
    pub fn size(&self) -> usize {
        self.deck_type.size()
    }

    /// Is the deck empty?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let v: Vec<Card> = d.pop_all().collect();
    /// assert!(d.is_empty());
    /// d.refill();
    /// assert!(d.is_not_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Is the deck not empty?
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// assert!(d.is_not_empty());
    /// let v: Vec<Card> = d.pop_all().collect();
    /// assert!(d.is_empty());
    /// ```
    pub fn is_not_empty(&self) -> bool {
        ! self.cards.is_empty()
    }

    /// Does the deck contain the given [Card]?
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let d = Deck::new(DeckType::English);
    /// assert!(d.contains(card!("As")));
    /// assert!(! d.contains(card!("Jk")));
    /// ```
    pub fn contains(&self, card: Card) -> bool {
        let c = self.deck_type.fix_ace(card);
        self.cards.contains(&c)
    }

    /// Push a [Card] onto the deck
    ///
    /// We do not generally expect cards to go in this direction,
    /// but might need to for testing and simulation.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut burn = d.new_hand();
    /// burn.push(d.pop().unwrap());
    /// // oops, put it back
    /// d.push(burn.pop().unwrap());
    /// ```
    pub fn push(&mut self, card: Card) -> bool {
        let c = self.deck_type.valid_card(card);
        self.cards.push(c);
        true
    }

    /// Pop a [Card] from the deck
    ///
    /// Return `None` if the deck is empty.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let c: Card = d.pop().unwrap();
    /// ```
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Push first `n` [Card]s of collection onto the deck
    ///
    /// Return the number actually pushed.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut burn = d.new_hand();
    /// burn.push_n(3, d.pop_n(3));
    /// // oops, put them all back
    /// d.push_n(3, burn.pop_n(3));
    /// ```
    pub fn push_n<I>(&mut self, n: usize, cards: I) -> usize
    where I: IntoIterator<Item = Card> {
        let mut pushed = 0;

        for c in cards {
            if self.push(c) {
                pushed += 1;
            }
            if pushed >= n {
                break;
            }
        }
        pushed
    }

    /// Push a collection of [Card]s onto the deck
    ///
    /// Return the number actually pushed.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut burn = d.new_hand();
    /// burn.push_all(d.draw(3));
    /// // oops, put them all back
    /// d.push_all(burn.pop_all());
    /// ```
    pub fn push_all<I>(&mut self, cards: I) -> usize
    where I: IntoIterator<Item = Card> {
        let mut pushed = 0;

        for c in cards {
            if self.push(c) {
                pushed += 1;
            }
        }
        pushed
    }

    /// Pop `n` cards from the deck as an iterator
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut flop: Vec<Card> = d.pop_n(3).collect();
    /// ```
    pub fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let mut v = Vec::new();

        for _ in 0..n {
            let Some(c) = self.pop() else {
                break;
            };
            v.push(c);
        }
        v.into_iter()
    }

    /// Synonym for `pop_n()`
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// // A common idiom for initial deals:
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut player1 = d.new_hand().init(d.draw(5));
    /// let mut player2 = d.new_hand().init(d.draw(5));
    /// ```
    pub fn draw(&mut self, n: usize) -> impl Iterator<Item = Card> {
        self.pop_n(n)
    }

    /// Pop all cards from the deck as an iterator
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// let mut pile: Vec<Card> = d.pop_all().collect();
    /// assert_eq!(52, pile.len());
    /// assert!(d.is_empty());
    /// ```
    pub fn pop_all(&mut self) -> impl Iterator<Item = Card> + '_ {
        self.cards.drain(..)
    }

    /// Remove a [Card] from the deck by value
    ///
    /// Return `true` if found.
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// assert!(d.remove_card(card!("As")));
    /// assert!(! d.remove_card(card!("Jk")));
    /// ```
    pub fn remove_card(&mut self, card: Card) -> bool {
        for i in 0..self.cards.len() {
            if self.cards[i] == card {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    /// Synonym for `remove_card()`
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// assert!(d.draw_card(card!("As")));
    /// ```
    pub fn draw_card(&mut self, c: Card) -> bool {
        self.remove_card(c)
    }

    /// Take the exactly given [Card]s from the [Deck]
    ///
    /// Useful for simulations and testing.
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let mut p1 = d.new_hand().init(d.draw_hand(hand!("Ac", "Kd")));
    /// let mut p2 = d.new_hand().init(d.draw_hand(hand!("2h", "2s")));
    /// ```
    pub fn draw_hand<I>(&mut self, cards: I) -> impl Iterator<Item = Card>
    where I: IntoIterator<Item = Card> {
        let mut v: Vec<Card> = Vec::new();

        for c in cards {
            if self.remove_card(c) {
                v.push(c);
            }
        }
        v.into_iter()
    }

    /// Sort the deck in place
    ///
    /// Uses the same sort as for hands, which sorts them descending by rank
    /// and then by suit. But remember that cards are `pop()`'d from the end,
    /// so the "top" of the deck is the end of the array, so cards will be
    /// dealt in ascending order.
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English).shuffled();
    /// d.sort();
    /// assert_eq!(card!("2c"), d.pop().unwrap());
    /// ```
    pub fn sort(&mut self) {
        oj_sort(&mut self.cards[..]);
    }

    /// Iterate over combinations
    ///
    /// Iterate over all combinations of `k` cards from those
    /// currently in the deck.
    /// ```rust
    /// #[macro_use] extern crate onejoker;
    /// use onejoker::prelude::*;
    ///
    /// let mut d = Deck::new(DeckType::English);
    /// let p1 = d.new_hand().init(d.draw_hand(hand!("Ac", "Kd")));
    /// let p2 = d.new_hand().init(d.draw_hand(hand!("2h", "2s")));
    /// // Run through  1,712,304 possible Texas Hold'em boards
    /// for h in d.combinations(5) {
    ///    // . . .
    /// }
    /// ```
    pub fn combinations(&self, k: usize) -> impl Iterator<Item = Hand> {
        CardCombinationIter::new(self, k)
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new(DeckType::default())
    }
}

impl std::str::FromStr for Deck {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Deck::new_by_name(s))
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let l = self.len();
        let mut max: usize = l;
        let mut tail: usize = 0;

        if l > 32 {
            max = 29;
            tail = l - 29;
        }
        write!(f, "{} [", self.deck_type.name())?;
        for i in ((l - max)..l).rev() {
            write!(f, "{}", self.cards[i])?;
        }
        if tail > 0 {
            write!(f, "...+{}", tail)?;
        }
        write!(f, "]")
    }
}

impl std::hash::Hash for Deck {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.deck_type.hash(state);
        self.cards.hash(state);
    }
}

impl std::cmp::PartialEq for Deck {
    fn eq(&self, other: &Self) -> bool {
        if self.deck_type != other.deck_type {
            return false;
        }
        if self.len() != other.len() {
            return false;
        }
        for i in 0..self.len() {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        true
    }
}
impl std::cmp::Eq for Deck {}

impl std::cmp::PartialOrd for Deck {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Deck {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.deck_type.cmp(&other.deck_type)
    }
}

impl Deck {
    /// Create a new iterator over the deck.
    pub fn iter(&self) -> CardIter {
        CardIter::new(self.to_vec())
    }
}

impl IntoIterator for Deck {
    type Item = Card;
    type IntoIter = CardIntoIter;

    fn into_iter(self) -> CardIntoIter {
        CardIntoIter::new(self.to_vec())
    }
}

impl<'a> IntoIterator for &'a Deck {
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
    pub fn new(deck: &Deck, k: usize) -> CardCombinationIter {
        let source = deck.to_vec();
        let dest: Hand =
            deck.new_hand().init(source[0..k].iter().cloned());

        debug_assert!(dest.len() == k);
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
            self.dest[i] = self.source[self.indices[i]];
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
    use std::cmp::{PartialOrd, PartialEq, Eq, Ord};
    use std::marker::{Sized, Send, Sync, Unpin};
    use std::fmt::{Debug, Display};

    fn has_traits<T: Debug + Display + PartialOrd + PartialEq + Eq + Ord + Clone +
        std::hash::Hash + std::default::Default + Sized + Send + Sync + Unpin>() {}

    #[test]
    fn test_live_deck() -> Result<()> {
        has_traits::<Deck>();

        let mut d1 = Deck::new(DeckType::English);
        let mut d2 = Deck::new_by_name("52");

        let mut prev = WHITE_JOKER;
        for _ in 0..52 {
            let c = d1.pop().unwrap();
            assert!(c >= prev);
            prev = c;
        }
        assert!(d1.is_empty());
        assert!(d1.pop().is_none());
        d1.push(card!("9d"));
        assert_eq!(NINE_OF_DIAMONDS, d1.pop().unwrap());
        d1.refill();

        assert_eq!(d1.len(), 52);
        assert_eq!(d2.remaining(), 52);
        assert_eq!(d1.size(), 52);

        assert!(d1.contains(FOUR_OF_HEARTS));
        assert!(! d2.contains(JOKER));

        d1.shuffle();
        d2.refill_and_shuffle();
        let _ = format!("{} {:?} {} {:?}", d1, d1, d2, d2);

        let mut h1 = d1.new_hand();
        let mut h2 = d2.new_hand().init(d2.draw(5));

        h1.set(d1.draw(5));
        assert_eq!(47, d1.len());
        assert_eq!(5, h1.len());
        assert_eq!(5, h2.len());

        h2.set(d2.draw(7));
        assert_eq!(40, d2.len());
        assert_eq!(7, h2.len());

        h1.clear();
        assert_eq!(0, h1.len());
        assert!(h1.is_empty());
        let c = h1.card_at(0);
        assert!(c.is_none());

        h1.push_all(hand!("Ac", "2d", "3h"));
        assert!(h1.contains(DEUCE_OF_DIAMONDS));
        h1.insert_at(1, FOUR_OF_SPADES);
        assert_eq!(DEUCE_OF_DIAMONDS, h1.card_at(2).unwrap());
        h1.remove_at(0);

        let v = h1.to_vec();
        assert_eq!(vec![FOUR_OF_SPADES, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS], v);
        h2.set(hand!("4s", "2d", "3h"));
        assert!(h1.equals(&h2));
        h1.push_all([JACK_OF_CLUBS, QUEEN_OF_SPADES]);
        h2.push_all(hand!("Jc", "Qs"));
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

        Ok(())
    }
}
