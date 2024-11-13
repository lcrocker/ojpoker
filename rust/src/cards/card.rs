//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Card) | A simple card object wrapping a u8.

use paste::paste;
use crate::errors::*;
use crate::cards::rank::*;
use crate::cards::suit::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Ordinal) | Integer representation for cards
/// Cards are represented as integers in the range 1..63
pub type Ordinal = u8;  // some machines might be faster with u32?

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Card) | A simple card object wrapping a u8.
/// A simple new-type wrapper around the `Ordinal` value,
/// which is just an alias for u8.
#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash, Default)]
pub struct Card(pub Ordinal);

#[macro_export]
/// Make const Card object from string. For example, `card!("Ac")`
/// is equivalent to the constant `ACE_OF_CLUBS`.
macro_rules! card {
    ( $x:literal ) => { Card::from_const_str($x) };
}

#[macro_export]
/// Make const array of Card objects from string literals.
/// For example, `cards!("Ac", "2d", "3h")` is equivalent to
/// `[ACE_OF_CLUBS, DEUCE_OF_DIAMONDS, TREY_OF_HEARTS]`.
macro_rules! cards {
    ( $( $x:literal ),* ) => {
        [ $( Card::from_const_str($x), )* ]
    };
}

// These are used in the impl block below to create the methods
// is_club(), is_four(), etc. for each suit and rank.

macro_rules! is_suit {
    ( $x:ident, $v:literal ) => {
        paste! {
            /// Is the card suit $x?
            pub const fn [<is_ $x>](&self) -> bool {
                if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 { return false; }
                $v == self.0 & 3
            }
        }
    };
}

macro_rules! is_rank {
    ( $x:ident, $v:literal ) => {
        paste! {
            /// Is the card rank $
            pub const fn [<is_ $x>](&self) -> bool {
                if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 { return false; }
                $v == (self.0 >> 2)
            }
        }
    };
}

impl Card {
    /// Create a new `Card` from an integer value
    /// ```rust
    /// use onejoker::*;
    ///
    /// let c = Card::from_i32(3).unwrap();
    /// assert_eq!(c, JOKER);
    /// ```
    pub const fn from_i32(v: i32) -> Option<Card> {
        if v < WHITE_JOKER.0 as i32 || v > ACE_OF_SPADES.0 as i32 {
            return None;
        }
        Some(Card(v as Ordinal))
    }

    /// Create a new `Card` from a `Rank` and a `Suit`. If the `Rank` and
    /// `Suit` objects are valid, this cannot fail, so it returns a real `Card`,
    /// not an Option.
    /// ```rust
    /// use onejoker::*;
    ///
    /// let c = Card::from_rank_suit(Rank::Ace, Suit::Spade);
    /// assert_eq!(c, ACE_OF_SPADES);
    /// ```
    #[inline]
    pub const fn from_rank_suit(r: Rank, s: Suit) -> Card {
        debug_assert!(r as u32 <= 15);
        debug_assert!(s as u32 <= 4);
        if r as u32 == 0 || s as u32 == 0 {
            return Card(0);
        }
        Card(((r as Ordinal) << 2) + (s as Ordinal) - 1)
    }

    /// Return a card value unmolested, unless it's a high ace, in which case
    /// return the low ace of the same suit. Mostly for internal use
    /// ```rust
    /// use onejoker::*;
    ///
    /// let c = Card::from_i32(63).unwrap();
    /// assert_eq!(c, ACE_OF_SPADES);
    /// assert_eq!(Card::low_ace_fix(c), LOW_ACE_OF_SPADES);
    /// ```
    #[inline]
    pub const fn low_ace_fix(v: Card) -> Card {
        if v.0 < ACE_OF_CLUBS.0 || v.0 > ACE_OF_SPADES.0 { return v }
        Card(v.0 - ACE_OF_CLUBS.0 + LOW_ACE_OF_CLUBS.0)
    }

