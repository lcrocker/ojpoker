//! [wiki](https://github.com/lcrocker/ojpoker/wiki/DeckType) | Represents a new, full deck

use std::sync::atomic::{ AtomicU8, Ordering };
use crate::cards::*;

static DEFAULT_DECK_TYPE: AtomicU8 = AtomicU8::new(1);

/// [wiki](https://github.com/lcrocker/ojpoker/wiki/DeckType) | Represents a new, full deck
///
/// Contains information about the kinds of decks used in various card games.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeckType {
    /// None / Invalid
    None = 0,
    /// 59-card deck with 3 jokers, knights, and high aces
    AllCards = 1,
    /// English/American 52-card deck with high aces
    English = 2,
    /// 53-card deck with one joker
    OneJoker = 3,
    /// 54-card deck with two jokers
    TwoJokers = 4,
    /// 52-card deck with low aces
    Low = 5,
    /// 53-card deck with low aces and joker
    LowJoker = 6,
    /// 40-card Spanish deck with low aces, knights, no 8/9/10
    Spanish = 7,
    /// 48-card Spanish deck with low aces, knights, no 10s
    Spanish48 = 8,
    /// 41-card Mexican deck with low aces, no 8/9/10, one joker
    Mexican = 9,
    /// 320-card Panguingue deck with low aces, knights, no 8/9/10, 8 decks
    Panguingue = 10,
    /// 32-card German deck with no 8/9/10
    Stripped = 11,
    /// 36-card Swiss/Russian deck with no 2..6
    Swiss = 12,
    /// 24-card Euchre deck with no 2..8
    Euchre = 13,
    /// 25-card Euchre deck with no 2..8, one joker
    Euchre25 = 14,
    /// 28-card Euchre deck with no 2..7
    Euchre28 = 15,
    /// 29-card Euchre deck with no 2..7, one joker
    Euchre29 = 16,
    /// 64-card Bezique deck with no 8/9/10, 2 decks
    Bezique = 17,
    /// 108-card Canasta deck with 2 jokers, 2 decks
    Canasta = 18,
    /// 48-card Pinochle deck with no 2..8, 2 decks
    Pinochle = 19,
}

const DECKTYPE_MAX: usize = DeckType::Pinochle as usize;

fn type_by_alias(alias: &str) -> DeckType {
    match &alias.to_lowercase()[..] {
        "allcards" => DeckType::AllCards,

        "english" | "french" | "poker" | "bridge" | "52" | "high"
            | "deucetoseven" | "tienlen" | "gin" | "spades" | "standard"
            | "hearts" | "boure" => DeckType::English,

        "onejoker" | "joker" | "53" | "bug" | "paigow" => DeckType::OneJoker,
        "twojokers" | "54" | "doudizhu" => DeckType::TwoJokers,

        "low" | "low52" | "razz" | "badugi" | "acetofive" | "blackjack"
            | "acetosix" | "cribbage" | "baccarat" => DeckType::Low,

        "lowjoker" | "lowball" | "low53" | "lowbug" => DeckType::LowJoker,
        "spanish" | "spanish40" | "40" => DeckType::Spanish,
        "spanish48" | "48" => DeckType::Spanish48,
        "mexican" | "41" => DeckType::Mexican,
        "panguingue" | "pan" => DeckType::Panguingue,

        "stripped" | "german" | "skat" | "piquet"
            | "manila" | "32" => DeckType::Stripped,

        "swiss" | "jass" | "russian" | "durak" | "36" => DeckType::Swiss,
        "euchre" | "24" => DeckType::Euchre,
        "euchre25" | "25" => DeckType::Euchre25,
        "euchre28" | "28" => DeckType::Euchre28,
        "euchre29" | "29" => DeckType::Euchre29,
        "bezique" => DeckType::Bezique,
        "canasta" => DeckType::Canasta,
        "pinochle" => DeckType::Pinochle,

        "default" => DeckType::from_u8(
            DEFAULT_DECK_TYPE.load(Ordering::Relaxed)
        ),
        _ => DeckType::English,
    }
}

impl DeckType {
    /// Get deck type by index. Primarily for internal use
    pub const fn from_u8(idx: u8) -> Self {
        match idx {
            1 => DeckType::AllCards,
            2 => DeckType::English,
            3 => DeckType::OneJoker,
            4 => DeckType::TwoJokers,
            5 => DeckType::Low,
            6 => DeckType::LowJoker,
            7 => DeckType::Spanish,
            8 => DeckType::Spanish48,
            9 => DeckType::Mexican,
            10 => DeckType::Panguingue,
            11 => DeckType::Stripped,
            12 => DeckType::Swiss,
            13 => DeckType::Euchre,
            14 => DeckType::Euchre25,
            15 => DeckType::Euchre28,
            16 => DeckType::Euchre29,
            17 => DeckType::Bezique,
            18 => DeckType::Canasta,
            19 => DeckType::Pinochle,
            _ => DeckType::None,
        }
    }

