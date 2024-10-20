//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Card) | A simple card object wrapping a u8.

use crate::errors::*;
use crate::cards::rank::*;
use crate::cards::suit::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Ordinal) | Integer representation for cards
/// Cards are represented as integers in the range 1..63
pub type Ordinal = u8;  // some machines might be faster with u32?

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Card) | Card class
/// A simple one-element tuple object wrapping an `Ordinal` value,
/// which is just an alias for u8.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub struct Card(pub Ordinal);

const UNICODE_SINGLES: [&str; 63] = [
    "🃟","🂿","🃏","🃑","🃁","🂱","🂡","🃒","🃂","🂲","🂢","🃓","🃃","🂳","🂣",
    "🃔","🃄","🂴","🂤","🃕","🃅","🂵","🂥","🃖","🃆","🂶","🂦","🃗","🃇","🂷","🂧",
    "🃘","🃈","🂸","🂨","🃙","🃉","🂹","🂩","🃚","🃊","🂺","🂪","🃛","🃋","🂻","🂫",
    "🃜","🃌","🂼","🃜","🃝","🃍","🂽","🂭","🃞","🃎","🂾","🂮","🃑","🃁","🂱","🂡",
];

impl Card {
    /// Create a new `Card` from an integer value.
    pub const fn from_i32_const(v: i32) -> Option<Card> {
        if v < WHITE_JOKER.0 as i32 || v > ACE_OF_SPADES.0 as i32 {
            return None;
        }
        Some(Card(v as Ordinal))
    }

    /// Create a new `Card` from an integer value.
    pub fn from_i32(v: i32) -> aResult<Card> {
        if v < WHITE_JOKER.0 as i32 || v > ACE_OF_SPADES.0 as i32 {
            bail!(OjError::NotCard(v.to_string()));
        }
        aOk(Card(v as Ordinal))
    }

    /// Create a new `Card` from a `Rank` and a `Suit`. If the `Rank` and
    /// `Suit` objects are valid, this cannot fail, so it returns a real `Card`,
    /// not an Option.
    pub fn from_rank_suit(r: Rank, s: Suit) -> Card {
        debug_assert!(r >= Rank::LowAce && r <= Rank::Ace);
        debug_assert!(s >= Suit::Club && s <= Suit::Spade);
        Card(((r as Ordinal) << 2) + (s as Ordinal) - 1)
    }

    /// Return a card value unmolested, unless it's a high ace, in which case
    /// return the low ace of the same suit.
    pub fn low_ace_fix(v: Card) -> Card {
        if v < ACE_OF_CLUBS || v > ACE_OF_SPADES { return v }
        Card(v.0 - ACE_OF_CLUBS.0 + LOW_ACE_OF_CLUBS.0)
    }

    /// Return a card value unmolested, unless it's a low ace, in which case
    /// return the high ace of the same suit.
    pub fn high_ace_fix(v: Card) -> Card {
        if v < LOW_ACE_OF_CLUBS || v > LOW_ACE_OF_SPADES { return v }
        Card(v.0 + ACE_OF_CLUBS.0 - LOW_ACE_OF_CLUBS.0)
    }

    /// Rank of the card, if any.
    pub fn rank(&self) -> aResult<Rank> {
        if *self < LOW_ACE_OF_CLUBS || *self > ACE_OF_SPADES {
            bail!(OjError::NotRank(self.to_string()));
        }
        Rank::from_i32((self.0 as i32) >> 2)
    }

    /// Suit of the card if any. `None` for jokers or illegal values.
    pub fn suit(&self) -> aResult<Suit> {
        if *self < LOW_ACE_OF_CLUBS || *self > ACE_OF_SPADES {
            bail!(OjError::NotSuit(self.to_string()));
        }
        Suit::from_i32((0x03 & (self.0 as i32)) + 1)
    }

    /// Does the object represent an actual card, and not a sentinel value?
    pub fn is_card(&self) -> bool {
        *self >= WHITE_JOKER && *self <= ACE_OF_SPADES
    }

    /// Is the card a diamond, heart, or red/colored joker?
    pub fn is_red(&self) -> bool {
        if *self == JOKER { return true }
        if *self < LOW_ACE_OF_CLUBS || *self > ACE_OF_SPADES { return false }
        1 == (self.0 & 3) || 2 == (self.0 & 3)
    }

    /// Is the card a club, spade, or black/generic joker?
    pub fn is_black(&self) -> bool {
        if *self == BLACK_JOKER { return true }
        if *self < LOW_ACE_OF_CLUBS || *self > ACE_OF_SPADES { return false }
        0 == (self.0 & 3) || 3 == (self.0 & 3)
    }

    /// Is the card any kind of joker?
    pub fn is_joker(&self) -> bool {
        *self >= WHITE_JOKER && *self <= JOKER
    }

    /// Is the card an ace (high or low)?
    pub fn is_ace(&self) -> bool {
        if *self < LOW_ACE_OF_CLUBS || *self > ACE_OF_SPADES { return false; }
        *self < DEUCE_OF_CLUBS || *self > KING_OF_SPADES
    }

    /// Produce text output form with Unicode suit symbol.
    pub fn to_unicode(&self) -> String {
        if ! self.is_card() { return String::from("??") }

        match *self {
            JOKER => { String::from("Jk") },
            BLACK_JOKER => { String::from("Jb") },
            WHITE_JOKER => { String::from("Jw") },
            _ => {
                let mut ret: String = String::new();
                let r: Rank = self.rank().expect("already handled jokers");
                let s: Suit = self.suit().expect("already handled jokers");
        
                ret.push(r.to_char());
                ret.push(s.to_symbol());
                ret 
            }
        }
    }