    /// Return a card value unmolested, unless it's a low ace, in which case
    /// return the high ace of the same suit. Mostly for internal use
    /// ```rust
    /// use onejoker::*;
    ///
    /// let c = Card::from_i32(4).unwrap();
    /// assert_eq!(c, LOW_ACE_OF_CLUBS);
    /// assert_eq!(Card::high_ace_fix(c), ACE_OF_CLUBS);
    /// ```
    #[inline]
    pub const fn high_ace_fix(v: Card) -> Card {
        if v.0 < LOW_ACE_OF_CLUBS.0 || v.0 > LOW_ACE_OF_SPADES.0 { return v }
        Card(v.0 + ACE_OF_CLUBS.0 - LOW_ACE_OF_CLUBS.0)
    }

    /// Rank of the card, if any. `None` for jokers or illegal values
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(Rank::Deuce, DEUCE_OF_CLUBS.rank());
    /// assert_eq!(Rank::None, BLACK_JOKER.rank());
    /// ```
    #[inline]
    pub const fn rank(&self) -> Rank {
        if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 {
            return Rank::None;
        }
        Rank::from_u8(self.0 >> 2)
    }

    /// Suit of the card if any. `None` for jokers or illegal values
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(Suit::Club, TREY_OF_CLUBS.suit());
    /// assert_eq!(Suit::None, JOKER.suit());
    /// ```
    #[inline]
    pub const fn suit(&self) -> Suit {
        if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 {
            return Suit::None;
        }
        Suit::from_u8((0x03 & self.0) + 1)
    }

    /// Does the object represent an actual card, and not an illegal value?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(ACE_OF_SPADES.is_card());
    /// assert!(JOKER.is_card());
    /// assert!(! Card(0).is_card());
    /// ```
    pub const fn is_card(&self) -> bool {
        self.0 >= WHITE_JOKER.0 && self.0 <= ACE_OF_SPADES.0
    }

    /// Is the card a diamond, heart, or red/colored joker?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(ACE_OF_DIAMONDS.is_red());
    /// assert!(JOKER.is_red());    // Yes, jokers have no suit, but are red/black
    /// assert!(! KING_OF_SPADES.is_red());
    /// ```
    pub const fn is_red(&self) -> bool {
        if self.0 == JOKER.0 { return true }
        if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 { return false }
        1 == (self.0 & 3) || 2 == (self.0 & 3)
    }

    /// Is the card a club, spade, or black joker?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(ACE_OF_CLUBS.is_black());
    /// assert!(BLACK_JOKER.is_black());
    /// assert!(! QUEEN_OF_DIAMONDS.is_black());
    /// ```
    pub const fn is_black(&self) -> bool {
        if self.0 == BLACK_JOKER.0 { return true }
        if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 { return false }
        0 == (self.0 & 3) || 3 == (self.0 & 3)
    }

    /// Is the card any kind of joker?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(JOKER.is_joker());
    /// assert!(BLACK_JOKER.is_joker());
    /// assert!(WHITE_JOKER.is_joker());
    /// assert!(! ACE_OF_CLUBS.is_joker());
    ///
    pub const fn is_joker(&self) -> bool {
        self.0 >= WHITE_JOKER.0 && self.0 <= JOKER.0
    }

    /// Is the card an ace (high or low)?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(ACE_OF_CLUBS.is_ace());
    /// assert!(LOW_ACE_OF_DIAMONDS.is_ace());
    /// assert!(! QUEEN_OF_HEARTS.is_ace());
    /// ```
    pub const fn is_ace(&self) -> bool {
        if self.0 < LOW_ACE_OF_CLUBS.0 || self.0 > ACE_OF_SPADES.0 { return false; }
        self.0 < DEUCE_OF_CLUBS.0 || self.0 > KING_OF_SPADES.0
    }

    is_suit!(club, 0);
    is_suit!(diamond, 1);
    is_suit!(heart, 2);
    is_suit!(spade, 3);

