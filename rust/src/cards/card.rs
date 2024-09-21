//@ cards/card.rs
//@ Lee Daniel Crocker <lee@piclab.com>

//! # card | [wiki](https://github.com/lcrocker/tspoker/wiki/Card) | A simple card object wrapping a u8.

use crate::errors::*;
use crate::cards::rank::*;
use crate::cards::suit::*;

/// # Ordinal | [wiki](https://github.com/lcrocker/tspoker/wiki/Ordinal)
/// Cards are represented as integers in the range 0..64:
/// 
/// Number          | Represents
/// ----------------|-----------
/// 1               | White/blue joker
/// 2               | Black/uncolored joker
/// 3               | Joker (generic, or colored/red)
/// 4, 5, 6, 7      | "Low" ace of clubs, diamonds, hearts, spades (see below)
/// 8, 9, 10, 11    | Deuce of clubs, diamonds, hearts, spades
/// 12, 13, 14, 15  | Trey of clubs, diamonds, hearts, spades
/// 16, 17, 18, 19  | Four of clubs, diamonds, hearts, spades
/// 20, 21, 22, 23  | Five of clubs, diamonds, hearts, spades
/// 24, 25, 26, 27  | Six of clubs, diamonds, hearts, spades
/// 28, 29, 30, 31  | Seven of clubs, diamonds, hearts, spades
/// 32, 33, 34, 35  | Eight of clubs, diamonds, hearts, spades
/// 36, 37, 38, 39  | Nine of clubs, diamonds, hearts, spades
/// 40, 41, 42, 43  | Ten of clubs, diamonds, hearts, spades
/// 44, 45, 46, 47  | Jack of clubs, diamonds, hearts, spades
/// 48, 49, 50, 51  | Queen of clubs, diamonds, hearts, spades
/// 52, 53, 54, 55  | King of clubs, diamonds, hearts, spades
/// 56, 57, 58, 59  | "High" ace of clubs, diamonds, hearts, spades
/// 60, 61, 62, 63  | Knight / Cavalier, etc.

pub type Ordinal = u8;  // some machines might be faster with u32?

/// # Card | [wiki](https://github.com/lcrocker/tspoker/wiki/Card)
/// A simple one-element tuple object wrapping an `Ordinal` value,
/// which is just an alias for u8 (although it should be easy to
/// change to u32 if that's faster on some machines).
/// Rust is good about optimizing away any overhead for one-member
/// tuple objects like this.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Card(pub Ordinal);

const UNICODE_SINGLES: [&str; 64] = [
    "⁇","🃟","🂿","🃏","🃑","🃁","🂱","🂡","🃒","🃂","🂲","🂢","🃓","🃃","🂳","🂣",
    "🃔","🃄","🂴","🂤","🃕","🃅","🂵","🂥","🃖","🃆","🂶","🂦","🃗","🃇","🂷","🂧",
    "🃘","🃈","🂸","🂨","🃙","🃉","🂹","🂩","🃚","🃊","🂺","🂪","🃛","🃋","🂻","🂫",
    "🃝","🃍","🂽","🂭","🃞","🃎","🂾","🂮","🃑","🃁","🂱","🂡","🃜","🃌","🂼","🃜",
];

impl Card {
    /// Create a new `Card` from an integer value. Should be basically a no-op,
    /// but must fail for invalid values, so returns an `Option`.
    #[inline]
    pub const fn from_i32(v: i32) -> Option<Card> {
        if v < 1 || v > 63 { return None }
        Some(Card(v as Ordinal))
    }

    /// Create a new `Card` from a `Rank` and a `Suit`. If the `Rank` and
    /// `Suit` objects are valid, this cannot fail, so it returns a real `Card`,
    /// not an Option.
    pub fn from_rank_suit(r: Rank, s: Suit) -> Card {
        debug_assert!((r as Ordinal) > 0 && (r as Ordinal) < 16);
        debug_assert!((s as Ordinal) > 0 && (s as Ordinal) < 5);
        Card(((r as Ordinal) << 2) + (s as Ordinal) - 1)
    }

    /// Return a card value unmolested, unless it's a high ace, in which case
    /// return the low ace of the same suit.
    pub fn low_ace_fix(v: Card) -> Card {
        if v < ACE_OF_CLUBS || v > ACE_OF_SPADES { return v }
        Card(v.0 - 52)
    }

