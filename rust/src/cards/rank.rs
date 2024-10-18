//! [wiki](https://github.com/lcrocker/ojpoker/wiki/Rank) | Simple numeric enum for card ranks.

use crate::errors::*;

/// [wiki](https://github.com/lcrocker/tspoker/wiki/Rank) | Enum for card ranks.
/// Simple integer enum. Specific numbers do matter: I do a lot of math with
/// them to optimize things, and the same numbers are used in the other
/// languages in the project.
/// 
/// Note that there are two slots for aces and a slot for knight/cavalier.
/// See [Ace](https://github.com/lcrocker/ojpoker/wiki/Ace) and
/// [Knight](https://github.com/lcrocker/ojpoker/wiki/Knight) @ wiki
/// for details.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
#[allow(missing_docs)]
pub enum Rank {
    None = 0,
    LowAce = 1,
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

const RANKS: [Rank; 15] = [ Rank::LowAce, Rank::Deuce,
    Rank::Trey, Rank::Four, Rank::Five, Rank::Six, Rank::Seven,
    Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack,  Rank::Knight,
    Rank::Queen, Rank::King, Rank::Ace,];
const CHARS: [char; 15] = [ 'A', '2', '3', '4', '5', '6', '7', '8',
    '9', 'T', 'J', 'C', 'Q', 'K', 'A' ];
const NAMES: [&str; 15] = [ "ace", "deuce", "trey", "four",
    "five", "six", "seven", "eight", "nine", "ten", "jack", "knight",
    "queen", "king", "ace" ];
const PLURALS: [&str; 15] = [ "aces", "deuces", "treys", "fours",
    "fives", "sixes", "sevens", "eights", "nines", "tens", "jacks", 
    "knights", "queens", "kings", "aces" ];

impl Rank {
    /// Convert integer to rank.
    pub const fn from_i32_const(v: i32) -> Option<Rank> {
        if v < 1 || v > 15 { return None; }
        Some(RANKS[v as usize - 1])
    }

    /// Convert integer to rank.
    pub fn from_i32(v: i32) -> aResult<Rank> {
        if v < 1 || v > 15 {
            return Err(anyhow!(OjError::NotRank(v.to_string())));
        }
        aOk(RANKS[v as usize - 1])
    }

    /// Convert character. High aces only.
    pub fn from_char(c: char) -> aResult<Rank> {
        aOk(match c {
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
            _ => bail!(OjError::NotRank(c.to_string())),
        })
    }

    /// Convert to char
    pub fn to_char(&self) -> char {
        debug_assert!(*self >= Rank::LowAce || *self <= Rank::Ace);
        CHARS[*self as usize - 1]
    }

    /// Produce "seven", "jack", etc.
    pub fn name(&self) -> &str {
        debug_assert!(*self >= Rank::LowAce || *self <= Rank::Ace);
        NAMES[*self as usize - 1]
    }

    /// Because we have to deal with "sixes".
    pub fn plural(&self) -> &str {
        debug_assert!(*self >= Rank::LowAce || *self <= Rank::Ace);
        PLURALS[*self as usize - 1]
    }

    /// Because we have to deal with "eight" and "ace".
    pub fn article(&self) -> &str {
        if *self == Rank::LowAce || *self == Rank::Ace ||
            *self == Rank::Eight { return "an"; }
        "a"
    }
}

impl std::str::FromStr for Rank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> aResult<Self, Self::Err> {
        let r = s.chars().next().ok_or(
            OjError::ParseEmpty(String::from("rank")))?;
        let rank = Rank::from_char(r).or(
            Err(OjError::NotRank(String::from(r))))?;
        aOk(rank)
    }
}

#[allow(clippy::from_over_into)]    // From would fail
impl std::convert::Into<i32> for Rank {
    fn into(self) -> i32 {
        self as i32
    }
}

#[allow(clippy::from_over_into)]    // From would fail
impl std::convert::Into<char> for Rank {
    fn into(self) -> char {
        self.to_char()
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

    #[test]
    fn test_ranks() -> aResult<()> {
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
                    assert_eq!(Rank::$r, Rank::from_char($c)?);
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

        aOk(())
    }
}