    is_rank!(deuce, 2);
    is_rank!(trey, 3);
    is_rank!(four, 4);
    is_rank!(five, 5);
    is_rank!(six, 6);
    is_rank!(seven, 7);
    is_rank!(eight, 8);
    is_rank!(nine, 9);
    is_rank!(ten, 10);
    is_rank!(jack, 11);
    is_rank!(knight, 12);
    is_rank!(queen, 13);
    is_rank!(king, 14);

    /// Produce text output form with Unicode suit symbol
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(ACE_OF_SPADES.to_unicode(), "A♠");
    /// assert_eq!(JOKER.to_unicode(), "Jk");
    /// ```
    pub fn to_unicode(&self) -> String {
        if ! self.is_card() { return String::from("??") }

        match *self {
            JOKER => { String::from("Jk") },
            BLACK_JOKER => { String::from("Jb") },
            WHITE_JOKER => { String::from("Jw") },
            _ => {
                let mut ret: String = String::new();
                let r: Rank = self.rank();
                let s: Suit = self.suit();

                ret.push(r.to_char());
                ret.push(s.to_symbol());
                ret
            }
        }
    }

    /// Produce the single-character Unicode version of this card
    /// (U+1F0A1..U+1F0DF)
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(ACE_OF_SPADES.to_unicode_single(), "🂡")
    /// ```
    pub fn to_unicode_single(&self) -> String {
        if ! self.is_card() { return String::from(UNICODE_SINGLES[0]); }
        String::from(UNICODE_SINGLES[self.0 as usize - 1])
    }

    /// Full English name of card, e.g. "ace of spades"
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(JACK_OF_DIAMONDS.full_name(), "jack of diamonds");
    /// assert_eq!(JOKER.full_name(), "joker");
    /// ```
    pub fn full_name(&self) -> String {
        match *self {
            BLACK_JOKER => String::from("black joker"),
            WHITE_JOKER => String::from("white joker"),
            JOKER => String::from("joker"),
            _ => format!("{} of {}",
                self.rank().name(), self.suit().plural())
        }
    }

    /// Const function to create a `Card` from a string literal.
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(ACE_OF_CLUBS, Card::from_const_str("Ac"));
    /// assert_eq!(JOKER, Card::from_const_str("Jk"));
    /// ```
    pub const fn from_const_str(st: &str) -> Card {
        let r: char = st.as_bytes()[0] as char;
        let s: char = st.as_bytes()[1] as char;

        if 'J' == r && 'k' == s { return Card(3); }
        if 'J' == r && 'b' == s { return Card(2); }
        if 'J' == r && 'w' == s { return Card(1); }
        Card::from_rank_suit(Rank::from_char(r), Suit::from_char(s))
    }
}

const UNICODE_SINGLES: [&str; 63] = [
    "🃟","🂿","🃏","🃑","🃁","🂱","🂡","🃒","🃂","🂲","🂢","🃓","🃃","🂳","🂣",
    "🃔","🃄","🂴","🂤","🃕","🃅","🂵","🂥","🃖","🃆","🂶","🂦","🃗","🃇","🂷","🂧",
    "🃘","🃈","🂸","🂨","🃙","🃉","🂹","🂩","🃚","🃊","🂺","🂪","🃛","🃋","🂻","🂫",
    "🃜","🃌","🂼","🃜","🃝","🃍","🂽","🂭","🃞","🃎","🂾","🂮","🃑","🃁","🂱","🂡",
];

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 3 { return write!(f, "Jk"); }
        if self.0 == 2 { return write!(f, "Jb"); }
        if self.0 == 1 { return write!(f, "Jw"); }
        write!(f, "{}{}", self.rank().to_char(), self.suit().to_char())
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Card {
    type Err = OjError;

    fn from_str(st: &str) -> Result<Self, Self::Err> {
        let mut chars = st.chars();
        let Some(rc) = chars.next() else {
            return Err(OjError::ParseEmpty(String::from(st)));
        };
        let Some(sc) = chars.next() else {
            return Err(OjError::ParseEmpty(String::from(st)));
        };
        if 'J' == rc && 'k' == sc { return Ok(JOKER); }
        if 'J' == rc && 'b' == sc { return Ok(BLACK_JOKER); }
        if 'J' == rc && 'w' == sc { return Ok(WHITE_JOKER); }

        let r = Rank::from_char(rc);
        if r == Rank::None {
            return Err(OjError::NotRank(String::from(st)));
        }
        let s = Suit::from_char(sc);
        if s == Suit::None {
            return Err(OjError::NotSuit(String::from(st)));
        }
        Ok(Card::from_rank_suit(r, s))
    }
}

