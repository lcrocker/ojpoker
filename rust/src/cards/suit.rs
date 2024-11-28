//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Suit) | A simple numeric enum for card suits
//! # Examples
//! ```
//! use onejoker::prelude::*;
//!
//! let s = Suit::Diamond;
//! println!("{}, {}, {}, {}", s.name(), s.to_char(),
//!     s.to_symbol(), Suit::from_char('d'));
//! ```

use crate::error::{Error, Result};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Suit) | A simple numeric enum for card suits
///
/// Specific numbers do matter: I do a lot of math with them to optimize
/// things, and the same numbers are used in the other languages.

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Suit {
    /// Joker, etc.
    None = 0,
    /// Clubs / Batons / Acorns / Wands
    #[default]
    Club = 1,
    /// Diamonds / Coins / Bells / Pentacles
    Diamond = 2,
    /// Hearts / Cups / Roses
    Heart = 3,
    /// Spades / Swords / Leaves / Shields
    Spade = 4,
}

const SUITS: [Suit; 5] = [ Suit::None, Suit::Club, Suit::Diamond,
    Suit::Heart, Suit::Spade ];
const CHARS: [char; 5] = [ '?', 'c', 'd', 'h', 's' ];
const SYMBOLS: [char; 5] = [ '?', '♣', '♦', '♥', '♠' ];
const NAMES: [&str; 5] = [ "?", "club", "diamond", "heart", "spade" ];
const PLURALS: [&str; 5] = [ "?", "clubs", "diamonds", "hearts", "spades" ];

impl Suit {
    /// Convert integer to suit
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(Suit::Club, Suit::from_u8(1));
    /// ```
    pub const fn from_u8(v: u8) -> Suit {
        if v > 4 { return Suit::None; }
        SUITS[v as usize]
    }

    /// Accept ASCII text or Unicode solid/black suit symbols
    /// U+2660, U+2665, U+2666, U+2663
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(Suit::Heart, Suit::from_char('h'));
    /// assert_eq!(Suit::Club, Suit::from_char('♣'));
    /// ```
    pub const fn from_char(c: char) -> Suit {
        match c {
            'c' | '♣' => Suit::Club,
            'd' | '♦' => Suit::Diamond,
            'h' | '♥' => Suit::Heart,
            's' | '♠' => Suit::Spade,
            _ => Suit::None,
        }
    }

    /// Produce the ASCII version
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!('d', Suit::Diamond.to_char());
    /// ```
    pub const fn to_char(&self) -> char {
        if (*self as usize) > 4 { return '?'; }
        CHARS[*self as usize]
    }

    /// Produce the Unicode version
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!('♠', Suit::Spade.to_symbol());
    /// ```
    pub const fn to_symbol(&self) -> char {
        if (*self as usize) > 4 { return '?'; }
        SYMBOLS[*self as usize]
    }

    /// Produce "club", "diamond", etc.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("diamond", Suit::Diamond.name());
    pub const fn name(&self) -> &str {
        if (*self as usize) > 4 { return "?"; }
        NAMES[*self as usize]
    }

    /// Produce "hearts", "spades", etc.
    ///
    /// Not really needed since there are no tricky ones like rank "six",
    /// but other (human) languages may need it.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("spades", Suit::Spade.plural());
    /// ```
    pub const fn plural(&self) -> &str {
        if (*self as usize) > 4 { return "?"; }
        PLURALS[*self as usize]
    }

    /// Likewise, no tricks, but for consistency
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("a", Suit::Club.article());
    /// ```
    pub const fn article(&self) -> &str { "a" }
}

impl std::convert::From<u32> for Suit {
    fn from(v: u32) -> Suit {
        Suit::from_u8(v as u8)
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Suit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let c = s.chars().next().ok_or(
            Error::ParseEmpty(String::from("suit")))?;
        Ok(Suit::from_char(c))
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

    fn has_traits<T: Debug + Display + PartialOrd + PartialEq + Eq + Ord + Clone + Copy +
        std::hash::Hash + std::default::Default + Sized + Send + Sync + Unpin>() {}

    #[test]
    fn test_suits() -> Result<()> {
        has_traits::<Suit>();

        macro_rules! suittests {
            ( $x:ident, $v:literal, $c:literal, $u:literal, $n:literal, $p:literal ) => {
                {
                    use std::str::FromStr;

                    assert_eq!($v, Suit::$x as i32);
                    assert_eq!($c, Suit::$x.to_char());
                    assert_eq!($u, Suit::$x.to_symbol());
                    assert_eq!($n, Suit::$x.name());
                    assert_eq!($p, Suit::$x.plural());
                    assert_eq!(Suit::$x, Suit::from_char($c));
                    assert_eq!(Suit::$x, Suit::from_char($u));
                    assert_eq!(Suit::$x, Suit::from_str(&String::from($c)[..])?);
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

        Ok(())
    }
}

