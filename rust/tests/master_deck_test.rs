//@ master_deck_test.rs
//@ Lee Daniel Crocker <lee@piclab.com>

use onejoker::cards::*;

#[test]
fn test_masterdecks() {
    for name in ["poker", "bug", "54", "razz", "lowball", "40", "48",
        "pan", "skat", "durak", "24", "bezique", "canasta", "pinochle"].iter() {

        let deck = MasterDeck::by_name(*name);

        let mut bits: u64 = 0;
        for c in deck.card_list { bits |= 1 << c.0; }
        assert_eq!(bits, deck.card_set);

        macro_rules! decktests {
            ( $d:literal, $a:literal, $q:literal, $len:literal ) => {
                assert_eq!($d, deck.dups_allowed);
                assert_eq!($a, deck.low_aces);
                assert_eq!($q, deck.q_is_knight);
                assert_eq!($len, deck.card_list.len());
            };
        }

        match deck.name {
            "english" => {
                decktests!(false, false, false, 52);
                assert!(!deck.has(JOKER));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "onejoker" => {
                decktests!(false, false, false, 53);
                assert!(deck.has(JOKER));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "twojokers" => {
                decktests!(false, false, false, 54);
                assert!(deck.has(BLACK_JOKER));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "low" => {
                decktests!(false, true, false, 52);
                assert!(!deck.has(JOKER));
                assert!(deck.has(LOW_ACE_OF_CLUBS));
            },
            "lowjoker" => {
                decktests!(false, true, false, 53);
                assert!(deck.has(JOKER));
                assert!(deck.has(LOW_ACE_OF_CLUBS));
            },
            "spanish" => {
                decktests!(false, true, false, 40);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(LOW_ACE_OF_CLUBS));
            },
            "spanish48" => {
                decktests!(false, true, false, 48);
                assert!(!deck.has(JOKER));
                assert!(deck.has(EIGHT_OF_CLUBS));
                assert!(!deck.has(TEN_OF_CLUBS));
                assert!(deck.has(LOW_ACE_OF_CLUBS));
            },
            "mexican" => {
                decktests!(false, false, false, 41);
                assert!(deck.has(JOKER));
                assert!(!deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "panguingue" => {
                decktests!(true, true, false, 320);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(LOW_ACE_OF_CLUBS));
            },
            "german" => {
                decktests!(false, false, false, 32);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(SEVEN_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "russian" => {
                decktests!(false, false, false, 36);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(FIVE_OF_CLUBS));
                assert!(deck.has(SIX_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "euchre" => {
                decktests!(false, false, false, 24);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(TEN_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "euchre25" => {
                decktests!(false, false, false, 25);
                assert!(deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(TEN_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "euchre28" => {
                decktests!(false, false, false, 28);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "euchre29" => {
                decktests!(false, false, false, 28);
                assert!(deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "bezique" => {
                decktests!(true, false, false, 64);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(SIX_OF_CLUBS));
                assert!(deck.has(SEVEN_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "canasta" => {
                decktests!(true, false, false, 108);
                assert!(deck.has(JOKER));
                assert!(deck.has(BLACK_JOKER));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            "pinochle" => {
                decktests!(true, false, false, 48);
                assert!(!deck.has(JOKER));
                assert!(!deck.has(EIGHT_OF_CLUBS));
                assert!(deck.has(ACE_OF_CLUBS));
            },
            _ => panic!(),
        }
    }
}