macro_rules! cardconst {
    ( $x:ident, $v:literal ) => {
        #[allow(missing_docs)]
        pub const $x: Card = Card($v);
    };
}

cardconst!(WHITE_JOKER, 1);
cardconst!(BLACK_JOKER, 2);
cardconst!(JOKER, 3);
cardconst!(LOW_ACE_OF_CLUBS, 4);
cardconst!(LOW_ACE_OF_DIAMONDS, 5);
cardconst!(LOW_ACE_OF_HEARTS, 6);
cardconst!(LOW_ACE_OF_SPADES, 7);
cardconst!(DEUCE_OF_CLUBS, 8);
cardconst!(DEUCE_OF_DIAMONDS, 9);
cardconst!(DEUCE_OF_HEARTS, 10);
cardconst!(DEUCE_OF_SPADES, 11);
cardconst!(TREY_OF_CLUBS, 12);
cardconst!(TREY_OF_DIAMONDS, 13);
cardconst!(TREY_OF_HEARTS, 14);
cardconst!(TREY_OF_SPADES, 15);
cardconst!(FOUR_OF_CLUBS, 16);
cardconst!(FOUR_OF_DIAMONDS, 17);
cardconst!(FOUR_OF_HEARTS, 18);
cardconst!(FOUR_OF_SPADES, 19);
cardconst!(FIVE_OF_CLUBS, 20);
cardconst!(FIVE_OF_DIAMONDS, 21);
cardconst!(FIVE_OF_HEARTS, 22);
cardconst!(FIVE_OF_SPADES, 23);
cardconst!(SIX_OF_CLUBS, 24);
cardconst!(SIX_OF_DIAMONDS, 25);
cardconst!(SIX_OF_HEARTS, 26);
cardconst!(SIX_OF_SPADES, 27);
cardconst!(SEVEN_OF_CLUBS, 28);
cardconst!(SEVEN_OF_DIAMONDS, 29);
cardconst!(SEVEN_OF_HEARTS, 30);
cardconst!(SEVEN_OF_SPADES, 31);
cardconst!(EIGHT_OF_CLUBS, 32);
cardconst!(EIGHT_OF_DIAMONDS, 33);
cardconst!(EIGHT_OF_HEARTS, 34);
cardconst!(EIGHT_OF_SPADES, 35);
cardconst!(NINE_OF_CLUBS, 36);
cardconst!(NINE_OF_DIAMONDS, 37);
cardconst!(NINE_OF_HEARTS, 38);
cardconst!(NINE_OF_SPADES, 39);
cardconst!(TEN_OF_CLUBS, 40);
cardconst!(TEN_OF_DIAMONDS, 41);
cardconst!(TEN_OF_HEARTS, 42);
cardconst!(TEN_OF_SPADES, 43);
cardconst!(JACK_OF_CLUBS, 44);
cardconst!(JACK_OF_DIAMONDS, 45);
cardconst!(JACK_OF_HEARTS, 46);
cardconst!(JACK_OF_SPADES, 47);
cardconst!(KNIGHT_OF_CLUBS, 48);
cardconst!(KNIGHT_OF_DIAMONDS, 49);
cardconst!(KNIGHT_OF_HEARTS, 50);
cardconst!(KNIGHT_OF_SPADES, 51);
cardconst!(QUEEN_OF_CLUBS, 52);
cardconst!(QUEEN_OF_DIAMONDS, 53);
cardconst!(QUEEN_OF_HEARTS, 54);
cardconst!(QUEEN_OF_SPADES, 55);
cardconst!(KING_OF_CLUBS, 56);
cardconst!(KING_OF_DIAMONDS, 57);
cardconst!(KING_OF_HEARTS, 58);
cardconst!(KING_OF_SPADES, 59);
cardconst!(ACE_OF_CLUBS, 60);
cardconst!(ACE_OF_DIAMONDS, 61);
cardconst!(ACE_OF_HEARTS, 62);
cardconst!(ACE_OF_SPADES, 63);

