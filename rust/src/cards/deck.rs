//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck) | "Live" deck of cards for play.

use crate::cards::*;
use crate::utils::*;
use crate::errors::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Deck) | "Live" deck of cards for play.
/// An array of [Card] objects with methods appropriate for a deck of cards.
/// Note that cards are pop()'d from end of the array for speed, making
/// that notionally the "top" of the deck. We show the [Deck] reversed
/// when printing for this reason to bake debugging easier. Cards in the
/// deck are not accessed randomly by index, though they can be removed
/// by value.
#[derive(Clone, Debug)]
pub struct Deck {
    /// Static pointer to associated [MasterDeck]
    pub master: &'static MasterDeck,
    /// Current contents of the deck
    pub cards: Vec<Card>,
}

impl Deck {
    /// Create a new deck from a [MasterDeck] by name.
    /// Filled but not shuffled.
    pub fn new(dname: &str) -> Deck {
        let m = MasterDeck::by_name(dname);
        Deck {
            master: m,
            cards: m.card_list.to_vec(),
        }
    }

    /// Initial shuffle for new deck
    pub fn shuffled(mut self) -> Deck {
        self.shuffle();
        self
    }

    /// Create a new [Hand] associated with the same [MasterDeck],
    /// and deal it some initial cards.
    pub fn new_hand(&self) -> Hand {
        Hand {
            master: self.master,
            cards: Vec::new(),
        }
    }

    /// Export the current contents of the deck as a vector of [Card].
    pub fn to_vec(&self) -> Vec<Card> {
        self.cards.to_vec()
    }

    /// Deal `n` cards from the deck to the given [Hand].
    pub fn deal_to(&mut self, h: &mut Hand, n: usize) -> bool {
        if self.cards.len() < n {
            return false;
        }
        h.push_n(self.pop_n(n));
        true
    }

    /// Validate that the give card is legal for this deck,
    /// or panic if not.
    pub fn valid_card(&self, card: Card) -> Option<Card> {
        let cout: Card = if self.master.low_aces {
            Card::low_ace_fix(card)
        } else {
            Card::high_ace_fix(card)
        };
        if self.master.card_list.contains(&cout) {
            return Some(cout);
        }
        None
    }

    /// Refill the deck from the master list.
    pub fn refill(&mut self) {
        self.cards = self.master.card_list.to_vec();
    }

    /// Refill the deck from the master list and shuffle
    pub fn refill_shuffled(&mut self) {
        self.cards = self.master.card_list.to_vec();
        oj_shuffle(&mut self.cards[..]);
    }

    /// Return the number of cards remaining in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Alias for len().
    pub fn remaining(&self) -> usize {
        self.cards.len()
    }

    /// Return the total number of cards in the full deck.
    pub fn size(&self) -> usize {
        self.master.size()
    }

    /// Is the deck empty?
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Is the deck not empty?
    pub fn is_not_empty(&self) -> bool {
        ! self.cards.is_empty()
    }

    /// Does the deck contain the given [Card]?
    pub fn contains(&self, card: Card) -> bool {
        if let Some(c) = self.valid_card(card) {
            return self.cards.contains(&c);
        }
        false
    }

    /// Push a [Card] onto the deck. We do not generally expects cards
    /// to go in this direction, but it is useful for testing and simulation.
    pub fn push(&mut self, card: Card) -> bool {
        if let Some(c) = self.valid_card(card) {
            self.cards.push(c);
            return true;
        }
        false
    }

    /// Pop a [Card] from the deck. Return `None` if the deck is empty.
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Push a collection of [Card]s onto the deck.
    pub fn push_n<I>(&mut self, cards: I) -> bool
    where I: IntoIterator<Item = Card> {
        let mut all_ok = true;

        for c in cards {
            if !self.push(c) {
                all_ok = false;
            }
        }
        all_ok
    }

    /// Pop `n` cards from the deck as an iterator.
    pub fn pop_n(&mut self, n: usize) -> impl Iterator<Item = Card> {
        let count = if self.len() < n { self.len() } else { n };
        let mut v = Vec::new();

        for _ in 0..count {
            v.push(self.pop().expect("already checked length"));
        }
        v.into_iter()
    }

    /// Synonym for pop_n
    pub fn draw(&mut self, n: usize) -> impl Iterator<Item = Card> {
        self.pop_n(n)
    }

    /// Remove a card from the deck by value. Return `true` if found.
    pub fn remove_card(&mut self, card: Card) -> bool {
        for i in 0..self.cards.len() {
            if self.cards[i] == card {
                self.cards.remove(i);
                return true;
            }
        }
        false
    }

    /// Synonym for remove_card
    pub fn draw_card(&mut self, c: Card) -> bool {
        self.remove_card(c)
    }

    /// Take the given [Card]s from the [Deck] and add them to the hand.
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

    /// Shuffle the deck in place.
    pub fn shuffle(&mut self) {
        oj_shuffle(&mut self.cards[..]);
    }

    /// Sort the deck in place. This uses the same sort as the [Hand] class,
    /// so it's technically descending by rank, but recall that [Deck]s are
    /// printed in reverse, so it will look ascending.
    pub fn sort(&mut self) {
        oj_sort(&mut self.cards[..]);
    }

    /// Iterate 
    pub fn combinations(&self, k: usize) -> impl Iterator<Item = Hand> {
        CardCombinationIter::new(self, k)
    }
}


impl Default for Deck {
    fn default() -> Self {
        let m = MasterDeck::by_name("default");
        Deck {
            master: m,
            cards: m.card_list.to_vec(),
        }
    }
}

impl std::str::FromStr for Deck {
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Deck::new(s))
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
        write!(f, "{} [", self.master.name)?;
        for i in ((l - max)..l).rev() {
            write!(f, "{}", self.cards[i])?;
        }
        if tail > 0 {
            write!(f, "...+{}", tail)?;
        }
        write!(f, "]")
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
        let mut dest: Hand = deck.new_hand();
        dest.push_n(source[0..k].iter().cloned().take(k));
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

    #[test]
    fn test_live_deck() -> Result<(), OjError> {
        let mut d1 = Deck::new("english");
        let mut d2 = Deck::new("52");

        let mut prev = WHITE_JOKER;
        for _ in 0..52 {
            let c = d1.pop().unwrap();
            assert!(c >= prev);
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

        d1.shuffle();
        d2.refill_shuffled();
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
    
        h1.push_n([ACE_OF_CLUBS, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS]);
        assert!(h1.contains(DEUCE_OF_DIAMONDS));
        h1.insert_at(1, FOUR_OF_SPADES);
        assert_eq!(DEUCE_OF_DIAMONDS, h1.card_at(2).unwrap());
        h1.remove_at(0);
    
        let v = h1.to_vec();
        assert_eq!(vec![FOUR_OF_SPADES, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS], v);
        h2.clear();
        h2.push_n(parse_cards("4s2d3h"));
        assert!(h1.equals(&h2));
        h1.push_n([JACK_OF_CLUBS, QUEEN_OF_SPADES]);
        h2.push_n(parse_cards("JcQs"));
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