    /// Return a card value unmolested, unless it's a low ace, in which case
    /// return the high ace of the same suit.
    pub fn high_ace_fix(v: Card) -> Card {
        if v < LOW_ACE_OF_CLUBS || v > LOW_ACE_OF_SPADES { return v }
        Card(v.0 + 52)
    }

    /// Rank of the card, if any. `None` for illegal values.
    pub fn rank(&self) -> Option<Rank> {
        if *self < LOW_ACE_OF_CLUBS || *self > KNIGHT_OF_SPADES { return None }
        Rank::from_i32((self.0 as i32) >> 2)
    }

    /// Suit of the card if any. `None` for jokers or illegal values.
    pub fn suit(&self) -> Option<Suit> {
        if *self < LOW_ACE_OF_CLUBS || *self > KNIGHT_OF_SPADES { return None }
        Suit::from_i32((0x03 & (self.0 as i32)) + 1)
    }

    /// Does the object represent an actual card, and not a sentinel value?
    pub fn is_card(&self) -> bool {
        *self >= WHITE_JOKER && *self <= KNIGHT_OF_SPADES
    }

    /// Is the card a diamond, heart, or red/colored joker?
    pub fn is_red(&self) -> bool {
        if *self == JOKER { return true }
        if *self < LOW_ACE_OF_CLUBS || *self > KNIGHT_OF_SPADES { return false }
        1 == (self.0 & 3) || 2 == (self.0 & 3)
    }

    /// Is the card a club, spade, or black/generic joker?
    pub fn is_black(&self) -> bool {
        if *self == BLACK_JOKER { return true }
        if *self < LOW_ACE_OF_CLUBS || *self > KNIGHT_OF_SPADES { return false }
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
                let mut r: Rank = self.rank().unwrap();
                let s: Suit = self.suit().unwrap();
        
                if r == Rank::LowAce { r = Rank::Ace }
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
        String::from(UNICODE_SINGLES[self.0 as usize])
    }

    pub fn full_name(&self) -> String {
        match *self {
            BLACK_JOKER => String::from("black joker"),
            WHITE_JOKER => String::from("white joker"),
            JOKER => String::from("joker"),
            _ => format!("{} of {}", self.rank().unwrap().name(),
                self.suit().unwrap().plural())
        }
    }
}

/// If the given string begins with a card representation (e.g. "Qd"),
/// return it and the string position immediately after so we can step
/// through a string to get a whole hand.

pub fn cards_from_text(text: &str) -> Vec<Card> {
    let mut ret: Vec<Card> = Vec::new();
    let mut state: i32 = 0;
    let mut bracket_allowed = true;
    let mut mr: Option<Rank> = None;
    let mut ms: Option<Suit>;

    for c in text.chars() {
        match state {
            0 => {
                if '[' == c {
                    if ! bracket_allowed { return ret; }
                    bracket_allowed = false;
                    continue;
                }
                if ' ' == c { continue; }
                bracket_allowed = false;
          
                if 'J' == c {
                    state = 1;
                    continue;
                }
                mr = Rank::from_char(c);
                if mr.is_none() { return ret; }
                state = 2
            },
            1 => {
                if 'k' == c || 'r' == c {
                    ret.push(JOKER);
                    state = 0;
                    continue;
                }
                if 'b' == c {
                    ret.push(BLACK_JOKER);
                    state = 0;
                    continue;
                }
                if 'w' == c {
                    ret.push(WHITE_JOKER);
                    state = 0;
                    continue;
                }
                mr = Some(Rank::Jack);
                ms = Suit::from_char(c);
                if ms.is_none() { return ret; }
                ret.push(Card::from_rank_suit(mr.unwrap(), ms.unwrap()));
                state = 0
            },
            2 => {
                debug_assert!(mr.is_some());
                ms = Suit::from_char(c);
                if ms.is_none() { return ret; }
                ret.push(Card::from_rank_suit(mr.unwrap(), ms.unwrap()));
                state = 0
            },
            _ => { return ret },
        };
    }
    ret
}