/*
 * CODE ENDS HERE
 */

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn test_cards() -> Result<(), OjError> {
         macro_rules! cardtests {
            ( $x:ident, $v:literal, $r:ident, $s:ident, $t:literal, $u:literal,
                $laf:literal, $haf:literal, $isj:literal, $isa:literal,
                $isr:literal, $isb:literal, $fn:literal ) => {
                {
                    use std::str::FromStr;

                    assert_eq!($x, Card::from_i32($v).unwrap());
                    assert_eq!($v, $x.0);
                    assert_eq!($x, Card::from_rank_suit(Rank::$r, Suit::$s));
                    assert_eq!(Rank::$r, $x.rank());
                    assert_eq!(Suit::$s, $x.suit());
                    assert_eq!($laf, Card::low_ace_fix($x).0);
                    assert_eq!($haf, Card::high_ace_fix($x).0);
                    assert!($x.is_card());
                    assert_eq!($isj, $x.is_joker());
                    assert_eq!($isa, $x.is_ace());
                    assert_eq!($isr, $x.is_red());
                    assert_eq!($isb, $x.is_black());
                    assert_eq!($t, $x.to_string());
                    assert_eq!($u, $x.to_unicode());
                    assert_eq!(UNICODE_SINGLES[$v as usize - 1], $x.to_unicode_single());

                    if Rank::$r != Rank::LowAce {
                        assert_eq!($x, Card::from_str($t).unwrap());
                        assert_eq!($x, Card::from_str($u).unwrap());
                        assert_eq!($x, card!($t));
                    }
                    assert_eq!($t, format!("{}", $x));
                    assert_eq!($t, format!("{:?}", $x));
                }
            };
        }
        cardtests!(LOW_ACE_OF_CLUBS, 4, LowAce, Club, "Ac", "A♣", 4, 60,
            false, true, false, true, "ace of clubs");
        cardtests!(LOW_ACE_OF_DIAMONDS, 5, LowAce, Diamond, "Ad", "A♦", 5, 61,
            false, true, true, false, "ace of diamonds");
        cardtests!(LOW_ACE_OF_HEARTS, 6, LowAce, Heart, "Ah", "A♥", 6, 62,
            false, true, true, false, "ace of hearts");
        cardtests!(LOW_ACE_OF_SPADES, 7, LowAce, Spade, "As", "A♠", 7, 63,
            false, true, false, true, "ace of spades");
        cardtests!(DEUCE_OF_CLUBS, 8, Deuce, Club, "2c", "2♣", 8, 8,
            false, false, false, true, "deuce of clubs");
        cardtests!(DEUCE_OF_DIAMONDS, 9, Deuce, Diamond, "2d", "2♦", 9, 9,
            false, false, true, false, "deuce of diamonds");
        cardtests!(DEUCE_OF_HEARTS, 10, Deuce, Heart, "2h", "2♥", 10, 10,
            false, false, true, false, "deuce of hearts");
        cardtests!(DEUCE_OF_SPADES, 11, Deuce, Spade, "2s", "2♠", 11, 11,
            false, false, false, true, "deuce of spades");
        cardtests!(TREY_OF_CLUBS, 12, Trey, Club, "3c", "3♣", 12, 12,
            false, false, false, true, "trey of clubs");
        cardtests!(TREY_OF_DIAMONDS, 13, Trey, Diamond, "3d", "3♦", 13, 13,
            false, false, true, false, "trey of diamonds");
        cardtests!(TREY_OF_HEARTS, 14, Trey, Heart, "3h", "3♥", 14, 14,
            false, false, true, false, "trey of hearts");
        cardtests!(TREY_OF_SPADES, 15, Trey, Spade, "3s", "3♠", 15, 15,
            false, false, false, true, "trey of spades");
        cardtests!(FOUR_OF_CLUBS, 16, Four, Club, "4c", "4♣", 16, 16,
            false, false, false, true, "four of clubs");
        cardtests!(FOUR_OF_DIAMONDS, 17, Four, Diamond, "4d", "4♦", 17, 17,
            false, false, true, false, "four of diamonds");
        cardtests!(FOUR_OF_HEARTS, 18, Four, Heart, "4h", "4♥", 18, 18,
            false, false, true, false, "four of hearts");
        cardtests!(FOUR_OF_SPADES, 19, Four, Spade, "4s", "4♠", 19, 19,
            false, false, false, true, "four of spades");
        cardtests!(FIVE_OF_CLUBS, 20, Five, Club, "5c", "5♣", 20, 20,
            false, false, false, true, "five of clubs");
        cardtests!(FIVE_OF_DIAMONDS, 21, Five, Diamond, "5d", "5♦", 21, 21,
            false, false, true, false, "five of diamonds");
        cardtests!(FIVE_OF_HEARTS, 22, Five, Heart, "5h", "5♥", 22, 22,
            false, false, true, false, "five of hearts");
        cardtests!(FIVE_OF_SPADES, 23, Five, Spade, "5s", "5♠", 23, 23,
            false, false, false, true, "five of spades");
        cardtests!(SIX_OF_CLUBS, 24, Six, Club, "6c", "6♣", 24, 24,
            false, false, false, true, "six of clubs");
        cardtests!(SIX_OF_DIAMONDS, 25, Six, Diamond, "6d", "6♦", 25, 25,
            false, false, true, false, "six of diamonds");
        cardtests!(SIX_OF_HEARTS, 26, Six, Heart, "6h", "6♥", 26, 26,
            false, false, true, false, "six of hearts");
        cardtests!(SIX_OF_SPADES, 27, Six, Spade, "6s", "6♠", 27, 27,
            false, false, false, true, "six of spades");
        cardtests!(SEVEN_OF_CLUBS, 28, Seven, Club, "7c", "7♣", 28, 28,
            false, false, false, true, "seven of clubs");
        cardtests!(SEVEN_OF_DIAMONDS, 29, Seven, Diamond, "7d", "7♦", 29, 29,
            false, false, true, false, "seven of diamonds");
        cardtests!(SEVEN_OF_HEARTS, 30, Seven, Heart, "7h", "7♥", 30, 30,
            false, false, true, false, "seven of hearts");
        cardtests!(SEVEN_OF_SPADES, 31, Seven, Spade, "7s", "7♠", 31, 31,
            false, false, false, true, "seven of spades");
        cardtests!(EIGHT_OF_CLUBS, 32, Eight, Club, "8c", "8♣", 32, 32,
            false, false, false, true, "eight of clubs");
        cardtests!(EIGHT_OF_DIAMONDS, 33, Eight, Diamond, "8d", "8♦", 33, 33,
            false, false, true, false, "eight of diamonds");
        cardtests!(EIGHT_OF_HEARTS, 34, Eight, Heart, "8h", "8♥", 34, 34,
            false, false, true, false, "eight of hearts");
        cardtests!(EIGHT_OF_SPADES, 35, Eight, Spade, "8s", "8♠", 35, 35,
            false, false, false, true, "eight of spades");
        cardtests!(NINE_OF_CLUBS, 36, Nine, Club, "9c", "9♣", 36, 36,
            false, false, false, true, "nine of clubs");
        cardtests!(NINE_OF_DIAMONDS, 37, Nine, Diamond, "9d", "9♦", 37, 37,
            false, false, true, false, "nine of diamonds");
        cardtests!(NINE_OF_HEARTS, 38, Nine, Heart, "9h", "9♥", 38, 38,
            false, false, true, false, "nine of hearts");
        cardtests!(NINE_OF_SPADES, 39, Nine, Spade, "9s", "9♠", 39, 39,
            false, false, false, true, "nine of spades");
        cardtests!(TEN_OF_CLUBS, 40, Ten, Club, "Tc", "T♣", 40, 40,
            false, false, false, true, "ten of clubs");
        cardtests!(TEN_OF_DIAMONDS, 41, Ten, Diamond, "Td", "T♦", 41, 41,
            false, false, true, false, "ten of diamonds");
        cardtests!(TEN_OF_HEARTS, 42, Ten, Heart, "Th", "T♥", 42, 42,
            false, false, true, false, "ten of hearts");
        cardtests!(TEN_OF_SPADES, 43, Ten, Spade, "Ts", "T♠", 43, 43,
            false, false, false, true, "ten of spades");
        cardtests!(JACK_OF_CLUBS, 44, Jack, Club, "Jc", "J♣", 44, 44,
            false, false, false, true, "jack of clubs");
        cardtests!(JACK_OF_DIAMONDS, 45, Jack, Diamond, "Jd", "J♦", 45, 45,
            false, false, true, false, "jack of diamonds");
        cardtests!(JACK_OF_HEARTS, 46, Jack, Heart, "Jh", "J♥", 46, 46,
            false, false, true, false, "jack of hearts");
        cardtests!(JACK_OF_SPADES, 47, Jack, Spade, "Js", "J♠", 47, 47,
            false, false, false, true, "jack of spades");
        cardtests!(KNIGHT_OF_CLUBS, 48, Knight, Club, "Cc", "C♣", 48, 48,
            false, false, false, true, "knight of clubs");
        cardtests!(KNIGHT_OF_DIAMONDS, 49, Knight, Diamond, "Cd", "C♦", 49, 49,
            false, false, true, false, "knight of diamonds");
        cardtests!(KNIGHT_OF_HEARTS, 50, Knight, Heart, "Ch", "C♥", 50, 50,
            false, false, true, false, "knight of hearts");
        cardtests!(KNIGHT_OF_SPADES, 51, Knight, Spade, "Cs", "C♠", 51, 51,
            false, false, false, true, "knight of spades");
        cardtests!(QUEEN_OF_CLUBS, 52, Queen, Club, "Qc", "Q♣", 52, 52,
            false, false, false, true, "queen of clubs");
        cardtests!(QUEEN_OF_DIAMONDS, 53, Queen, Diamond, "Qd", "Q♦", 53, 53,
            false, false, true, false, "queen of diamonds");
        cardtests!(QUEEN_OF_HEARTS, 54, Queen, Heart, "Qh", "Q♥", 54, 54,
            false, false, true, false, "queen of hearts");
        cardtests!(QUEEN_OF_SPADES, 55, Queen, Spade, "Qs", "Q♠", 55, 55,
            false, false, false, true, "queen of spades");
        cardtests!(KING_OF_CLUBS, 56, King, Club, "Kc", "K♣", 56, 56,
            false, false, false, true, "king of clubs");
        cardtests!(KING_OF_DIAMONDS, 57, King, Diamond, "Kd", "K♦", 57, 57,
            false, false, true, false, "king of diamonds");
        cardtests!(KING_OF_HEARTS, 58, King, Heart, "Kh", "K♥", 58, 58,
            false, false, true, false, "king of hearts");
        cardtests!(KING_OF_SPADES, 59, King, Spade, "Ks", "K♠", 59, 59,
            false, false, false, true, "king of spades");
        cardtests!(ACE_OF_CLUBS, 60, Ace, Club, "Ac", "A♣", 4, 60,
            false, true, false, true, "ace of clubs");
        cardtests!(ACE_OF_DIAMONDS, 61, Ace, Diamond, "Ad", "A♦", 5, 61,
            false, true, true, false, "ace of diamonds");
        cardtests!(ACE_OF_HEARTS, 62, Ace, Heart, "Ah", "A♥", 6, 62,
            false, true, true, false, "ace of hearts");
        cardtests!(ACE_OF_SPADES, 63, Ace, Spade, "As", "A♠", 7, 63,
            false, true, false, true, "ace of spades");

        macro_rules! cardtests {
            // name, ord, isred, isblack
            ( $x:ident, $v:literal, $isr:literal, $isb:literal, $t:literal, $fn:literal ) => {
                {
                    use std::str::FromStr;

                    assert_eq!($x, Card::from_i32($v).unwrap());
                    assert_eq!($v, $x.0);
                    assert_eq!($v, Card::low_ace_fix($x).0);
                    assert_eq!($v, Card::high_ace_fix($x).0);
                    assert!($x.is_card());
                    assert!($x.is_joker());
                    assert!(! $x.is_ace());
                    assert_eq!($isr, $x.is_red());
                    assert_eq!($isb, $x.is_black());
                    assert_eq!($t, $x.to_string());
                    assert_eq!($t, $x.to_unicode());
                    assert_eq!(UNICODE_SINGLES[$v as usize - 1], $x.to_unicode_single());
                    assert_eq!($x, Card::from_str($t).unwrap());
                    assert_eq!($t, format!("{}", $x));
                    assert_eq!($t, format!("{:?}", $x));
                    assert_eq!($fn, $x.full_name());
                }
            };
        }
        cardtests!(WHITE_JOKER, 1, false, false, "Jw", "white joker");
        cardtests!(BLACK_JOKER, 2, false, true, "Jb", "black joker");
        cardtests!(JOKER, 3, true, false, "Jk", "joker");

        assert_eq!(LOW_ACE_OF_CLUBS, card!("1c"));
        assert_eq!(LOW_ACE_OF_DIAMONDS, card!("1d"));
        assert_eq!(LOW_ACE_OF_HEARTS, card!("1h"));
        assert_eq!(LOW_ACE_OF_SPADES, card!("1s"));

        assert!(! Card(65).is_card());
        assert!(! Card(0).is_card());
        assert!(Card::from_i32(65).is_none());

        assert!(LOW_ACE_OF_SPADES < DEUCE_OF_CLUBS);
        assert!(KING_OF_SPADES < ACE_OF_CLUBS);
        assert!(KNIGHT_OF_CLUBS < KNIGHT_OF_DIAMONDS);

        assert!(DEUCE_OF_CLUBS.is_club());
        assert!(TREY_OF_DIAMONDS.is_diamond());
        assert!(FOUR_OF_HEARTS.is_heart());
        assert!(FIVE_OF_SPADES.is_spade());
        assert!(! SIX_OF_CLUBS.is_heart());
        assert!(! SEVEN_OF_DIAMONDS.is_spade());

        assert!(DEUCE_OF_DIAMONDS.is_deuce());
        assert!(TREY_OF_HEARTS.is_trey());
        assert!(FOUR_OF_SPADES.is_four());
        assert!(FIVE_OF_CLUBS.is_five());
        assert!(SIX_OF_DIAMONDS.is_six());
        assert!(SEVEN_OF_HEARTS.is_seven());
        assert!(EIGHT_OF_SPADES.is_eight());
        assert!(NINE_OF_CLUBS.is_nine());
        assert!(TEN_OF_DIAMONDS.is_ten());
        assert!(JACK_OF_HEARTS.is_jack());
        assert!(KNIGHT_OF_SPADES.is_knight());
        assert!(QUEEN_OF_CLUBS.is_queen());
        assert!(KING_OF_DIAMONDS.is_king());
        assert!(! DEUCE_OF_HEARTS.is_trey());
        assert!(! TREY_OF_SPADES.is_deuce());

        Ok(())
    }
}