    /// Get deck type by name or alias
    /// ```rust
    /// use onejoker::*;
    ///
    /// let dt = DeckType::by_name("bridge");
    /// assert_eq!(dt, DeckType::English);
    /// ```
    pub fn by_name(dname: &str) -> Self {
        type_by_alias(dname)
    }

    /// Set default deck type
    /// ```rust
    /// use onejoker::*;
    ///
    /// DeckType::set_default(DeckType::Pinochle);
    /// let h = Hand::default();
    /// assert_eq!(h.deck_type(), DeckType::Pinochle);
    /// ```
    pub fn set_default(t: Self) {
        DEFAULT_DECK_TYPE.store(t as u8, Ordering::Relaxed);
    }

    /// Canonical name of deck type
    /// ```rust
    /// use onejoker::*;
    ///
    /// let d = Deck::new_by_name("lowball");
    /// assert_eq!(d.deck_type.name(), "lowjoker");
    /// ```
    pub const fn name(&self) -> &'static str {
        DECK_INFO_TABLE[*self as usize - 1].name
    }

    /// Number of cards in full deck
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert_eq!(DeckType::Bezique.size(), 64);
    /// ```
    pub const fn size(&self) -> usize {
        DECK_INFO_TABLE[*self as usize - 1].card_list.len()
    }

    /// Does the deck use low aces?
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new_by_name("razz");
    /// assert!(d.deck_type.low_aces());
    /// d = Deck::new_by_name("swiss");
    /// assert!(! d.deck_type.low_aces());
    /// ```
    pub const fn low_aces(&self) -> bool {
        DECK_INFO_TABLE[*self as usize - 1].low_aces
    }

    /// Does the deck allow duplicate cards?
    /// ```rust
    /// use onejoker::*;
    ///
    /// let mut d = Deck::new_by_name("german");
    /// assert!(! d.deck_type.dups_allowed());
    /// d = Deck::new_by_name("canasta");
    /// assert!(d.deck_type.dups_allowed());
    /// ```
    pub const fn dups_allowed(&self) -> bool {
        DECK_INFO_TABLE[*self as usize - 1].dups_allowed
    }

    /// Does the deck allow this specific card?
    /// ```rust
    /// use onejoker::*;
    ///
    /// assert!(DeckType::Spanish.has(KNIGHT_OF_CLUBS));
    /// assert!(! DeckType::Stripped.has(DEUCE_OF_CLUBS));
    /// ```
    pub const fn has(&self, c: Card) -> bool {
        0 != (DECK_INFO_TABLE[*self as usize - 1].card_set & (1 << c.0))
    }

    /// Get a slice of the full deck
    /// ```rust
    /// use onejoker::*;
    ///
    /// let v: Vec<Card> = DeckType::Spanish.card_list().to_vec();
    /// assert_eq!(v.len(), 40);
    /// ```
    pub const fn card_list(&self) -> &'static [Card] {
        DECK_INFO_TABLE[*self as usize - 1].card_list
    }

    /// Validate a card for this deck. Note that unlike `has`, which just
    /// gives a yes/no, this function will returns card with aces fixed if
    /// necessary, and maybe other fixes in the future.
    /// ```rust
    /// use onejoker::*;
    ///
    /// let dt = DeckType::by_name("lowball");
    /// assert_eq!(LOW_ACE_OF_CLUBS, dt.valid_card(ACE_OF_CLUBS).unwrap());
    /// assert_eq!(DEUCE_OF_CLUBS, dt.valid_card(DEUCE_OF_CLUBS).unwrap());
    /// assert_eq!(None, dt.valid_card(KNIGHT_OF_CLUBS));
    /// ```
    pub const fn valid_card(&self, cin: Card) -> Option<Card> {
        let g = &DECK_INFO_TABLE[*self as usize - 1];

        let cout = if g.low_aces {
            Card::low_ace_fix(cin)
        } else {
            Card::high_ace_fix(cin)
        };
        if 0 != (g.card_set & (1 << cout.0)) {
            Some(cout)
        } else {
            None
        }
    }
}