const CARD_NAMES: [&str; 63] = [ "Jw", "Jb", "Jk",
    "Ac", "Ad", "Ah", "As", "2c", "2d", "2h", "2s", "3c", "3d", "3h", "3s",
    "4c", "4d", "4h", "4s", "5c", "5d", "5h", "5s", "6c", "6d", "6h", "6s",
    "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s", "9c", "9d", "9h", "9s",
    "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js", "Qc", "Qd", "Qh", "Qs",
    "Kc", "Kd", "Kh", "Ks", "Ac", "Ad", "Ah", "As", "Cc", "Cd", "Ch", "Cs", ];

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
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rs = s.chars().nth(0);
        if rs.is_none() { return Err(OjError::ParseEmpty(String::from(s))); }
        let ss = s.chars().nth(1);
        if ss.is_none() { return Err(OjError::ParseEmpty(String::from(s))); }

        let rc = rs.unwrap();
        let sc = ss.unwrap();

        if 'J' == rc {
            if 'k' == sc || 'r' == sc { return Ok(JOKER); }
            else if 'b' == sc { return Ok(BLACK_JOKER); }
            else if 'w' == sc { return Ok(WHITE_JOKER); }
        }
        let mr = Rank::from_char(rc);
        if mr.is_none() {
            return Err(OjError::ParseNotRank(String::from(rc)));
        }
        let ms = Suit::from_char(sc);
        if ms.is_none() {
            return Err(OjError::ParseNotSuit(String::from(sc)));
        }
        Ok(Card::from_rank_suit(mr.unwrap(), ms.unwrap()))
    }
}

pub const WHITE_JOKER: Card = Card(1);
pub const BLACK_JOKER: Card = Card(2);
pub const JOKER: Card = Card(3);
pub const LOW_ACE_OF_CLUBS: Card = Card(4);
pub const LOW_ACE_OF_DIAMONDS: Card = Card(5);
pub const LOW_ACE_OF_HEARTS: Card = Card(6);
pub const LOW_ACE_OF_SPADES: Card = Card(7);
pub const DEUCE_OF_CLUBS: Card = Card(8);
pub const DEUCE_OF_DIAMONDS: Card = Card(9);
pub const DEUCE_OF_HEARTS: Card = Card(10);
pub const DEUCE_OF_SPADES: Card = Card(11);
pub const TREY_OF_CLUBS: Card = Card(12);
pub const TREY_OF_DIAMONDS: Card = Card(13);
pub const TREY_OF_HEARTS: Card = Card(14);
pub const TREY_OF_SPADES: Card = Card(15);
pub const FOUR_OF_CLUBS: Card = Card(16);
pub const FOUR_OF_DIAMONDS: Card = Card(17);
pub const FOUR_OF_HEARTS: Card = Card(18);
pub const FOUR_OF_SPADES: Card = Card(19);
pub const FIVE_OF_CLUBS: Card = Card(20);
pub const FIVE_OF_DIAMONDS: Card = Card(21);
pub const FIVE_OF_HEARTS: Card = Card(22);
pub const FIVE_OF_SPADES: Card = Card(23);
pub const SIX_OF_CLUBS: Card = Card(24);
pub const SIX_OF_DIAMONDS: Card = Card(25);
pub const SIX_OF_HEARTS: Card = Card(26);
pub const SIX_OF_SPADES: Card = Card(27);
pub const SEVEN_OF_CLUBS: Card = Card(28);
pub const SEVEN_OF_DIAMONDS: Card = Card(29);
pub const SEVEN_OF_HEARTS: Card = Card(30);
pub const SEVEN_OF_SPADES: Card = Card(31);
pub const EIGHT_OF_CLUBS: Card = Card(32);
pub const EIGHT_OF_DIAMONDS: Card = Card(33);
pub const EIGHT_OF_HEARTS: Card = Card(34);
pub const EIGHT_OF_SPADES: Card = Card(35);
pub const NINE_OF_CLUBS: Card = Card(36);
pub const NINE_OF_DIAMONDS: Card = Card(37);
pub const NINE_OF_HEARTS: Card = Card(38);
pub const NINE_OF_SPADES: Card = Card(39);
pub const TEN_OF_CLUBS: Card = Card(40);
pub const TEN_OF_DIAMONDS: Card = Card(41);
pub const TEN_OF_HEARTS: Card = Card(42);
pub const TEN_OF_SPADES: Card = Card(43);
pub const JACK_OF_CLUBS: Card = Card(44);
pub const JACK_OF_DIAMONDS: Card = Card(45);
pub const JACK_OF_HEARTS: Card = Card(46);
pub const JACK_OF_SPADES: Card = Card(47);
pub const QUEEN_OF_CLUBS: Card = Card(48);
pub const QUEEN_OF_DIAMONDS: Card = Card(49);
pub const QUEEN_OF_HEARTS: Card = Card(50);
pub const QUEEN_OF_SPADES: Card = Card(51);
pub const KING_OF_CLUBS: Card = Card(52);
pub const KING_OF_DIAMONDS: Card = Card(53);
pub const KING_OF_HEARTS: Card = Card(54);
pub const KING_OF_SPADES: Card = Card(55);
pub const ACE_OF_CLUBS: Card = Card(56);
pub const ACE_OF_DIAMONDS: Card = Card(57);
pub const ACE_OF_HEARTS: Card = Card(58);
pub const ACE_OF_SPADES: Card = Card(59);
pub const KNIGHT_OF_CLUBS: Card = Card(60);
pub const KNIGHT_OF_DIAMONDS: Card = Card(61);
pub const KNIGHT_OF_HEARTS: Card = Card(62);
pub const KNIGHT_OF_SPADES: Card = Card(63);
pub const HIGH_SENTINEL: Card = Card(64);

