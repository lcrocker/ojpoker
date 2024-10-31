// Do not edit: File generated with build_master_deck_code.ts
//! [wiki](https://github.com/lcrocker/ojpoker/wiki/MasterDeck) | Represents a new, full deck

use crate::cards::*;

/// [wiki](https://github.com/lcrocker/tspoker/wiki/MasterDeck) | A new full deck of cards
///
/// A static object that describes the properties of a new deck of cards for a
/// certain game or set of games.
/// For example, the "English" master deck has 52 cards, no jokers, aces are high,
/// and no duplicate cards are allowed.
/// The "Canasta" deck has 108 cards including jokers and duplicates.
///
/// Since this is all unchanging information, `MasterDeck::new()`
/// just returns a pointer to an existing static object based on the name you
/// pass in.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MasterDeck {
    /// Canonical name of deck
    pub name: &'static str,
    /// Bitset of cards in deck for quick lookup
    pub card_set: u64,
    /// List of cards in full deck
    pub card_list: &'static [Card],
    /// Are duplicate cards allowed?
    pub dups_allowed: bool,
    /// Are aces low?
    pub low_aces: bool,
}

impl MasterDeck {
    /// Retrieve pointer to [MasterDeck] by name (or alias).
    pub fn by_name(dname: &str) -> &'static Self { masterdeck_by_name(dname) }

    /// Does this deck contain the given card?
    pub fn has(&self, c: Card) -> bool { 0 != (self.card_set & (1 << c.0)) }

    /// How many cards in full deck?
    pub fn size(&self) -> usize { self.card_list.len() }
}

impl core::fmt::Debug for MasterDeck {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("MasterDeck")
            .field("name", &format_args!("{}", self.name))
            .field("card_set", &format_args!("0x{:X}", self.card_set))
            .field("card_list", &(self.card_list.len()))
            .field("dups", &format_args!("{}", if self.dups_allowed { "Yes" } else { "No" }))
            .field("aces", &format_args!("{}", if self.low_aces { "Low" } else { "High" }))
            .finish()
    }
}

impl std::fmt::Display for MasterDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{} ({})", self.name.chars().next().unwrap().to_uppercase(),
            self.name.chars().skip(1).collect::<String>(),
        self.card_list.len())
    }
}

fn masterdeck_by_name(alias: &str) -> &'static MasterDeck {
    match &alias.to_lowercase()[..] {
        "english" | "french" | "poker" | "bridge" | "52" | "deucetoseven" | "tienlen" | "gin" | "spades" | "hearts" | "boure" => &DECK_INFO[0],
        "allcards" | "default" => &DECK_INFO[1],
        "onejoker" | "joker" | "53" | "bug" | "paigow" => &DECK_INFO[2],
        "twojokers" | "54" | "doudizhu" => &DECK_INFO[3],
        "low" | "low52" | "razz" | "badugi" | "acetofive" | "blackjack" | "cribbage" | "baccarat" => &DECK_INFO[4],
        "lowjoker" | "lowball" | "low53" | "lowbug" => &DECK_INFO[5],
        "spanish" | "spanish40" | "40" => &DECK_INFO[6],
        "spanish48" | "48" => &DECK_INFO[7],
        "mexican" | "41" => &DECK_INFO[8],
        "panguingue" | "pan" => &DECK_INFO[9],
        "german" | "skat" | "piquet" | "32" => &DECK_INFO[10],
        "swiss" | "jass" | "russian" | "durak" | "36" => &DECK_INFO[11],
        "euchre" | "24" => &DECK_INFO[12],
        "euchre25" | "25" => &DECK_INFO[13],
        "euchre28" | "28" => &DECK_INFO[14],
        "euchre29" | "29" => &DECK_INFO[15],
        "bezique" => &DECK_INFO[16],
        "canasta" => &DECK_INFO[17],
        "pinochle" => &DECK_INFO[18],
        _ => &DECK_INFO[0],
    }
}

/// Retrieve pointer to [MasterDeck] by index (1-based)
pub fn masterdeck_by_index(idx: usize) -> &'static MasterDeck {
    &DECK_INFO[idx - 1]
}
/// How many decks are there?
pub fn deck_count() -> usize { DECK_INFO.len() }

macro_rules! masterdeck {
    ( $name:literal, $set:literal, $list:expr,
        $d:literal, $la:literal ) => {
        MasterDeck {
            name: $name,
            card_set: $set,
            card_list: $list,
            dups_allowed: $d,
            low_aces: $la,
        }
    };
}

const DECK_INFO: [MasterDeck; 19] = [
    masterdeck!("english",
         0xfff0ffffffffff00,
         &ENGLISH_CARDS,
         false,
         false),
    masterdeck!("allcards",
         0xfff0ffffffffff0e,
         &ALLCARDS_CARDS,
         false,
         false),
    masterdeck!("onejoker",
         0xfff0ffffffffff08,
         &ONEJOKER_CARDS,
         false,
         false),
    masterdeck!("twojokers",
         0xfff0ffffffffff0c,
         &TWOJOKERS_CARDS,
         false,
         false),
    masterdeck!("low",
         0xff0fffffffffff0,
         &LOW_CARDS,
         false,
         true),
    masterdeck!("lowjoker",
         0xff0fffffffffff8,
         &LOWJOKER_CARDS,
         false,
         true),
    masterdeck!("spanish",
         0xf0ff000fffffff0,
         &SPANISH_CARDS,
         false,
         true),
    masterdeck!("spanish48",
         0xf0ff0fffffffff0,
         &SPANISH48_CARDS,
         false,
         true),
    masterdeck!("mexican",
         0xfff0f000ffffff08,
         &MEXICAN_CARDS,
         false,
         false),
    masterdeck!("panguingue",
         0xff0f000fffffff0,
         &PANGUINGUE_CARDS,
         true,
         true),
    masterdeck!("german",
         0xfff0fffff0000000,
         &GERMAN_CARDS,
         false,
         false),
    masterdeck!("swiss",
         0xfff0ffffff000000,
         &SWISS_CARDS,
         false,
         false),
    masterdeck!("euchre",
         0xfff0fff000000000,
         &EUCHRE_CARDS,
         false,
         false),
    masterdeck!("euchre25",
         0xfff0fff000000008,
         &EUCHRE25_CARDS,
         false,
         false),
    masterdeck!("euchre28",
         0xfff0ffff00000000,
         &EUCHRE28_CARDS,
         false,
         false),
    masterdeck!("euchre29",
         0xfff0ffff00000008,
         &EUCHRE29_CARDS,
         false,
         false),
    masterdeck!("bezique",
         0xfff0fffff0000000,
         &BEZIQUE_CARDS,
         true,
         false),
    masterdeck!("canasta",
         0xfff0ffffffffff0c,
         &CANASTA_CARDS,
         true,
         false),
    masterdeck!("pinochle",
         0xfff0fff000000000,
         &PINOCHLE_CARDS,
         true,
         false),
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

const ENGLISH_CARDS: [Card; 52] =
card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8);

const ALLCARDS_CARDS: [Card; 55] =
card_array!(63,62,61,60,59,58,57,56,55,54,53,52,47,46,45,44,43,42,41,40,39,38,37,36,35,34,33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,3,2,1);

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