impl std::convert::From<u8> for DeckType {
    fn from(n: u8) -> Self {
        DeckType::from_u8(n)
    }
}

impl std::default::Default for DeckType {
    fn default() -> Self {
        DeckType::from_u8(
            DEFAULT_DECK_TYPE.load(Ordering::Relaxed)
        )
    }
}

// Static structure containing information about each deck type
#[derive(Debug)]
struct DeckInfo {
    /// Canonical name of deck
    name: &'static str,
    /// List of cards in full deck
    card_list: &'static [Card],
    /// Bitset of cards in deck for quick lookup
    card_set: u64,
    /// Are duplicate cards allowed?
    dups_allowed: bool,
    /// Are aces low?
    low_aces: bool,
}

macro_rules! deck_info {
    ( $name:literal, $set:literal, $list:expr, $d:literal, $la:literal ) => {
        DeckInfo {
            name: $name,
            card_list: $list,
            card_set: $set,
            dups_allowed: $d,
            low_aces: $la,
        }
    };
}

const DECK_INFO_TABLE: [DeckInfo; DECKTYPE_MAX] = [
    deck_info!("allcards",  0xFFFF_FFFF_FFFF_FF0E,&ALLCARDS_CARDS,false,false),
    deck_info!("english",   0xFFF0_FFFF_FFFF_FF00,&ENGLISH_CARDS,false,false),
    deck_info!("onejoker",  0xFFF0_FFFF_FFFF_FF08,&ONEJOKER_CARDS,false,false),
    deck_info!("twojokers", 0xFFF0_FFFF_FFFF_FF0C,&TWOJOKERS_CARDS,false,false),
    deck_info!("low",       0x0FF0_FFFF_FFFF_FFF0,&LOW_CARDS,false,true),
    deck_info!("lowjoker",  0x0FF0_FFFF_FFFF_FFF8,&LOWJOKER_CARDS,false,true),
    deck_info!("spanish",   0x0F0F_F000_FFFF_FFF0,&SPANISH_CARDS,false,true),
    deck_info!("spanish48", 0x0F0F_F0FF_FFFF_FFF0,&SPANISH48_CARDS,false,true),
    deck_info!("mexican",   0xFFF0_F000_FFFF_FF08,&MEXICAN_CARDS,false,false),
    deck_info!("panguingue",0x0FF0_F000_FFFF_FFF0,&PANGUINGUE_CARDS,true,true),
    deck_info!("stripped",  0xFFF0_FFFF_F000_0000,&GERMAN_CARDS,false,false),
    deck_info!("swiss",     0xFFF0_FFFF_FF00_0000,&SWISS_CARDS,false,false),
    deck_info!("euchre",    0xFFF0_FFF0_0000_0000,&EUCHRE_CARDS,false,false),
    deck_info!("euchre25",  0xFFF0_FFF0_0000_0008,&EUCHRE25_CARDS,false,false),
    deck_info!("euchre28",  0xFFF0_FFFF_0000_0000,&EUCHRE28_CARDS,false,false),
    deck_info!("euchre29",  0xFFF0_FFFF_0000_0008,&EUCHRE29_CARDS,false,false),
    deck_info!("bezique",   0xFFF0_FFFF_F000_0000,&BEZIQUE_CARDS,true,false),
    deck_info!("canasta",   0xFFF0_FFFF_FFFF_FF0C,&CANASTA_CARDS,true,false),
    deck_info!("pinochle",  0xFFF0_FFF0_0000_0000,&PINOCHLE_CARDS,true,false),
];

macro_rules! card_array {
    ( $( $x:expr ),* ) => {
        [
            $(
                Card($x)
            ),*
        ]
    };
}

const ALLCARDS_CARDS: [Card; 59] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,3,2,1);
const ENGLISH_CARDS: [Card; 52] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8);
const ONEJOKER_CARDS: [Card; 53] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,3);
const TWOJOKERS_CARDS: [Card; 54] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,3,2);
const LOW_CARDS: [Card; 52] =
    card_array!(59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4);
const LOWJOKER_CARDS: [Card; 53] =
    card_array!(59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3);
const SPANISH_CARDS: [Card; 40] =
    card_array!(59,58,57,56,51,50,49,48,47,46,45,44,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4);
const SPANISH48_CARDS: [Card; 48] =
    card_array!(59,58,57,56,51,50,49,48,47,46,45,44,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4);
