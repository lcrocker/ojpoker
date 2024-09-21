//@ cards/masterdeck.rs
//@ Lee Daniel Crocker <lee@piclab.com>

//! # masterdeck | [wiki](https://github.com/lcrocker/tspoker/wiki/MasterDeck) | A fresh unused pack of cards for specific games

use crate::cards::*;

/// # MasterDeck | [wiki](https://github.com/lcrocker/tspoker/wiki/MasterDeck)
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
    id: u32,
    name: &'static str,
    card_set: u64,
    card_list: &'static [Card],
    dups_allowed: bool,
    low_aces: bool,
    q_is_knight: bool,
}

impl MasterDeck {
    pub fn new(dname: &str) -> &'static Self { masterdeck_by_name(dname) }

    /// Rust won't let us implement `Default` as a pointer, so we roll our own.
    pub fn default_as_ptr() -> &'static MasterDeck { &DECK_INFO[0] }

    /// Pointer to the internal array of cards representing a full deck.
    pub fn card_list_ptr(&self) -> &'static [Card] { self.card_list }
}

impl core::fmt::Debug for MasterDeck {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct("MasterDeck")
            .field("name", &format_args!("{}", self.name))
            .field("card_set", &format_args!("0x{:X}", self.card_set))
            .field("card_list", &(self.card_list.len()))
            .field("dups", &format_args!("{}", if self.dups_allowed { "Yes" } else { "No" }))
            .field("aces", &format_args!("{}", if self.low_aces { "Low" } else { "High" }))
            .field("queen", &format_args!("{}", if self.q_is_knight { "Is Knight" } else { "Is Queen" }))
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

macro_rules! masterdeck {
    ( $id:literal, $name:literal, $set:literal, $list:expr,
        $d:literal, $la:literal, $q:literal ) => {
        MasterDeck {
            id: $id,
            name: $name,
            card_set: $set,
            card_list: $list,
            dups_allowed: $d,
            low_aces: $la,
            q_is_knight: $q,
        }
    };
}

const DECK_INFO: [MasterDeck; 14] = [
    masterdeck!(0xa9d53d64, "english", 0xfffffffffffff00, &ENGLISH_CARDS,
        false, false, false),
    masterdeck!(0x36c47598, "onejoker", 0xfffffffffffff08, &ONEJOKER_CARDS,
        false, false, false),
    masterdeck!(0x791a3445, "twojokers", 0xfffffffffffff0c, &TWOJOKERS_CARDS,
        false, false, false),
    masterdeck!(0x3d80a9cd, "low", 0xfffffffffffff0, &LOW_CARDS,
        false, true, false),
    masterdeck!(0x9fe03751, "lowjoker", 0xfffffffffffff8, &LOWJOKER_CARDS,
        false, true, false),
    masterdeck!(0x37c5bfc5, "spanish", 0xfff000fffffff0, &SPANISH_CARDS,
        false, true, false),
    masterdeck!(0x83ea607b, "spanish48", 0xfff0fffffffff0, &SPANISH48_CARDS,
        false, true, false),
    masterdeck!(0x83bfc27c, "mexican", 0xffff000ffffff08, &MEXICAN_CARDS,
        false, false, false),
    masterdeck!(0x33fd617e, "panguingue", 0xfff000fffffff0, &PANGUINGUE_CARDS,
        true, true, false),
    masterdeck!(0x62782f8f, "german", 0xffffffff0000000, &GERMAN_CARDS,
        false, false, false),
    masterdeck!(0x0858e4b7, "russian", 0xfffffffff000000, &RUSSIAN_CARDS,
        false, false, false),
    masterdeck!(0x8ca62f40, "bezique", 0xffffffff0000000, &BEZIQUE_CARDS,
        true, false, false),
    masterdeck!(0x832a48d8, "canasta", 0xfffffffffffff0c, &CANASTA_CARDS,
        true, false, false),
    masterdeck!(0x8f0816ef, "pinochle", 0xffffff000000000, &PINOCHLE_CARDS,
        true, false, false),
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

const ENGLISH_CARDS: [Card; 52] = card_array!(8,9,10,11, 12,13,14,15,
    16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31, 32,33,34,35,
    36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51, 52,53,54,55,
    56,57,58,59);
const ONEJOKER_CARDS: [Card; 53] = card_array!(3, 8,9,10,11, 12,13,14,15,
    16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31, 32,33,34,35,
    36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51, 52,53,54,55,
    56,57,58,59);
const TWOJOKERS_CARDS: [Card; 54] = card_array!(2,3, 8,9,10,11,
    12,13,14,15, 16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31,
    32,33,34,35, 36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51,
    52,53,54,55, 56,57,58,59);
const LOW_CARDS: [Card; 52] = card_array!(4,5,6,7, 8,9,10,11, 12,13,14,15,
    16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31, 32,33,34,35,
    36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51, 52,53,54,55);
const LOWJOKER_CARDS: [Card; 53] = card_array!(3, 4,5,6,7, 8,9,10,11,
    12,13,14,15, 16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31,
    32,33,34,35, 36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51,
    52,53,54,55);
const SPANISH_CARDS: [Card; 40] = card_array!(4,5,6,7, 8,9,10,11,
    12,13,14,15, 16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31,
    44,45,46,47, 48,49,50,51, 52,53,54,55);
const SPANISH48_CARDS: [Card; 48] = card_array!(4,5,6,7, 8,9,10,11,
    12,13,14,15, 16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31,
    32,33,34,35, 36,37,38,39, 44,45,46,47, 48,49,50,51, 52,53,54,55);
const MEXICAN_CARDS: [Card; 41] = card_array!(3, 8,9,10,11, 12,13,14,15,
    16,17,18,19, 20,21,22,23, 24,25,26,27, 28,29,30,31, 44,45,46,47,
    48,49,50,51, 52,53,54,55, 56,57,58,59);
const PANGUINGUE_CARDS: [Card; 320] = card_array!(4,4,4,4, 4,4,4,4,
    5,5,5,5, 5,5,5,5, 6,6,6,6, 6,6,6,6, 7,7,7,7, 7,7,7,7, 8,8,8,8,
    8,8,8,8, 9,9,9,9, 9,9,9,9, 10,10,10,10, 10,10,10,10, 11,11,11,11,
    11,11,11,11, 12,12,12,12, 12,12,12,12, 13,13,13,13, 13,13,13,13,
    14,14,14,14, 14,14,14,14, 15,15,15,15, 15,15,15,15, 16,16,16,16,
    16,16,16,16, 17,17,17,17, 17,17,17,17, 18,18,18,18, 18,18,18,18,
    19,19,19,19, 19,19,19,19, 20,20,20,20, 20,20,20,20, 21,21,21,21,
    21,21,21,21, 22,22,22,22, 22,22,22,22, 23,23,23,23, 23,23,23,23,
    24,24,24,24, 24,24,24,24, 25,25,25,25, 25,25,25,25, 26,26,26,26,
    26,26,26,26, 27,27,27,27, 27,27,27,27, 28,28,28,28, 28,28,28,28,
    29,29,29,29, 29,29,29,29, 30,30,30,30, 30,30,30,30, 31,31,31,31,
    31,31,31,31, 44,44,44,44, 44,44,44,44, 45,45,45,45, 45,45,45,45,
    46,46,46,46, 46,46,46,46, 47,47,47,47, 47,47,47,47, 48,48,48,48,
    48,48,48,48, 49,49,49,49, 49,49,49,49, 50,50,50,50, 50,50,50,50,
    51,51,51,51, 51,51,51,51, 52,52,52,52, 52,52,52,52, 53,53,53,53,
    53,53,53,53, 54,54,54,54, 54,54,54,54, 55,55,55,55, 55,55,55,55);
const GERMAN_CARDS: [Card; 32] = card_array!(28,29,30,31, 32,33,34,35,
    36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51, 52,53,54,55,
    56,57,58,59);
const RUSSIAN_CARDS: [Card; 36] = card_array!(24,25,26,27, 28,29,30,31,
    32,33,34,35, 36,37,38,39, 40,41,42,43, 44,45,46,47, 48,49,50,51,
    52,53,54,55, 56,57,58,59);
const BEZIQUE_CARDS: [Card; 64] = card_array!(28,28,29,29, 30,30,31,31,
    32,32,33,33, 34,34,35,35, 36,36,37,37, 38,38,39,39, 40,40,41,41,
    42,42,43,43, 44,44,45,45, 46,46,47,47, 48,48,49,49, 50,50,51,51,
    52,52,53,53, 54,54,55,55, 56,56,57,57, 58,58,59,59);
const CANASTA_CARDS: [Card; 108] = card_array!(2,2,3,3, 8,8,9,9,
    10,10,11,11, 12,12,13,13, 14,14,15,15, 16,16,17,17, 18,18,19,19,
    20,20,21,21, 22,22,23,23, 24,24,25,25, 26,26,27,27, 28,28,29,29,
    30,30,31,31, 32,32,33,33, 34,34,35,35, 36,36,37,37, 38,38,39,39,
    40,40,41,41, 42,42,43,43, 44,44,45,45, 46,46,47,47, 48,48,49,49,
    50,50,51,51, 52,52,53,53, 54,54,55,55, 56,56,57,57, 58,58,59,59);
const PINOCHLE_CARDS: [Card; 48] = card_array!(36,36,37,37, 38,38,39,39,
    40,40,41,41, 42,42,43,43, 44,44,45,45, 46,46,47,47, 48,48,49,49,
    50,50,51,51, 52,52,53,53, 54,54,55,55, 56,56,57,57, 58,58,59,59);

fn masterdeck_by_name(alias: &str) -> &'static MasterDeck {
    match &alias.to_lowercase()[..] {
        "english" | "french" | "poker" | "52" |
        "deucetoseven" | "default" | ""             => &DECK_INFO[0],
        "onejoker" | "joker" | "53"                 => &DECK_INFO[1],
        "twojokers" | "54"                          => &DECK_INFO[2],
        "low" | "low52" | "blackjack" | "razz" |
        "badugi" | "acetofive"                      => &DECK_INFO[3],
        "lowjoker" | "lowball" | "low53"            => &DECK_INFO[4],
        "spanish" | "spanish40" | "40"              => &DECK_INFO[5],
        "spanish48" | "48"                          => &DECK_INFO[6],
        "mexican" | "41"                            => &DECK_INFO[7],
        "panguingue" | "pan"                        => &DECK_INFO[8],
        "german" | "skat" | "piquet" | "32"         => &DECK_INFO[9],
        "russian" | "durak" | "36"                  => &DECK_INFO[10],
        "bezique"                                   => &DECK_INFO[11],
        "canasta"                                   => &DECK_INFO[12],
        "pinochle"                                  => &DECK_INFO[13],
        _                                           => &DECK_INFO[0],
    }
}

/*
 * CODE ENDS HERE
 */

#[test]
fn test_masterdecks() {
    for di in DECK_INFO {
        let mut bits: u64 = 0;
        let mut ids = std::collections::HashSet::new();

        for c in di.card_list { bits |= 1 << c.0; }
        assert_eq!(bits, di.card_set);
        assert!(! ids.contains(&di.id));
        ids.insert(&di.id);

        macro_rules! decktests {
            ( $d:literal, $a:literal, $q:literal, $len:literal ) => {
                assert_eq!($d, di.dups_allowed);
                assert_eq!($a, di.low_aces);
                assert_eq!($q, di.q_is_knight);
                assert_eq!($len, di.card_list.len());
            };
        }

        match di.name {
            "english" => {
                decktests!(false, false, false, 52);
                assert!(!di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "onejoker" => {
                decktests!(false, false, false, 53);
                assert!(di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "twojokers" => {
                decktests!(false, false, false, 54);
                assert!(di.card_list.contains(&BLACK_JOKER));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "low" => {
                decktests!(false, true, false, 52);
                assert!(!di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&LOW_ACE_OF_CLUBS));
            },
            "lowjoker" => {
                decktests!(false, true, false, 53);
                assert!(di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&LOW_ACE_OF_CLUBS));
            },
            "spanish" => {
                decktests!(false, true, false, 40);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&EIGHT_OF_CLUBS));
                assert!(di.card_list.contains(&LOW_ACE_OF_CLUBS));
            },
            "spanish48" => {
                decktests!(false, true, false, 48);
                assert!(!di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&EIGHT_OF_CLUBS));
                assert!(!di.card_list.contains(&TEN_OF_CLUBS));
                assert!(di.card_list.contains(&LOW_ACE_OF_CLUBS));
            },
            "mexican" => {
                decktests!(false, false, false, 41);
                assert!(di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&EIGHT_OF_CLUBS));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "panguingue" => {
                decktests!(true, true, false, 320);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&EIGHT_OF_CLUBS));
                assert!(di.card_list.contains(&LOW_ACE_OF_CLUBS));
            },
            "german" => {
                decktests!(false, false, false, 32);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&SIX_OF_CLUBS));
                assert!(di.card_list.contains(&SEVEN_OF_CLUBS));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "russian" => {
                decktests!(false, false, false, 36);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&FIVE_OF_CLUBS));
                assert!(di.card_list.contains(&SIX_OF_CLUBS));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "bezique" => {
                decktests!(true, false, false, 64);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&SIX_OF_CLUBS));
                assert!(di.card_list.contains(&SEVEN_OF_CLUBS));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "canasta" => {
                decktests!(true, false, false, 108);
                assert!(di.card_list.contains(&JOKER));
                assert!(di.card_list.contains(&BLACK_JOKER));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            "pinochle" => {
                decktests!(true, false, false, 48);
                assert!(!di.card_list.contains(&JOKER));
                assert!(!di.card_list.contains(&EIGHT_OF_CLUBS));
                assert!(di.card_list.contains(&ACE_OF_CLUBS));
            },
            _ => panic!(),
        }
    }
}
