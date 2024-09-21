//@ cards/suit.rs
//@ Lee Daniel Crocker <lee@piclab.com>

//! # suit | [wiki](https://github.com/lcrocker/tspoker/wiki/Suit) | A simple numeric enum for card suits.

use crate::cards::OjError;

/// # Suit | [wiki](https://github.com/lcrocker/tspoker/wiki/Suit)
/// Simple integer enum. Numbers do matter: I do a lot of math with them to optimize
/// things in various other places, and the same numbers are used in the other
/// languages in the OneJoker project.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub enum Suit {
    Club = 1,
    Diamond = 2,
    Heart = 3,
    Spade = 4,
}

const SUITS: [Suit; 4] = [ Suit::Club, Suit::Diamond,
    Suit::Heart, Suit::Spade ];
const CHARS: [char; 4] = [ 'c', 'd', 'h', 's' ];
const SYMBOLS: [char; 4] = [ '♣', '♦', '♥', '♠' ];
const NAMES: [&str; 4] = [ "club", "diamond", "heart", "spade" ];
const PLURALS: [&str; 4] = [ "clubs", "diamonds", "hearts", "spades" ];

impl Suit {
    /// Convert integer to suit.
    pub fn from_i32(v: i32) -> Option<Suit> {
        if !(1..=4).contains(&v) { return None; }
        Some(SUITS[v as usize - 1])
    }

    /// Accept ASCII text or Unicode solid/black suit symbols
    /// U+2660, U+2665, U+2666, U+2663 
    pub fn from_char(c: char) -> Option<Suit> {
        match c {
            'c' | '♣' => Some(Suit::Club),
            'd' | '♦' => Some(Suit::Diamond),
            'h' | '♥' => Some(Suit::Heart),
            's' | '♠' => Some(Suit::Spade),
            _ => None,
        }
    }

    /// Produce the ASCII version.
    pub fn to_char(&self) -> char {
        if *self < Suit::Club || *self > Suit::Spade { return '?'; }
        CHARS[*self as usize - 1]
    }

    /// Produce the Unicode version.
    pub fn to_symbol(&self) -> char {
        if *self < Suit::Club || *self > Suit::Spade { return '?'; }
        SYMBOLS[*self as usize - 1]
    }

    /// Produce "club", "diamond", etc.
    pub fn name(&self) -> &str {
        if *self < Suit::Club || *self > Suit::Spade { return "?"; }
        NAMES[*self as usize - 1]
    }

    /// Produce "hearts", "spades", etc. Not really needed since there are
    /// no tricky ones like rank "six", but other languages may need it.
    pub fn plural(&self) -> &str {
        if *self < Suit::Club || *self > Suit::Spade { return "?"; }
        PLURALS[*self as usize - 1]
    }

    /// Likewise, no tricks, but for consistency.
    pub fn article(&self) -> &str { "a" }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Suit {
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let co = s.chars().next();
        if co.is_none() {
            return Err(OjError::ParseEmpty(String::from("suit")));
        }
        let c = co.unwrap();
    
        let suit = Suit::from_char(c);
        if suit.is_none() {
            return Err(OjError::ParseNotSuit(String::from(c)));
        }
        Ok(suit.unwrap())
    }
}

/*
 * CODE ENDS HERE
 */

#[test]
fn test_suits() {
    macro_rules! suittests {
        ( $x:ident, $v:literal, $c:literal, $u:literal, $n:literal, $p:literal ) => {
            {
                use std::str::FromStr;

                assert_eq!($v, Suit::$x as i32);
                assert_eq!($c, Suit::$x.to_char());
                assert_eq!($u, Suit::$x.to_symbol());
                assert_eq!($n, Suit::$x.name());
                assert_eq!($p, Suit::$x.plural());
                assert_eq!(Suit::$x, Suit::from_char($c).unwrap());
                assert_eq!(Suit::$x, Suit::from_char($u).unwrap());
                assert_eq!(Suit::$x, Suit::from_str(&String::from($c)[..]).unwrap());
            }
        };
    }

    suittests!(Club, 1, 'c', '♣', "club", "clubs");
    suittests!(Diamond, 2, 'd', '♦', "diamond", "diamonds");
    suittests!(Heart, 3, 'h', '♥', "heart", "hearts");
    suittests!(Spade, 4, 's', '♠', "spade", "spades");

    // PartialOrd
    assert!(Suit::Club < Suit::Diamond);
    assert!(Suit::Diamond < Suit::Heart);
    assert!(Suit::Heart < Suit::Spade);

    // Debug, Display
    let s = format!("{} {} {} {}", Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade);
    assert_eq!("Club Diamond Heart Spade", s);
    let d = format!("{:?} {:?} {:?} {:?}", Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade);
    assert_eq!("Club Diamond Heart Spade", d);
}