const MEXICAN_CARDS: [Card; 41] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,3);
const PANGUINGUE_CARDS: [Card; 320] =
    card_array!(59,59,59,59,59,59,59,59,58,58,58,58,58,58,58,58,57,57,57,57,57,57,57,57,56,56,56,56,56,56,56,56,55,55,55,55,55,55,55,55,54,54,54,54,54,54,54,54,53,53,53,53,53,53,53,53,52,52,52,52,52,52,52,52,47,47,47,47,47,47,47,47,46,46,46,46,46,46,46,46,45,45,45,45,45,45,45,45,44,44,44,44,44,44,44,44,31,31,31,31,31,31,31,31,30,30,30,30,30,30,30,30,29,29,29,29,29,29,29,29,28,28,28,28,28,28,28,28,27,27,27,27,27,27,27,27,26,26,26,26,26,26,26,26,25,25,25,25,25,25,25,25,24,24,24,24,24,24,24,24,23,23,23,23,23,23,23,23,22,22,22,22,22,22,22,22,21,21,21,21,21,21,21,21,20,20,20,20,20,20,20,20,19,19,19,19,19,19,19,19,18,18,18,18,18,18,18,18,17,17,17,17,17,17,17,17,16,16,16,16,16,16,16,16,15,15,15,15,15,15,15,15,14,14,14,14,14,14,14,14,13,13,13,13,13,13,13,13,12,12,12,12,12,12,12,12,11,11,11,11,11,11,11,11,10,10,10,10,10,10,10,10,9,9,9,9,9,9,9,9,8,8,8,8,8,8,8,8,7,7,7,7,7,7,7,7,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,4,4,4,4,4,4,4,4);
const GERMAN_CARDS: [Card; 32] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28);
const SWISS_CARDS: [Card; 36] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24);
const EUCHRE_CARDS: [Card; 24] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36);
const EUCHRE25_CARDS: [Card; 25] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,3);
const EUCHRE28_CARDS: [Card; 28] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32);
const EUCHRE29_CARDS: [Card; 29] =
    card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,3);
const BEZIQUE_CARDS: [Card; 64] =
    card_array!(63,63,62,62,61,61,60,60,59,59,58,58,57,57,56,56,55,55,54,54,53,53,52,52,47,47,46,46,45,45,44,44,43,43,42,42,41,41,40,40,39,39,38,38,37,37,36,36,35,35,34,34,33,33,32,32,31,31,30,30,29,29,28,28);
const CANASTA_CARDS: [Card; 108] =
    card_array!(63,63,62,62,61,61,60,60,59,59,58,58,57,57,56,56,55,55,54,54,53,53,52,52,47,47,46,46,45,45,44,44,43,43,42,42,41,41,40,40,39,39,38,38,37,37,36,36,35,35,34,34,33,33,32,32,31,31,30,30,29,29,28,28,27,27,26,26,25,25,24,24,23,23,22,22,21,21,20,20,19,19,18,18,17,17,16,16,15,15,14,14,13,13,12,12,11,11,10,10,9,9,8,8,3,3,2,2);
