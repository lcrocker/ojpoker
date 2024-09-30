//@ cards/rank.rs

//! # rank | [wiki](https://github.com/lcrocker/ojpoker/wiki/Rank) | Simple numeric enum for card ranks.

use crate::cards::OjError;

/// # Rank | [wiki](https://github.com/lcrocker/tspoker/wiki/Rank)
/// Simple integer enum. Numbers do matter: I do a lot of math with them to optimize
/// things in various other places, and the same numbers are used in the other
/// languages in the OneJoker project.
/// 
/// A note about aces: In most poker games, aces are high, but in some they are low.
/// so to speed up some operations in those games, we allow the choice of values for
/// the ace, either below deuce or above king. The default is high, so some of the
/// operations here only produce high aces (the `FromStr` trait, for example), and
/// must be corrected afterward for low games.

#[allow(dead_code)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug, Hash)]
pub enum Rank {
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
    #[inline]
    pub fn from_i32(v: i32) -> Option<Rank> {
        if !(0..=15).contains(&v) { return None; }
        Some(RANKS[v as usize - 1])
    }

    /// Convert character. High aces only.
    pub fn from_char(c: char) -> Option<Rank> {
        match c {
            '2' => Some(Rank::Deuce),
            '3' => Some(Rank::Trey),
            '4' => Some(Rank::Four),
            '5' => Some(Rank::Five),
            '6' => Some(Rank::Six),
            '7' => Some(Rank::Seven),
            '8' => Some(Rank::Eight),
            '9' => Some(Rank::Nine),
            'T' => Some(Rank::Ten),
            'J' => Some(Rank::Jack),
            'C' => Some(Rank::Knight),
            'Q' => Some(Rank::Queen),
            'K' => Some(Rank::King),
            'A' => Some(Rank::Ace),
            _ => None,
        }
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
    type Err = OjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ro = s.chars().next();
        if ro.is_none() {
            return Err(OjError::ParseEmpty(String::from("rank")));
        }
        let r = ro.unwrap();
        let rank = Rank::from_char(r);
        if rank.is_none() {
            return Err(OjError::ParseNotRank(String::from(r)));
        }
        Ok(rank.unwrap())
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

#[test]
fn test_ranks() {
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
                assert_eq!(Rank::$r, Rank::from_char($c).unwrap());
                assert_eq!(Rank::$r, Rank::from_str(&String::from($c)[..]).unwrap())
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
}
