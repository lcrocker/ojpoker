// Do not edit: File generated with build_master_deck_code.ts
//@ cards/master_deck.rs
//@ Lee Daniel Crocker <lee@onejoker.org>

//! # masterdeck | [wiki](https://github.com/lcrocker/ojpoker/wiki/MasterDeck)

use crate::cards::*;

// # MasterDeck | [wiki](https://github.com/lcrocker/tspoker/wiki/MasterDeck)
/// A static object that describes the properties of a new deck of cards for a
/// certain game. For example, the "English" base deck has 52 cards, no jokers,
/// aces are high, and no duplicate cards are allowed. The "Canasta" deck has
/// 108 cards including jokers and duplicates. \
/// Since this is all unchanging information, `MasterDeck::new()`
/// just returns a pointer to an existing static object based on the name you
/// pass in. The Default trait doesn't allow this, so we provide
/// `default_as_ptr()`.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MasterDeck {
    pub name: &'static str,
    pub card_set: u64,
    pub card_list: &'static [Card],
    pub dups_allowed: bool,
    pub low_aces: bool,
}

impl MasterDeck {
    pub fn by_name(dname: &str) -> &'static Self { masterdeck_by_name(dname) }

    /// Rust won't let us implement `Default` as a pointer, so we roll our own.
    pub fn default_as_ptr() -> &'static MasterDeck { &DECK_INFO[0] }

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
        "english" | "default" | "french" | "poker" | "bridge" | "52" | "deucetoseven" | "tienlen" | "gin" | "spades" | "hearts" | "bouré" => &DECK_INFO[0],
        "onejoker" | "joker" | "53" | "bug" | "paigow" => &DECK_INFO[1],
        "twojokers" | "54" | "doudizhu" => &DECK_INFO[2],
        "low" | "low52" | "razz" | "badugi" | "acetofive" | "blackjack" | "cribbage" | "baccarat" => &DECK_INFO[3],
        "lowjoker" | "lowball" | "low53" | "lowbug" => &DECK_INFO[4],
        "spanish" | "spanish40" | "40" => &DECK_INFO[5],
        "spanish48" | "48" => &DECK_INFO[6],
        "mexican" | "41" => &DECK_INFO[7],
        "panguingue" | "pan" => &DECK_INFO[8],
        "german" | "skat" | "piquet" | "32" => &DECK_INFO[9],
        "swiss" | "jass" | "russian" | "durak" | "36" => &DECK_INFO[10],
        "euchre" | "24" => &DECK_INFO[11],
        "euchre25" | "25" => &DECK_INFO[12],
        "euchre28" | "28" => &DECK_INFO[13],
        "euchre29" | "29" => &DECK_INFO[14],
        "bezique" => &DECK_INFO[15],
        "canasta" => &DECK_INFO[16],
        "pinochle" => &DECK_INFO[17],
        _ => &DECK_INFO[0],
    }
}

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

const DECK_INFO: [MasterDeck; 18] = [
    masterdeck!("english",
         0xfff0ffffffffff00,
         &ENGLISH_CARDS,
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
card_array!(8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const ONEJOKER_CARDS: [Card; 53] =
card_array!(3,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const TWOJOKERS_CARDS: [Card; 54] =
card_array!(2,3,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const LOW_CARDS: [Card; 52] =
card_array!(4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59);

const LOWJOKER_CARDS: [Card; 53] =
card_array!(3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59);

const SPANISH_CARDS: [Card; 40] =
card_array!(4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,44,45,46,47,48,49,50,51,56,57,58,59);

const SPANISH48_CARDS: [Card; 48] =
card_array!(4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,44,45,46,47,48,49,50,51,56,57,58,59);

const MEXICAN_CARDS: [Card; 41] =
card_array!(3,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const PANGUINGUE_CARDS: [Card; 320] =
card_array!(4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,7,8,8,8,8,8,8,8,8,9,9,9,9,9,9,9,9,10,10,10,10,10,10,10,10,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,13,13,13,13,13,13,13,13,14,14,14,14,14,14,14,14,15,15,15,15,15,15,15,15,16,16,16,16,16,16,16,16,17,17,17,17,17,17,17,17,18,18,18,18,18,18,18,18,19,19,19,19,19,19,19,19,20,20,20,20,20,20,20,20,21,21,21,21,21,21,21,21,22,22,22,22,22,22,22,22,23,23,23,23,23,23,23,23,24,24,24,24,24,24,24,24,25,25,25,25,25,25,25,25,26,26,26,26,26,26,26,26,27,27,27,27,27,27,27,27,28,28,28,28,28,28,28,28,29,29,29,29,29,29,29,29,30,30,30,30,30,30,30,30,31,31,31,31,31,31,31,31,44,44,44,44,44,44,44,44,45,45,45,45,45,45,45,45,46,46,46,46,46,46,46,46,47,47,47,47,47,47,47,47,52,52,52,52,52,52,52,52,53,53,53,53,53,53,53,53,54,54,54,54,54,54,54,54,55,55,55,55,55,55,55,55,56,56,56,56,56,56,56,56,57,57,57,57,57,57,57,57,58,58,58,58,58,58,58,58,59,59,59,59,59,59,59,59);

const GERMAN_CARDS: [Card; 32] =
card_array!(28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const SWISS_CARDS: [Card; 36] =
card_array!(24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const EUCHRE_CARDS: [Card; 24] =
card_array!(36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const EUCHRE25_CARDS: [Card; 25] =
card_array!(3,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const EUCHRE28_CARDS: [Card; 28] =
card_array!(32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const EUCHRE29_CARDS: [Card; 29] =
card_array!(3,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,52,53,54,55,56,57,58,59,60,61,62,63);

const BEZIQUE_CARDS: [Card; 64] =
card_array!(28,28,29,29,30,30,31,31,32,32,33,33,34,34,35,35,36,36,37,37,38,38,39,39,40,40,41,41,42,42,43,43,44,44,45,45,46,46,47,47,52,52,53,53,54,54,55,55,56,56,57,57,58,58,59,59,60,60,61,61,62,62,63,63);

const CANASTA_CARDS: [Card; 108] =
card_array!(2,2,3,3,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,15,16,16,17,17,18,18,19,19,20,20,21,21,22,22,23,23,24,24,25,25,26,26,27,27,28,28,29,29,30,30,31,31,32,32,33,33,34,34,35,35,36,36,37,37,38,38,39,39,40,40,41,41,42,42,43,43,44,44,45,45,46,46,47,47,52,52,53,53,54,54,55,55,56,56,57,57,58,58,59,59,60,60,61,61,62,62,63,63);

const PINOCHLE_CARDS: [Card; 48] =
card_array!(36,36,37,37,38,38,39,39,40,40,41,41,42,42,43,43,44,44,45,45,46,46,47,47,52,52,53,53,54,54,55,55,56,56,57,57,58,58,59,59,60,60,61,61,62,62,63,63);