const PINOCHLE_CARDS: [Card; 48] =
    card_array!(63,63,62,62,61,61,60,60,59,59,58,58,57,57,56,56,55,55,54,54,53,53,52,52,47,47,46,46,45,45,44,44,43,43,42,42,41,41,40,40,39,39,38,38,37,37,36,36);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::OjError;
    use crate::utils::oj_rand_range;

    #[test]
    fn deck_type_test() -> Result<(), OjError> {
        assert_eq!(DeckType::default(), DeckType::AllCards);
        assert_eq!(DeckType::by_name("default"), DeckType::AllCards);
        assert_eq!(DeckType::by_name("poker"), DeckType::English);
        assert_eq!(DeckType::from_u8(2), DeckType::by_name("english"));

        assert_eq!(DeckType::by_name("bridge"), DeckType::English);
        assert_eq!(DeckType::by_name("52"), DeckType::English);
        assert_eq!(DeckType::by_name("bug"), DeckType::OneJoker);
        assert_eq!(DeckType::from_u8(3),DeckType::by_name("onejoker"));

        assert_eq!(DeckType::by_name("54"), DeckType::TwoJokers);
        assert_eq!(DeckType::by_name("razz"), DeckType::Low);
        assert_eq!(DeckType::by_name("blackjack"), DeckType::Low);
        assert_eq!(DeckType::by_name("baccarat"), DeckType::Low);

        DeckType::set_default(DeckType::English);
        assert_eq!(DeckType::default(), DeckType::English);
        assert_eq!(DeckType::by_name("default"), DeckType::English);
        DeckType::set_default(DeckType::by_name("allcards"));
        assert_eq!(DeckType::default(), DeckType::AllCards);

        assert_eq!(DeckType::by_name("lowball"), DeckType::LowJoker);
        assert_eq!(DeckType::by_name("40"), DeckType::Spanish);
        assert_eq!(DeckType::by_name("german"), DeckType::Stripped);
        assert_eq!(DeckType::by_name("durak"), DeckType::Swiss);

        for i in 1..=DECKTYPE_MAX {
            let dt = DeckType::from_u8(i as u8);

            assert_eq!(dt.size(), dt.card_list().len());
            let mut mask: u64 = 0;

            for j in 0..dt.size() {
                let card = dt.card_list()[j];
                mask |= 1 << card.0;
            }
            assert_eq!(mask, DECK_INFO_TABLE[dt as usize - 1].card_set);

            for _ in 0..10 {
                let card = Card(1 + oj_rand_range(63) as u8);
                let vc = dt.valid_card(card);

                if let Some(c) = vc {
                    if dt.low_aces() {
                        assert_eq!(c, Card::low_ace_fix(card));
                    } else {
                        assert_eq!(c, Card::high_ace_fix(card));
                    }
                    assert!(dt.has(c));
                    assert!(0 != (mask & (1 << c.0)));
                } else {
                    assert!(!dt.has(card));
                    assert!(0 == (mask & (1 << card.0)));
                }
            }
            match dt.name() {
                "allcards" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 59);
                    assert!(dt.has(card!("Jk")));
                    assert!(dt.has(card!("Jb")));
                    assert!(dt.has(card!("4c")));
                    assert!(dt.has(card!("9c")));
                    assert!(dt.has(card!("Cc")));
                    assert!(dt.has(card!("Ac")));
                    assert!(! dt.has(card!("1c")));
                },
                "english" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 52);
                    assert!(! dt.has(card!("Jk")));
                    assert!(dt.has(card!("7c")));
                    assert!(! dt.has(card!("Cc")));
                    assert!(dt.has(card!("Ac")));
                },
                "twojokers" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 54);
                    assert!(dt.has(card!("Jb")));
                    assert!(dt.has(card!("Tc")));
                    assert!(! dt.has(card!("Cc")));
                    assert!(dt.has(card!("Ac")));
                    assert!(! dt.has(card!("1c")));
                },
                "low" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), true);
                    assert_eq!(dt.size(), 52);
                    assert!(! dt.has(JOKER));
                    assert!(dt.has(EIGHT_OF_CLUBS));
                    assert!(! dt.has(ACE_OF_CLUBS));
                    assert!(! dt.has(KNIGHT_OF_CLUBS));
                    assert!(dt.has(LOW_ACE_OF_CLUBS));
                },
                "spanish" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), true);
                    assert_eq!(dt.size(), 40);
                    assert!(! dt.has(JOKER));
                    assert!(! dt.has(EIGHT_OF_CLUBS));
                    assert!(dt.has(LOW_ACE_OF_CLUBS));
                    assert!(dt.has(KNIGHT_OF_CLUBS));
                    assert!(! dt.has(QUEEN_OF_CLUBS));
                },
                "stripped" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 32);
                    assert!(! dt.has(JOKER));
                    assert!(! dt.has(DEUCE_OF_CLUBS));
                    assert!(! dt.has(SIX_OF_CLUBS));
                    assert!(dt.has(JACK_OF_CLUBS));
                    assert!(dt.has(ACE_OF_CLUBS));
                },
                "euchre" => {
                    assert_eq!(dt.dups_allowed(), false);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 24);
                    assert!(! dt.has(FIVE_OF_CLUBS));
                    assert!(dt.has(TEN_OF_CLUBS));
                    assert!(dt.has(ACE_OF_CLUBS));
                },
                "bezique" => {
                    assert_eq!(dt.dups_allowed(), true);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 64);
                    assert!(! dt.has(JOKER));
                    assert!(! dt.has(SIX_OF_CLUBS));
                    assert!(dt.has(SEVEN_OF_CLUBS));
                    assert!(dt.has(ACE_OF_CLUBS));
                },
                "pinochle" => {
                    assert_eq!(dt.dups_allowed(), true);
                    assert_eq!(dt.low_aces(), false);
                    assert_eq!(dt.size(), 48);
                    assert!(! dt.has(JOKER));
                    assert!(! dt.has(EIGHT_OF_CLUBS));
                    assert!(dt.has(ACE_OF_CLUBS));
                },
                _ => (),
            }
        }
        Ok(())
    }
}
