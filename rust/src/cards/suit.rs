//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Suit) | A simple numeric enum for card suits.
//! # Examples
//! ```
//! use onejoker::cards::Suit;
//! let s = Suit::Diamond;
//! println!("{}, {}, {}, {}", s.name(), s.to_char(),
//!     s.to_symbol(), Suit::from_char('d').unwrap());
//! ```

use crate::errors::*;

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/Suit) | Simple integer enum.
/// Specific numbers do matter: I do a lot of math with
/// them to optimize things, and the same numbers are used in the other
/// languages in the project.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub enum Suit {
    /// Not valid suit, only used internally for special cases.
    None = 0,
    /// Clubs / Batons / Acorns / Wands
    Club = 1,
    /// Diamonds / Coins / Bells / Pentacles
    Diamond = 2,
    /// Hearts / Cups / Roses
    Heart = 3,
    /// Spades / Swords / Leaves / Shields
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
    pub const fn from_i32_const(v: i32) -> Option<Suit> {
        if v < 1 || v > 4 { return None; }
        Some(SUITS[v as usize - 1])
    }

    /// Convert integer to suit.
    pub fn from_i32(v: i32) -> aResult<Suit> {
        if v < 1 || v > 4 {
            bail!(OjError::NotSuit(v.to_string()));
        }
        aOk(SUITS[v as usize - 1])
    }

    /// Accept ASCII text or Unicode solid/black suit symbols
    /// U+2660, U+2665, U+2666, U+2663 
    pub fn from_char(c: char) -> aResult<Suit> {
        aOk(match c {
            'c' | '♣' => Suit::Club,
            'd' | '♦' => Suit::Diamond,
            'h' | '♥' => Suit::Heart,
            's' | '♠' => Suit::Spade,
            _ => bail!(OjError::NotSuit(c.to_string())),
        })
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

#[allow(clippy::from_over_into)]    // From would fail
impl std::convert::Into<i32> for Suit {
    fn into(self) -> i32 {
        self as i32
    }
}

#[allow(clippy::from_over_into)]    // From would fail
impl std::convert::Into<char> for Suit {
    fn into(self) -> char {
        self.to_char()
    }
}

impl std::str::FromStr for Suit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> aResult<Self, Self::Err> {
        let c = s.chars().next().ok_or(
            OjError::ParseEmpty(String::from("suit")))?;
    
        let suit = Suit::from_char(c).or(
            Err(OjError::NotSuit(c.to_string())))?;
        aOk(suit)
    }
}

/*
 * CODE ENDS HERE
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suits() -> aResult<()> {
        macro_rules! suittests {
            ( $x:ident, $v:literal, $c:literal, $u:literal, $n:literal, $p:literal ) => {
                {
                    use std::str::FromStr;

                    assert_eq!($v, Suit::$x as i32);
                    assert_eq!($c, Suit::$x.to_char());
                    assert_eq!($u, Suit::$x.to_symbol());
                    assert_eq!($n, Suit::$x.name());
                    assert_eq!($p, Suit::$x.plural());
                    assert_eq!(Suit::$x, Suit::from_char($c)?);
                    assert_eq!(Suit::$x, Suit::from_char($u)?);
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

        aOk(())
    }
}