    /// Produce the single-character Unicode version of this card
    /// (U+1F0A1..U+1F0DF)
    pub fn to_unicode_single(&self) -> String {
        if ! self.is_card() { return String::from(UNICODE_SINGLES[0]); }
        String::from(UNICODE_SINGLES[self.0 as usize - 1])
    }

    /// Full English name of card, e.g. "ace of spades"
    pub fn full_name(&self) -> String {
        match *self {
            BLACK_JOKER => String::from("black joker"),
            WHITE_JOKER => String::from("white joker"),
            JOKER => String::from("joker"),
            _ => format!("{} of {}",
                self.rank().expect("already handled jokers").name(),
                self.suit().expect("already handled jokers").plural())
        }
    }
}

struct CardParseIter<'a> {
    source: std::str::Chars<'a>,
    state: i32,
    bracket_allowed: bool,
    done: bool,
}

impl<'a> CardParseIter<'a> {
    pub fn new(chars: std::str::Chars<'a>) -> Self {
        CardParseIter {
            source: chars,
            state: 0,
            bracket_allowed: true,
            done: false,
        }
    }
}

impl<'a> Iterator for CardParseIter<'a> {
    type Item = Card;

    #[allow(unused_assignments)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }
        let mut r: Rank = Rank::Ace;
        self.state = 0;

        let mut loop_guard = 10000;
        loop {
            loop_guard -= 1;
            if loop_guard <= 0 { break; }

            if let Some(c) = self.source.next() {
                match self.state {
                    0 => {
                        if '[' == c {
                            if ! self.bracket_allowed { break; }
                            self.bracket_allowed = false;
                            continue;
                        }
                        if ' ' == c { continue; }
                        self.bracket_allowed = false;
                      
                        if 'J' == c {
                            self.state = 1;
                            continue;
                        }
                        if let Ok(x) = Rank::from_char(c) {
                            r = x;
                            self.state = 2;
                            continue;
                        }
                        break;
                    },
                    1 => {
                        if 'k' == c || 'r' == c {
                            return Some(JOKER);
                        }
                        if 'b' == c {
                            return Some(BLACK_JOKER);
                        }
                        if 'w' == c {
                            return Some(WHITE_JOKER);
                        }
                        r = Rank::Jack;
                        if let Ok(s) = Suit::from_char(c) {
                            return Some(Card::from_rank_suit(r, s));
                        }
                        break;
                    },
                    2 => {
                        let Ok(s) = Suit::from_char(c) else { break; };
                        return Some(Card::from_rank_suit(r, s));
                    },
                    _ => { break; },
                };
            } else {
                break;
            }
        }
        self.done = true;
        None
    }
}

/// If the given string begins with a card representation (e.g. "Qd"),
/// return it and the string position immediately after so we can step
/// through a string to get a whole hand.

pub fn oj_cards_from_text(text: &str) -> impl Iterator<Item = Card> + '_ {
    CardParseIter::new(text.chars())
}

const CARD_NAMES: [&str; 63] = [ "Jw", "Jb", "Jk",
    "Ac", "Ad", "Ah", "As", "2c", "2d", "2h", "2s", "3c", "3d", "3h", "3s",
    "4c", "4d", "4h", "4s", "5c", "5d", "5h", "5s", "6c", "6d", "6h", "6s",
    "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s", "9c", "9d", "9h", "9s",
    "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js", "Cc", "Cd", "Ch", "Cs",
    "Qc", "Qd", "Qh", "Qs", "Kc", "Kd", "Kh", "Ks", "Ac", "Ad", "Ah", "As",  ];

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", CARD_NAMES[self.0 as usize - 1])
    }
}
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> aResult<Self, Self::Err> {
        let mut chars = s.chars();
        let Some(rc) = chars.next() else {
            bail!(OjError::ParseEmpty(String::from(s)));
        };
        let Some(sc) = chars.next() else {
            bail!(OjError::ParseEmpty(String::from(s)));
        };

        if 'J' == rc {
            if 'k' == sc || 'r' == sc { return aOk(JOKER); }
            else if 'b' == sc { return aOk(BLACK_JOKER); }
            else if 'w' == sc { return aOk(WHITE_JOKER); }
        }
        let Ok(r) = Rank::from_char(rc) else {
            bail!(OjError::NotRank(String::from(rc)));
        };
        let Ok(s) = Suit::from_char(sc) else {
            bail!(OjError::NotSuit(String::from(sc)));
        };
        aOk(Card::from_rank_suit(r, s))
    }
}

#[allow(clippy::from_over_into)]
impl std::convert::Into<i32> for Card {
    fn into(self) -> i32 {
        self.0 as i32
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
    fn test_cards() -> aResult<()> {
        macro_rules! cardtests {
            ( $x:ident, $v:literal, $r:ident, $s:ident, $t:literal, $u:literal,
                $laf:literal, $haf:literal, $isj:literal, $isa:literal,
                $isr:literal, $isb:literal, $fn:literal ) => {
                {
                    use std::str::FromStr;
            
                    assert_eq!($x, Card::from_i32($v).unwrap());
                    assert_eq!($v, $x.0);
                    assert_eq!($x, Card::from_rank_suit(Rank::$r, Suit::$s));
                    assert_eq!(Rank::$r, $x.rank().unwrap());
                    assert_eq!(Suit::$s, $x.suit().unwrap());
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
                    assert!($x.suit().is_err());
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
        
        assert!(! Card(65).is_card());
        assert!(! Card(0).is_card());
        assert!(Card::from_i32(65).is_err());

        assert!(LOW_ACE_OF_SPADES < DEUCE_OF_CLUBS);
        assert!(KING_OF_SPADES < ACE_OF_CLUBS);
        assert!(KNIGHT_OF_CLUBS < KNIGHT_OF_DIAMONDS);

        aOk(())
    }

}