/*
 * CODE ENDS HERE
 */

 #[test]
fn test_card() {
    macro_rules! cardtests {
        // name, ord, rank, suit, lowacefix, highacefix, knightfix, isjoker, isace, isred, isblack
        ( $x:ident, $v:literal, $r:ident, $s:ident, $t:literal, $u:literal,
            $laf:literal, $haf:literal, $kf:literal, $isj:literal, $isa:literal,
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
                assert_eq!(UNICODE_SINGLES[$v as usize], $x.to_unicode_single());
                if Rank::$r != Rank::LowAce {
                    assert_eq!($x, Card::from_str($t).unwrap());
                    assert_eq!($x, Card::from_str($u).unwrap());
                }
                assert_eq!($t, format!("{}", $x));
                assert_eq!($t, format!("{:?}", $x));
            }
        };
    }
    cardtests!(LOW_ACE_OF_CLUBS, 4, LowAce, Club, "Ac", "A♣", 4, 56, 4,
        false, true, false, true, "ace of clubs");
    cardtests!(LOW_ACE_OF_DIAMONDS, 5, LowAce, Diamond, "Ad", "A♦", 5, 57, 5,
        false, true, true, false, "ace of diamonds");
    cardtests!(LOW_ACE_OF_HEARTS, 6, LowAce, Heart, "Ah", "A♥", 6, 58, 6,
        false, true, true, false, "ace of hearts");
    cardtests!(LOW_ACE_OF_SPADES, 7, LowAce, Spade, "As", "A♠", 7, 59, 7,
        false, true, false, true, "ace of spades");
    cardtests!(DEUCE_OF_CLUBS, 8, Deuce, Club, "2c", "2♣", 8, 8, 8,
        false, false, false, true, "deuce of clubs");
    cardtests!(DEUCE_OF_DIAMONDS, 9, Deuce, Diamond, "2d", "2♦", 9, 9, 9,
        false, false, true, false, "deuce of diamonds");
    cardtests!(DEUCE_OF_HEARTS, 10, Deuce, Heart, "2h", "2♥", 10, 10, 10,
        false, false, true, false, "deuce of hearts");
    cardtests!(DEUCE_OF_SPADES, 11, Deuce, Spade, "2s", "2♠", 11, 11, 11,
        false, false, false, true, "deuce of spades");
    cardtests!(TREY_OF_CLUBS, 12, Trey, Club, "3c", "3♣", 12, 12, 12,
        false, false, false, true, "trey of clubs");
    cardtests!(TREY_OF_DIAMONDS, 13, Trey, Diamond, "3d", "3♦", 13, 13, 13,
        false, false, true, false, "trey of diamonds");
    cardtests!(TREY_OF_HEARTS, 14, Trey, Heart, "3h", "3♥", 14, 14, 14,
        false, false, true, false, "trey of hearts");
    cardtests!(TREY_OF_SPADES, 15, Trey, Spade, "3s", "3♠", 15, 15, 15,
        false, false, false, true, "trey of spades");
    cardtests!(FOUR_OF_CLUBS, 16, Four, Club, "4c", "4♣", 16, 16, 16,
        false, false, false, true, "four of clubs");
    cardtests!(FOUR_OF_DIAMONDS, 17, Four, Diamond, "4d", "4♦", 17, 17, 17,
        false, false, true, false, "four of diamonds");
    cardtests!(FOUR_OF_HEARTS, 18, Four, Heart, "4h", "4♥", 18, 18, 18,
        false, false, true, false, "four of hearts");
    cardtests!(FOUR_OF_SPADES, 19, Four, Spade, "4s", "4♠", 19, 19, 19,
        false, false, false, true, "four of spades");
    cardtests!(FIVE_OF_CLUBS, 20, Five, Club, "5c", "5♣", 20, 20, 20,
        false, false, false, true, "five of clubs");
    cardtests!(FIVE_OF_DIAMONDS, 21, Five, Diamond, "5d", "5♦", 21, 21, 21,
        false, false, true, false, "five of diamonds");
    cardtests!(FIVE_OF_HEARTS, 22, Five, Heart, "5h", "5♥", 22, 22, 22,
        false, false, true, false, "five of hearts");
    cardtests!(FIVE_OF_SPADES, 23, Five, Spade, "5s", "5♠", 23, 23, 23,
        false, false, false, true, "five of spades");
    cardtests!(SIX_OF_CLUBS, 24, Six, Club, "6c", "6♣", 24, 24, 24,
        false, false, false, true, "six of clubs");
    cardtests!(SIX_OF_DIAMONDS, 25, Six, Diamond, "6d", "6♦", 25, 25, 25,
        false, false, true, false, "six of diamonds");
    cardtests!(SIX_OF_HEARTS, 26, Six, Heart, "6h", "6♥", 26,  26, 26,
        false, false, true, false, "six of hearts");
    cardtests!(SIX_OF_SPADES, 27, Six, Spade, "6s", "6♠", 27, 27, 27,
        false, false, false, true, "six of spades");
    cardtests!(SEVEN_OF_CLUBS, 28, Seven, Club, "7c", "7♣", 28, 28, 28,
        false, false, false, true, "seven of clubs");
    cardtests!(SEVEN_OF_DIAMONDS, 29, Seven, Diamond, "7d", "7♦", 29, 29, 29,
        false, false, true, false, "seven of diamonds");
    cardtests!(SEVEN_OF_HEARTS, 30, Seven, Heart, "7h", "7♥", 30, 30, 30,
        false, false, true, false, "seven of hearts");
    cardtests!(SEVEN_OF_SPADES, 31, Seven, Spade, "7s", "7♠", 31, 31, 31,
        false, false, false, true, "seven of spades");
    cardtests!(EIGHT_OF_CLUBS, 32, Eight, Club, "8c", "8♣", 32, 32, 32,
        false, false, false, true, "eight of clubs");
    cardtests!(EIGHT_OF_DIAMONDS, 33, Eight, Diamond, "8d", "8♦", 33, 33, 33,
        false, false, true, false, "eight of diamonds");
    cardtests!(EIGHT_OF_HEARTS, 34, Eight, Heart, "8h", "8♥", 34, 34, 34,
        false, false, true, false, "eight of hearts");
    cardtests!(EIGHT_OF_SPADES, 35, Eight, Spade, "8s", "8♠", 35, 35, 35,
        false, false, false, true, "eight of spades");
    cardtests!(NINE_OF_CLUBS, 36, Nine, Club, "9c", "9♣", 36, 36, 36,
        false, false, false, true, "nine of clubs");
    cardtests!(NINE_OF_DIAMONDS, 37, Nine, Diamond, "9d", "9♦", 37, 37, 37,
        false, false, true, false, "nine of diamonds");
    cardtests!(NINE_OF_HEARTS, 38, Nine, Heart, "9h", "9♥", 38, 38, 38,
        false, false, true, false, "nine of hearts");
    cardtests!(NINE_OF_SPADES, 39, Nine, Spade, "9s", "9♠", 39, 39, 39,
        false, false, false, true, "nine of spades");
    cardtests!(TEN_OF_CLUBS, 40, Ten, Club, "Tc", "T♣", 40, 40, 40,
        false, false, false, true, "ten of clubs");
    cardtests!(TEN_OF_DIAMONDS, 41, Ten, Diamond, "Td", "T♦", 41, 41, 41,
        false, false, true, false, "ten of diamonds");
    cardtests!(TEN_OF_HEARTS, 42, Ten, Heart, "Th", "T♥", 42, 42, 42,
        false, false, true, false, "ten of hearts");
    cardtests!(TEN_OF_SPADES, 43, Ten, Spade, "Ts", "T♠", 43, 43, 43,
        false, false, false, true, "ten of spades");
    cardtests!(JACK_OF_CLUBS, 44, Jack, Club, "Jc", "J♣", 44, 44, 44,
        false, false, false, true, "jack of clubs");
    cardtests!(JACK_OF_DIAMONDS, 45, Jack, Diamond, "Jd", "J♦", 45, 45, 45,
        false, false, true, false, "jack of diamonds");
    cardtests!(JACK_OF_HEARTS, 46, Jack, Heart, "Jh", "J♥", 46, 46, 46,
        false, false, true, false, "jack of hearts");
    cardtests!(JACK_OF_SPADES, 47, Jack, Spade, "Js", "J♠", 47, 47, 47,
        false, false, false, true, "jack of spades");
    cardtests!(QUEEN_OF_CLUBS, 48, Queen, Club, "Qc", "Q♣", 48, 48, 48,
        false, false, false, true, "queen of clubs");
    cardtests!(QUEEN_OF_DIAMONDS, 49, Queen, Diamond, "Qd", "Q♦", 49, 49, 49,
        false, false, true, false, "queen of diamonds");
    cardtests!(QUEEN_OF_HEARTS, 50, Queen, Heart, "Qh", "Q♥", 50, 50, 50,
        false, false, true, false, "queen of hearts");
    cardtests!(QUEEN_OF_SPADES, 51, Queen, Spade, "Qs", "Q♠", 51, 51, 51,
        false, false, false, true, "queen of spades");
    cardtests!(KING_OF_CLUBS, 52, King, Club, "Kc", "K♣", 52, 52, 52,
        false, false, false, true, "king of clubs");
    cardtests!(KING_OF_DIAMONDS, 53, King, Diamond, "Kd", "K♦", 53, 53, 53,
        false, false, true, false, "king of diamonds");
    cardtests!(KING_OF_HEARTS, 54, King, Heart, "Kh", "K♥", 54, 54, 54,
        false, false, true, false, "king of hearts");
    cardtests!(KING_OF_SPADES, 55, King, Spade, "Ks", "K♠", 55, 55, 55,
        false, false, false, true, "king of spades");
    cardtests!(ACE_OF_CLUBS, 56, Ace, Club, "Ac", "A♣", 4, 56, 56,
        false, true, false, true, "ace of clubs");
    cardtests!(ACE_OF_DIAMONDS, 57, Ace, Diamond, "Ad", "A♦", 5, 57, 57,
        false, true, true, false, "ace of diamonds");
    cardtests!(ACE_OF_HEARTS, 58, Ace, Heart, "Ah", "A♥", 6, 58, 58,
        false, true, true, false, "ace of hearts");
    cardtests!(ACE_OF_SPADES, 59, Ace, Spade, "As", "A♠", 7, 59, 59,
        false, true, false, true, "ace of spades");
    cardtests!(KNIGHT_OF_CLUBS, 60, Knight, Club, "Cc", "C♣", 60, 60, 48,
        false, false, false, true, "knight of clubs");
    cardtests!(KNIGHT_OF_DIAMONDS, 61, Knight, Diamond, "Cd", "C♦", 61, 61, 49,
        false, false, true, false, "knight of diamonds");
    cardtests!(KNIGHT_OF_HEARTS, 62, Knight, Heart, "Ch", "C♥", 62, 62, 50,
        false, false, true, false, "knight of hearts");
    cardtests!(KNIGHT_OF_SPADES, 63, Knight, Spade, "Cs", "C♠", 63, 63, 51,
        false, false, false, true, "knight of spades");

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
                assert!($x.suit().is_none());
                assert_eq!($t, $x.to_string());
                assert_eq!($t, $x.to_unicode());
                assert_eq!(UNICODE_SINGLES[$v as usize], $x.to_unicode_single());
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
    assert!(Card::from_i32(65).is_none());

    assert!(LOW_ACE_OF_SPADES < DEUCE_OF_CLUBS);
    assert!(KING_OF_SPADES < ACE_OF_CLUBS);
    assert!(KNIGHT_OF_CLUBS < KNIGHT_OF_DIAMONDS);
}
