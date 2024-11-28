//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Rank) | Simple numeric enum for card ranks
//! # Examples
//! ```
//! use onejoker::prelude::*;
//!
//! let r = Rank::Six;
//! println!("{}, {}, {}", r.name(), r.to_char(),
//!     Rank::from_char('J'));
//! ```

use crate::error::{Error,Result};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// [wiki](https://github.com/lcrocker/tspoker/wiki/Rank) | Simple numeric enum for card ranks
///
/// Simple integer enum. Specific numbers do matter: I do a lot of math with
/// them to optimize things, and the same numbers are used in the other
/// languages in the project.
///
/// Note that there are two slots for aces and a slot for knight/cavalier.
/// See [Ace](https://github.com/lcrocker/ojpoker/wiki/Ace) and
/// [Knight](https://github.com/lcrocker/ojpoker/wiki/Knight) @ wiki
/// for details.

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub enum Rank {
    None = 0,
    LowAce = 1,
    #[default]
    Deuce = 2,
    Trey = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Knight = 12,
    Queen = 13,
    King = 14,
    Ace = 15,
}

const RANKS: [Rank; 16] = [ Rank::None, Rank::LowAce, Rank::Deuce,
    Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
    Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack,  Rank::Knight,
    Rank::Queen, Rank::King, Rank::Ace,];
const CHARS: [char; 16] = [ '?', 'A', '2', '3', '4', '5', '6', '7', '8',
    '9', 'T', 'J', 'C', 'Q', 'K', 'A' ];
const NAMES: [&str; 16] = [ "?", "ace", "deuce", "trey", "four",
    "five", "six", "seven", "eight", "nine", "ten", "jack", "knight",
    "queen", "king", "ace" ];
const PLURALS: [&str; 16] = [ "?", "aces", "deuces", "treys", "fours",
    "fives", "sixes", "sevens", "eights", "nines", "tens", "jacks",
    "knights", "queens", "kings", "aces" ];

impl Rank {
    /// Convert integer to rank
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(Rank::Deuce, Rank::from_u8(2));
    /// ```
    pub const fn from_u8(v: u8) -> Rank {
        if v > 15 { return Rank::None; }
        RANKS[v as usize]
    }

    /// Convert character to rank
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!(Rank::Trey, Rank::from_char('3'));
    /// ```
    pub const fn from_char(c: char) -> Rank {
        match c {
            '1' => Rank::LowAce,
            '2' => Rank::Deuce,
            '3' => Rank::Trey,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'C' => Rank::Knight,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            _ => Rank::None,
        }
    }

    /// Convert to char
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!('4', Rank::Four.to_char());
    /// ```
    pub const fn to_char(&self) -> char {
        if (*self as usize) > 15 { return '?'; }
        CHARS[*self as usize]
    }

    /// Produce "seven", "jack", etc.
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("seven", Rank::Seven.name());
    /// ```
    pub const fn name(&self) -> &str {
        if (*self as usize) > 15 { return "?"; }
        NAMES[*self as usize]
    }

    /// Because we have to deal with "sixes"
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("fives", Rank::Five.plural());
    /// assert_eq!("sixes", Rank::Six.plural());
    /// ```
    pub const fn plural(&self) -> &str {
        if (*self as usize) > 15 { return "?"; }
        PLURALS[*self as usize]
    }

    /// Because we have to deal with "eight" and "ace"
    /// ```rust
    /// use onejoker::prelude::*;
    ///
    /// assert_eq!("an", Rank::Eight.article());
    /// assert_eq!("a", Rank::Nine.article());
    /// ```
    pub const fn article(&self) -> &str {
        if (*self as usize) == 1 || (*self as usize) == 15 ||
            (*self as usize) == 8 { return "an"; }
        "a"
    }
}

impl std::convert::From<u32> for Rank {
    fn from(v: u32) -> Self {
        Rank::from_u8(v as u8)
    }
}

impl std::str::FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let r = s.chars().next().ok_or(
            Error::ParseEmpty(String::from("rank")))?;
        Ok(Rank::from_char(r))
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
    fn test_ranks() -> Result<()> {
        has_traits::<Rank>();
        assert_eq!(Rank::LowAce as i32, 1);

        assert_eq!("ace", Rank::LowAce.name());
        assert_eq!("aces", Rank::LowAce.plural());
        assert_eq!("an", Rank::LowAce.article());
        assert_eq!('A', Rank::LowAce.to_char());

        macro_rules! ranktests {
            ( $r:ident, $v:literal, $c:literal, $a:literal, $n:literal, $p:literal ) => {
                {
                    use std::str::FromStr;

                    assert_eq!($v, Rank::$r as i32);
                    assert_eq!($a, Rank::$r.article());
                    assert_eq!($n, Rank::$r.name());
                    assert_eq!($p, Rank::$r.plural());
                    assert_eq!($c, Rank::$r.to_char());
                    assert_eq!(Rank::$r, Rank::from_char($c));
                    assert_eq!(Rank::$r, Rank::from_str(&String::from($c)[..])?);
                }
            };
        }

        ranktests!(Deuce, 2, '2', "a", "deuce", "deuces");
        ranktests!(Trey, 3, '3', "a", "trey", "treys");
        ranktests!(Four, 4, '4', "a", "four", "fours");
        ranktests!(Five, 5, '5', "a", "five", "fives");
        ranktests!(Six, 6, '6', "a", "six", "sixes");
        ranktests!(Seven, 7, '7', "a", "seven", "sevens");
        ranktests!(Eight, 8, '8', "an", "eight", "eights");
        ranktests!(Nine, 9, '9', "a", "nine", "nines");
        ranktests!(Ten, 10, 'T', "a", "ten", "tens");
        ranktests!(Jack, 11, 'J', "a", "jack", "jacks");
        ranktests!(Knight, 12, 'C', "a", "knight", "knights");
        ranktests!(Queen, 13, 'Q', "a", "queen", "queens");
        ranktests!(King, 14, 'K', "a", "king", "kings");
        ranktests!(Ace, 15, 'A', "an", "ace", "aces");

        // PartialOrd
        assert!(Rank::Deuce < Rank::Trey);

        // Debug, Display
        let mut s = format!("{} {} {} {} {} {} {} {}",
            Rank::LowAce, Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight);
        assert_eq!("LowAce Deuce Trey Four Five Six Seven Eight", s);
        s = format!("{} {} {} {} {} {} {}",
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Knight, Rank::Queen,
            Rank::King, Rank::Ace);
        assert_eq!("Nine Ten Jack Knight Queen King Ace", s);
        s = format!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            Rank::LowAce, Rank::Deuce, Rank::Trey, Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight);
        assert_eq!("LowAce Deuce Trey Four Five Six Seven Eight", s);
        s = format!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Knight, Rank::Queen,
            Rank::King, Rank::Ace);
        assert_eq!("Nine Ten Jack Knight Queen King Ace", s);

        Ok(())
    }
}
